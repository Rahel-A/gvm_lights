use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use itertools::Itertools;
use log::info;
use futures::stream::StreamExt;

use btleplug::api::{Characteristic, Central, Manager as _, Peripheral,
    ScanFilter, WriteType, CharPropFlags};
use btleplug::platform::{Adapter, Manager, Peripheral as UnknownPeripherals};

use crate::{ControlMessage, LightCmd};

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

        // subscribe to notifications
        if cmd_char.properties.contains(CharPropFlags::NOTIFY) {
            info!("Subscribing to notifications");

            light.subscribe(&cmd_char).await?;

        }

        Ok(GvmBleClient {
                adapter: light,
                characteric: cmd_char.clone()
            })
    }

    // send a request to read initial values:
    pub async fn get_state(&self) -> Result<Vec<ControlMessage>, Box<dyn std::error::Error>> {
        let light = &self.adapter;
        let msg :&[u8] = b"4C5409000053000001009474FF";
        let test = GvmBleClient::format_msg(msg);

        light.write(
            &self.characteric, &test,
            WriteType::WithoutResponse).await?;
        let mut notification_stream =
            light.notifications().await?.take(2);

        let mut states:Vec<ControlMessage> = Vec::new();
        while let Some(data) = notification_stream.next().await {
            info!(
                "Received data from {:?} [{:?}]: {:X?}",
                light.address(), data.uuid, data.value
            );
            let mut _states = match data.value[2] {
                0xE =>
                    vec![ControlMessage::Light(
                            if data.value[6] == 0x01 { LightCmd::On }
                            else { LightCmd::Off }),
                         ControlMessage::RGB(data.value[7]),
                         ControlMessage::Brightness(data.value[8]),
                         ControlMessage::Temperature(data.value[9] as u16 * 100),
                         ControlMessage::Hue(data.value[10] as u16 * 5),
                         ControlMessage::Saturation(data.value[11]),
                    ],
                0xA => vec![],

                _ => panic!("unexpected value in notification"),
            };
            if _states.len() != 0 {
                states.append(&mut _states)
            }
        }
        info!("found these: {:?}", states);
        Ok(states)
    }

    fn format_msg(cmd: &[u8]) -> Vec<u8> {
        let mut msg_formatted:Vec<u8> = Vec::new();

        for (upper,lower) in cmd.into_iter().tuples() {
            let ascii_to_hex = |i:&u8|if *i < 59 {*i - 48} else { *i - 55 };
            let up = ascii_to_hex(upper);
            let lo = ascii_to_hex(lower);
            msg_formatted.push((up << 4) | lo);
        }
        msg_formatted
    }

    pub async fn send_to(&self, cmd: &[u8])
        -> Result<(), Box<dyn std::error::Error>> {

        let msg_formatted = GvmBleClient::format_msg(cmd);
        info!("Writing command to GVM Light:  {:X?}", msg_formatted);

        let light = &self.adapter;
        light.write(
            &self.characteric,
            &msg_formatted,
            WriteType::WithoutResponse).await?;

        info!("Checking notifications");
        let mut notification_stream =
            light.notifications().await?.take(1);
        // Process while the BLE connection is not broken or stopped.
        while let Some(data) = notification_stream.next().await {
            info!(
                "Received data from {:?} [{:?}]: {:X?}",
                light.address(), data.uuid, data.value
            );
        }

        Ok(())
    }
}
