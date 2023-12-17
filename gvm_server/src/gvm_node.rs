use crc_any::CRC;
use futures::stream::StreamExt;
use itertools::Itertools;
use log::{error, info, trace};
use std::future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time;
use uuid::Uuid;

use btleplug::api::{
    Central, CharPropFlags, Characteristic, Manager as _, Peripheral, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral as UnknownPeripherals};

use crate::gvm_node_status::units::*;
use crate::{GvmNodeCommand, GvmNodeError, GvmNodeResult, GvmNodeStatus, NodeCommandEncoder};

#[derive(Clone)]
pub struct GvmNode800D {
    id: usize,
    adapter: Arc<UnknownPeripherals>,
    characteric: Arc<Characteristic>,
}

const LIGHT_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x00010203_0405_0607_0809_0a0b0c0d2b10);
const LIGHT_NAME: &'static str = "BT_LED";

impl GvmNode800D {
    async fn find_all_gvm_lights(adapter: &Adapter) -> GvmNodeResult<Vec<UnknownPeripherals>> {
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
    ) -> GvmNodeResult<Option<UnknownPeripherals>> {
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
    pub fn hid(&self) -> String {
        self.adapter.address().to_string()
    }

    pub async fn close(&self) -> GvmNodeResult<()> {
        Ok(self.adapter.disconnect().await?)
    }

    pub async fn setup() -> GvmNodeResult<Adapter> {
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
    ) -> GvmNodeResult<(UnknownPeripherals, Characteristic)> {
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

    pub async fn new() -> GvmNodeResult<Vec<GvmNode800D>> {
        let adapter = Self::setup().await?;

        let mut gvm_nodes: Vec<GvmNode800D> = vec![];
        for light in Self::find_all_gvm_lights(&adapter).await? {
            let (connected_light, cmd_char) = Self::connect(light).await?;

            gvm_nodes.push(GvmNode800D {
                id: (gvm_nodes.len() + 1) as usize,
                adapter: Arc::new(connected_light),
                characteric: Arc::new(cmd_char.clone()),
            })
        }
        if gvm_nodes.is_empty() {
            log::error!("Did not find any GVM Nodes");
            Err(Box::new(GvmNodeError::NodesNotFound))
        } else {
            Ok(gvm_nodes)
        }
    }

    pub async fn new_single(uid: usize, local_addr: &str) -> GvmNodeResult<GvmNode800D> {
        let adapter = Self::setup().await?;

        let light = Self::find_gvm_light(&adapter, local_addr)
            .await?
            .expect("GVM light not found");

        let (connected_light, cmd_char) = Self::connect(light).await?;
        Ok(GvmNode800D {
            id: uid,
            adapter: Arc::new(connected_light),
            characteric: Arc::new(cmd_char.clone()),
        })
    }

    pub async fn disconnect(&mut self) -> GvmNodeResult<()> {
        let start = Instant::now();
        let light = &self.adapter;
        let chars = light.characteristics();
        let cmd_char = chars
            .iter()
            .find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
            .expect("Unable to find characterics");

        if cmd_char.properties.contains(CharPropFlags::NOTIFY) {
            // TODO does disconnect or unsubscribe take long time?
            info!("unsubscribing to notifications");
            light.unsubscribe(&cmd_char).await?;
        }
        light.disconnect().await?;
        let duration = start.elapsed();
        info!(target: "gvm_server_bench", "disconnected from light (took {duration:?})");
        Ok(())
    }

    pub async fn get_state(&self) -> GvmNodeResult<GvmNodeStatus> {
        let start = Instant::now();
        let light = &self.adapter;
        let msg: &[u8] = b"4C5409000053000001009474FF";
        let cmd = GvmNode800D::format_msg(msg);

        // send a request to receive light configuration values
        light
            .write(&self.characteric, &cmd, WriteType::WithoutResponse)
            .await?;

        let mut received_state = None;

        if let Some(data) = light
            .notifications()
            .await?
            .skip_while(|data| future::ready(data.value.capacity() > 2 && data.value[2] != 0xE))
            .next()
            .await
        {
            trace!("data: {:?}", data.value);
            received_state = Some(GvmNodeStatus {
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
            });
        } else {
            error!("get_state failed")
        }

        let duration = start.elapsed();
        info!(target: "gvm_server_bench", "received states in {duration:?} {:?}", received_state);
        received_state.ok_or(Box::new(GvmNodeError::StateRetrievalFailed))
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

    pub async fn send_to<'a>(&self, cmd: GvmNodeCommand) -> GvmNodeResult<()> {
        let start = Instant::now();
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
        trace!("Writing command to GVM Light: {:X?}", msg_formatted);

        let light = &self.adapter;
        light
            .write(
                &self.characteric,
                &msg_formatted,
                WriteType::WithoutResponse,
            )
            .await?;

        let duration = start.elapsed();
        info!(target: "gvm_server_bench", "Finished sending command to gvm node in {duration:?}");

        Ok(())
    }
}
