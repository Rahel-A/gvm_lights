use crate::{gvm_node_command::GvmNodeCommand, gvm_node_status::GvmNodeStatus};

// Frames exchanged between any client and this server
#[derive(Clone, Copy, Debug)]
pub enum GvmServerEvent {
    GetNodeCount,
    NodeCount(u8),
    GetNodeStatus(u8),
    NodeStatus(u8, GvmNodeStatus),
    NodeCommand(u8, GvmNodeCommand),
}
