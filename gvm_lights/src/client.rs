use crate::codec::{ControlMessage, encode};
use async_std::net::UdpSocket;
use log::trace;

pub struct GvmClient {
    socket: UdpSocket,
}

impl GvmClient {
    pub async fn new(local_addr: &str) -> Result<GvmClient, Box<dyn std::error::Error>> {
        Ok(GvmClient {
            socket: UdpSocket::bind(format!("{}:0", local_addr)).await?,
        })
    }

    pub async fn send_to(&self, target: &str, cmd: &ControlMessage) -> Result<(), Box<dyn std::error::Error>> {
        // socket.set_broadcast(true)?;
        // let (n, peer) = socket.recv_from(&mut buf).await?;
        let msg = encode(cmd)?;
        let sent_bytes = self.socket.send_to(&msg, &format!("{}:2525", target)).await?;
        trace!("{:X?}", msg);
        Ok(())
    }
}