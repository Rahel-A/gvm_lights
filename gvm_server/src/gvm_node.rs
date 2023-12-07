use crc_any::CRC;
use futures::stream::StreamExt;
use itertools::Itertools;
use log::{info, trace};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

use btleplug::api::{
    Central, CharPropFlags, Characteristic, Manager as _, Peripheral, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral as UnknownPeripherals};

use crate::gvm_node_status::units::*;
use crate::{GvmNodeCommand, GvmNodeStatus, GvmServerError, NodeCommandEncoder};

#[derive(Clone)]
pub struct GvmNode800D {
    id: usize,
    adapter: Arc<UnknownPeripherals>,
    characteric: Arc<Characteristic>,
    last_received_state: GvmNodeStatus,
}

const LIGHT_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00010203_0405_0607_0809_0a0b0c0d2b10);
const LIGHT_NAME: &'static str = "BT_LED";

impl GvmNode800D {
    async fn find_all_gvm_lights(
        adapter: &Adapter,
    ) -> Result<Vec<UnknownPeripherals>, Box<dyn std::error::Error>> {
        let mut gvm_lights: Vec<UnknownPeripherals> = vec![];
        for p in adapter.peripherals().await.unwrap() {
            if LIGHT_NAME
                == p.properties()
                    .await
                    .unwrap()
                    .unwrap()
                    .local_name
                    .unwrap_or("".to_string())
                    .to_string()
            {
                trace!("Found GVM Light node!: {:?} {:?}", p, p.id());
                gvm_lights.push(p);
            }
        }
        Ok(gvm_lights)
    }

