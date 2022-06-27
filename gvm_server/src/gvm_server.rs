use gvm_lights::{GvmBleClient, ServerMessage, ControlMessage};
use gvm_lights::encode;
use log::info;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::io::AsyncWriteExt;

pub struct Server {
    listener: TcpListener,
    gvm_clients: Vec<GvmBleClient>
}

impl Server {
    pub async fn new<A>(address: A, clients: String)
        -> Result<Server, Box< dyn std::error::Error>>
        where A: ToSocketAddrs + std::fmt::Display {
        info!("Opening server on interface: {}", address);
        let listener = TcpListener::bind(address).await?;

        let mut gvm_clients: Vec<GvmBleClient> = Vec::new();
        let mut counter = 1;
        for bt_address in clients.split(',').collect::<Vec<_>>().into_iter() {
            gvm_clients.push(GvmBleClient::new(counter, bt_address).await?);
            counter = counter + 1;
        };
        Ok(Server {listener, gvm_clients})
    }
    pub async fn run(&mut self) -> Result<Server, Box<dyn std::error::Error>> {
        loop {
            let (mut socket, _) = self.listener.accept().await?;
            socket.readable().await?;

            let mut buffer = [0; 50];
            let n = socket.try_read(&mut buffer)?;
            info!("Received message from client! {:?}", &buffer[..n]);

            match serde_json::from_slice(&buffer[..n]) {
                Ok(ServerMessage{client, msg}) => {
                    let filtered_clients: Vec<_> = self.gvm_clients
                        .clone()
                        .into_iter()
                        // select either a target client or ALL clients
                        .filter(|gvm_client| gvm_client.id() == client || client == 255)
                        .collect();
                    socket.write_u8(filtered_clients.len() as u8).await?;
                    socket.flush().await?;
                    for gvm_client in filtered_clients {
                        // client can send multiple commands (actions)
                        for action in msg.iter() {
                            match action {
                                ControlMessage::ReadState() => {
                                    // get list of states of the gvm client to send back
                                    let states = gvm_client.get_state()
                                        .await
                                        .unwrap();
                                    let cmd_json = serde_json::to_string(&states
                                        .into_iter()
                                        .map(|state| ServerMessage{client:gvm_client.id(), msg:vec![state]})
                                        .collect::<Vec<ServerMessage>>())?;
                                    socket.write(cmd_json.as_bytes()).await?;
                                    socket.write("\n".as_bytes()).await?;
                                    socket.flush().await?;
                                },
                                _ => {
                                    // set new state on the gvm client
                                    gvm_client.send_to(&encode(&action).unwrap())
                                        .await
                                        .expect("Failed to send message to GVM Client");
                                }
                            };
                        }
                    }
                },
                _ => eprintln!("Received unexpected message from client"),
            }
        }
    }
}
