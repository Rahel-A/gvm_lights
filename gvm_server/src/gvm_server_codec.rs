use crate::gvm_node_status::GvmNodeStatus;
use crate::gvm_server_consts::*;
use crate::gvm_server_error::*;
use crate::gvm_server_event::*;

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use log::{error, trace};

pub struct GvmServerCodec {}

impl GvmServerCodec {
    pub fn new() -> GvmServerCodec {
        GvmServerCodec {}
    }
}

impl Encoder<GvmServerEvent> for GvmServerCodec {
    type Error = GvmServerError;

    fn encode(&mut self, item: GvmServerEvent, dst: &mut BytesMut) -> Result<(), Self::Error> {
        trace!("Encoder started");
        match item {
            GvmServerEvent::GetNodeCount => {
                trace!("Encoding get node count");
                dst.put_u8(GET_NODE_COUNT);
            }
            GvmServerEvent::NodeCount(count) => {
                trace!("Encoding node count");
                dst.put_u8(NODE_COUNT);
                dst.put_u8(count.into());
            }
            GvmServerEvent::GetNodeStatus(uid) => {
                trace!("Encoding get node status");
                dst.put_u8(GET_NODE_STATUS);
                dst.put_u8(uid);
            }
            GvmServerEvent::NodeStatus(uid, node_status) => {
                trace!("Encoding node status");
                dst.put_u8(NODE_STATUS);
                dst.put_u8(uid);
                let serialized = bincode::serialize(&node_status).map_err(bincode_to_io_error)?;
                dst.put_slice(&serialized);
            }
            GvmServerEvent::NodeCommand(uid, node_command) => {
                trace!("Encoding node command");
                dst.put_u8(NODE_COMMAND);
                dst.put_u8(uid);
                let serialized = bincode::serialize(&node_command).map_err(bincode_to_io_error)?;
                dst.put_slice(&serialized);
            }
        };
        Ok(())
    }
}

impl Decoder for GvmServerCodec {
    type Item = GvmServerEvent;
    type Error = GvmServerError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        trace!("Decoder started! {:?}, len: {}", src, src.len());
        let mut i = 0;
        let len = src.len();
        if len == 0 {
            return Ok(None);
        }
        match src[i] {
            GET_NODE_COUNT => {
                trace!("Decoding get number of nodes");
                src.advance(i + 1);
                return Ok(Some(GvmServerEvent::GetNodeCount));
            }

            NODE_COUNT => {
                trace!("Decoding number of nodes");
                if len <= 1 {
                    trace!("Waiting for more data");
                    return Ok(None);
                }

                i += 1;
                let count = src[i];
                trace!("number of nodes found: {}", count);

                src.advance(i + 1);
                return Ok(Some(GvmServerEvent::NodeCount(count)));
            }
            GET_NODE_STATUS => {
                trace!("Decoding get node status");
                if len <= 1 {
                    trace!("Waiting for more data");
                    return Ok(None);
                }

                i += 1;
                let uid = src[i];
                trace!("node status required for uid: {}", uid);

                src.advance(i + 1);
                return Ok(Some(GvmServerEvent::GetNodeStatus(uid)));
            }
            NODE_STATUS => {
                trace!("Decoding node status");
                let max_length = bincode::serialized_size(&GvmNodeStatus::new())
                    .map_err(bincode_to_io_error)? as usize;

                if len <= max_length {
                    trace!(
                        "Waiting for more data, length {}, max length {}",
                        len,
                        max_length
                    );
                    return Ok(None);
                }

                i += 1;
                let uid = src[i];

                i += 1;
                if let Ok(node_status) = bincode::deserialize(&src[i..]) {
                    trace!("node ({}) has status: {:?}", uid, node_status);

                    let len =
                        bincode::serialized_size(&node_status).map_err(bincode_to_io_error)?;
                    src.advance(i + len as usize);
                    Ok(Some(GvmServerEvent::NodeStatus(uid, node_status)))
                } else {
                    error!("Failed to deserialize buffer: {:?}", src);
                    Err(GvmServerError::ParsingError)
                }
            }
            NODE_COMMAND => {
                trace!("Decoding node command");
                if len <= 3 {
                    // size is assumed to be pretty small!
                    trace!("Waiting for more data");
                    return Ok(None);
                }

                i += 1;
                let uid = src[i];

                i += 1;
                if let Ok(node_command) = bincode::deserialize(&src[i..]) {
                    trace!(
                        "node ({}) shall perform the following command: {:?}",
                        uid,
                        node_command
                    );

                    let len =
                        bincode::serialized_size(&node_command).map_err(bincode_to_io_error)?;
                    src.advance(i + len as usize);
                    Ok(Some(GvmServerEvent::NodeCommand(uid, node_command)))
                } else {
                    error!("Failed to deserialize buffer: {:?}", src);
                    Err(GvmServerError::ParsingError)
                }
            }

            _ => panic!("todo not done yet!"),
        }
    }
}
