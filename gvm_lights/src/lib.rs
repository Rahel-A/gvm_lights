pub mod codec;
pub mod client;
pub mod ble_client;

pub use client::GvmClient;
pub use ble_client::{ServerMessage, GvmBleClient};
pub use codec::{ControlMessage, encode, LightCmd, ModeCmd};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
