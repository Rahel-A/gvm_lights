#[cfg(not(feature = "mqtt"))]
pub mod gvm_server_custom;
#[cfg(not(feature = "mqtt"))]
pub use gvm_server_custom::run;

pub mod gvm_server_consts;
pub mod gvm_server_error;
pub use gvm_server_error::GvmServerError;
pub mod gvm_server_event;
pub use gvm_server_event::GvmServerEvent;
pub mod gvm_server_codec;
pub use gvm_server_codec::GvmServerCodec;

mod gvm_node;
use gvm_node::GvmNode800D;
mod gvm_node_encoder;
use gvm_node_encoder::NodeCommandEncoder;

pub mod gvm_node_command;
pub use gvm_node_command::{GvmNodeCommand, LightCmd, ModeCmd, SceneCmd};
pub mod gvm_node_status;
pub use gvm_node_status::GvmNodeStatus;

pub type Error = GvmServerError;
pub type GvmServerResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