    async fn find_gvm_light(
        adapter: &Adapter,
        address: &str,
    ) -> Result<Option<UnknownPeripherals>, Box<dyn std::error::Error>> {
        for p in adapter.peripherals().await.unwrap() {
            if address == p.properties().await.unwrap().unwrap().address.to_string() {
                return Ok(Some(p));
            }
        }
        Ok(None)
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.adapter.disconnect().await?)
    }

    pub async fn setup() -> Result<Adapter, Box<dyn std::error::Error>> {
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

        Ok(adapter)
    }

    pub async fn connect(
        light: UnknownPeripherals,
    ) -> Result<(UnknownPeripherals, Characteristic), Box<dyn std::error::Error>> {
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
        Ok((light, cmd_char.clone()))
    }

    pub async fn new() -> Result<Vec<GvmNode800D>, Box<dyn std::error::Error>> {
        let adapter = Self::setup().await?;

        let mut gvm_nodes: Vec<GvmNode800D> = vec![];
        for light in Self::find_all_gvm_lights(&adapter).await? {
            let (connected_light, cmd_char) = Self::connect(light).await?;

            gvm_nodes.push(GvmNode800D {
                id: (gvm_nodes.len() + 1) as usize,
                adapter: Arc::new(connected_light),
                characteric: Arc::new(cmd_char.clone()),
                last_received_state: GvmNodeStatus::new(),
            })
        }
        if gvm_nodes.is_empty() {
            log::error!("Did not find any GVM Nodes");
            Err(Box::new(GvmServerError::NodesNotFound))
        } else {
            Ok(gvm_nodes)
        }
    }

    pub async fn new_single(
        uid: usize,
        local_addr: &str,
    ) -> Result<GvmNode800D, Box<dyn std::error::Error>> {
        let adapter = Self::setup().await?;

        let light = Self::find_gvm_light(&adapter, local_addr)
            .await?
            .expect("GVM light not found");

        let (connected_light, cmd_char) = Self::connect(light).await?;
        Ok(GvmNode800D {
            id: uid,
            adapter: Arc::new(connected_light),
            characteric: Arc::new(cmd_char.clone()),
            last_received_state: GvmNodeStatus::new(),
        })
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let light = &self.adapter;
        let chars = light.characteristics();
        let cmd_char = chars
            .iter()
            .find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
            .expect("Unable to find characterics");

        if cmd_char.properties.contains(CharPropFlags::NOTIFY) {
            info!("unsubscribing to notifications");
            light.unsubscribe(&cmd_char).await?;
        }
        light.disconnect().await?;
        info!("disconnected from light");
        Ok(())
    }

    pub async fn get_state(&mut self) -> Result<GvmNodeStatus, Box<dyn std::error::Error>> {
        let light = &self.adapter;
        let msg: &[u8] = b"4C5409000053000001009474FF";
        let cmd = GvmNode800D::format_msg(msg);

        // send a request to receive light configuration values
        light
            .write(&self.characteric, &cmd, WriteType::WithoutResponse)
            .await?;
        let mut notification_stream = light.notifications().await?.take(2);

        // invalidate previous state
        self.last_received_state = GvmNodeStatus::new();

        while let Some(data) = notification_stream.next().await {
            info!(
                "Received data from {:?} [{:?}]: {:X?}",
                light.address(),
                data.uuid,
                data.value
            );
            match data.value[2] {
                0xE => {
                    trace!("Received states from GVM Light node");
                    self.last_received_state = GvmNodeStatus {
                        power_state: PowerState {
                            value: if data.value[6] == 1 { true } else { false },
                        },
                        brightness: Brightness {
                            value: data.value[8],
                        },
                        temperature: Temperature {
                            value: data.value[9] as u16 * 100,
                        },
                        hue: Hue {
                            value: data.value[10] as u16 * 5,
                        },
                        saturation: Saturation {
                            value: data.value[11],
                        },
                        rgb: RGB {
                            value: data.value[7],
                        },
                    };
                }
                _ => trace!(
                    "Received unknown message from GVM Light node of type: {:?}",
                    data.value[2]
                ),
            };
        }
        info!("last received states: {:?}", self.last_received_state);
        Ok(self.last_received_state)
    }

    // ASCI encoded hex bytes into raw hex values
    fn format_msg(cmd: &[u8]) -> Vec<u8> {
        let mut msg_formatted: Vec<u8> = Vec::new();

        for (upper, lower) in cmd.into_iter().tuples() {
            let ascii_to_hex = |i: &u8| if *i < 59 { *i - 48 } else { *i - 55 };
            let up = ascii_to_hex(upper);
            let lo = ascii_to_hex(lower);
            msg_formatted.push((up << 4) | lo);
        }
        msg_formatted
    }

    pub async fn send_to<'a>(&self, cmd: GvmNodeCommand) -> Result<(), Box<dyn std::error::Error>> {
        trace!("Encoding the following message: {:?}", &cmd);
        let dev_id = b"00";
        let dev_type = b"30";

        let mut buf = Vec::<u8>::new();
        buf.extend_from_slice(b"4C5409");
        buf.extend_from_slice(dev_id);
        buf.extend_from_slice(dev_type);
        buf.extend_from_slice(b"5700");
        buf.append(&mut cmd.encode());

        let buf_decoded = hex::decode(&buf)?;
        let mut crc16 = CRC::crc16xmodem();
        crc16.digest(&buf_decoded);
        let crc_str = hex::encode_upper(crc16.get_crc_vec_be());

        buf.extend_from_slice(crc_str.as_bytes());

        buf.extend_from_slice(b"FF");

        let msg_formatted = GvmNode800D::format_msg(&buf);
        info!("Writing command to GVM Light: {:X?}", msg_formatted);

        let light = &self.adapter;
        light
            .write(
                &self.characteric,
                &msg_formatted,
                WriteType::WithoutResponse,
            )
            .await?;

        let mut notification_stream = light.notifications().await?.take(1);
        // Process while the BLE connection is not broken or stopped.
        while let Some(data) = notification_stream.next().await {
            info!(
                "Received data from {:?} [{:?}]: {:X?}",
                light.address(),
                data.uuid,
                data.value
            );
        }

        Ok(())
    }
}
