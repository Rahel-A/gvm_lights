use gvm_lights::{ServerMessage, ControlMessage};
use log::{info, trace};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[derive(Debug)]
pub struct Client {
    uid: u8,
    stream: TcpStream
}

impl Client {
    pub async fn new<A>(address: A, uid: u8)
            -> Result<Client, Box<dyn std::error::Error>>
        where A: ToSocketAddrs + std::fmt::Display {
        info!("Initialising connection! {}", address);
        let stream = TcpStream::connect(address).await?;
        Ok(Client{uid, stream})
    }

    pub async fn send_message(&mut self, msg: Vec<ControlMessage>)
            -> Result<Option<Vec<ServerMessage>>, Box<dyn std::error::Error>> {
        let receive = msg.contains(&ControlMessage::ReadState());
        let client = self.uid;
        let cmd_json = serde_json::to_string(&ServerMessage{client, msg})?;
        info!("Sending message to server! {:?}", &cmd_json);
        self.stream.write_all(cmd_json.as_bytes()).await?;

        let states = if receive {
                Some(self.receive_message().await?)
            } else {
                None
            };
        Ok(states)
    }

    async fn receive_message(&mut self) -> Result<Vec<ServerMessage>, Box<dyn std::error::Error>> {
        let states = loop {
            let n = self.stream.read_u8().await?;
            let mut client_states = Vec::new();
            for i in 0..n {
                let states = loop {
                    let mut buffer = [0; 500];
                    let n = self.stream.read(&mut buffer).await?;
                    if let Ok(msgs) = serde_json::from_slice::<Vec<ServerMessage>>(&buffer[0..n]) {
                        let mut states = Vec::new();
                        for ServerMessage{client:_, mut msg} in msgs {
                            trace!("Received message from server! {:?}", msg);
                            states.push(msg.remove(0));
                        };
                        break states;
                    }
                    else {
                         panic!("Unknown message from server: {:?}", String::from_utf8(buffer[..n].to_vec()));
                    };
                };
                client_states.push(ServerMessage{client: (i+1), msg:states});
                // Read the newline (terminator)!
                if i != (n-1) {
                    self.stream.read_u8().await?;
                }
            }
            break client_states;
        };
        Ok(states)
    }
}
