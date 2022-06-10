use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use itertools::Itertools;
use log::info;

use btleplug::api::{Characteristic, Central, Manager as _, Peripheral, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral as UnknownPeripherals};

use crate::codec::{ControlMessage, encode};

pub struct GvmBleClient {
    adapter: UnknownPeripherals,
    characteric: Characteristic
}

const LIGHT_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00010203_0405_0607_0809_0a0b0c0d2b10);

async fn find_gvm_light(central: &Adapter, address: &str) -> Option<UnknownPeripherals> {
    for p in central.peripherals().await.unwrap() {
        if address == p.properties()
            .await
            .unwrap()
            .unwrap()
            .address.to_string()
        {
            return Some(p);
        }
    }
    None
}

impl GvmBleClient {
    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.adapter.disconnect().await?)
    }
    pub async fn new(local_addr: &str)
        -> Result<GvmBleClient, Box<dyn std::error::Error>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        let adapter = adapter_list
            .into_iter()
            .nth(0)
            .expect("No Bluetooth adapters found");

        info!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");

        time::sleep(Duration::from_secs(3)).await;

        let light = find_gvm_light(&adapter, local_addr)
            .await
            .expect("GVM light not found");

        // connect to the device
        light.connect().await?;

        // discover services and characteristics
        light.discover_services().await?;

        // find the characteristic we want
        let chars = light.characteristics();
        let cmd_char = chars
            .iter()
            .find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
            .expect("Unable to find characterics");

        Ok(GvmBleClient {
                adapter: light,
                characteric: cmd_char.clone()
            })
    }

    pub async fn send_to(&self, cmd: &ControlMessage)
        -> Result<&GvmBleClient, Box<dyn std::error::Error>> {
        let mut msg_formatted:Vec<u8> = Vec::new();

        for (upper,lower) in encode(cmd)?.into_iter().tuples() {
            let ascii_to_hex = |i|if i < 59 {i - 48} else { i - 55 };
            let up = ascii_to_hex(upper);
            let lo = ascii_to_hex(lower);
            msg_formatted.push((up << 4) | lo);
        }
        info!("Writing command {:#X?} to GVM Light", msg_formatted);

        self.adapter.write(
            &self.characteric,
            &msg_formatted,
            WriteType::WithoutResponse).await?;
        Ok(self)
    }
}
