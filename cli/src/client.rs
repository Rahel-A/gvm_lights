use gvm_lights::{ServerMessage, ControlMessage};
use log::info;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

pub struct Client {
    uid: u8,
    stream: TcpStream
}

impl Client {
    pub async fn new<A>(address: A, uid: u8)
            -> Result<Client, Box<dyn std::error::Error>>
        where A: ToSocketAddrs + std::fmt::Display {
        let stream = TcpStream::connect(address).await?;
        Ok(Client{uid, stream})
    }

    pub async fn send_message(&mut self, msg: Vec<ControlMessage>)
            -> Result<(), Box<dyn std::error::Error>> {
        let cmd_json = serde_json::to_string(&ServerMessage{client:self.uid, msg})?;
        info!("Sending message to server! {:?}", &cmd_json);
        self.stream.write_all(cmd_json.as_bytes()).await?;
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Result<Vec<ControlMessage>, Box<dyn std::error::Error>> {
        let states = loop {
            let mut buffer = [0; 500];
            let n = self.stream.read(&mut buffer).await?;
            if let Ok(msgs) = serde_json::from_slice::<Vec<ServerMessage>>(&buffer[..n]) {
                let mut states = Vec::new();
                for ServerMessage{client:_, mut msg} in msgs {
                    info!("Received message from server! {:?}", msg);
                    states.push(msg.remove(0));
                };
                break states;
            }
            else {
                 panic!("Unknown message from server: {:?}", String::from_utf8(buffer[..n].to_vec()));
            };
        };
        Ok(states)
    }
}
