use gvm_lights::{ServerMessage, ControlMessage};
use log::{info, trace};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[derive(Debug)]
pub struct Client {
    pub uid: u8,
    stream: TcpStream
}

impl Client {
    pub async fn new<A>(address: A, uid: u8)
            -> Result<Vec<Client>, Box<dyn std::error::Error>>
        where A: ToSocketAddrs + std::fmt::Display {
        info!("Initialising connection! {}", address);
        let mut stream = TcpStream::connect(&address).await?;
        let mut clients = Vec::new();
        if uid == 255 {
            // create a client for each gvm light in server.
            stream.write_all("get_clients".as_bytes()).await?;
            let n = loop {
                let n = stream.read_u8().await?;
                println!("read: {n}");
                break n;
            };
            for i in 0..n {
                let stream = TcpStream::connect(&address).await?;
                clients.push(Client{uid:i+1, stream});
            }
        } else {
            clients.push(Client{uid, stream});
        }
        Ok(clients)
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

    async fn receive_message(&mut self)
        -> Result<Vec<ServerMessage>, Box<dyn std::error::Error>> {
        let states = loop {
            let n = self.stream.read_u8().await?;
            let mut client_states = Vec::new();
            for i in 0..n {
                let (id,states) = loop {
                    let mut buffer = [0; 500];
                    let mut id = 0;
                    let n = self.stream.read(&mut buffer).await?;
                    if let Ok(msgs) = serde_json::from_slice::<Vec<ServerMessage>>(&buffer[0..n]) {
                        let mut states = Vec::new();
                        for ServerMessage{client:_id, mut msg} in msgs {
                            id = _id;
                            trace!("Received message from server! {:?}", msg);
                            states.push(msg.remove(0));
                        };
                        break (id,states);
                    }
                    else {
                         panic!("Unknown message from server: {:?}", String::from_utf8(buffer[..n].to_vec()));
                    };
                };
                client_states.push(ServerMessage{client: id, msg:states});
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
