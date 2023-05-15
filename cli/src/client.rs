use futures::sink::SinkExt;
use gvm_server::{GvmNodeStatus, GvmServerCodec, GvmServerEvent};
use log::{info, trace};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub struct Client {
    pub uid: u8,
    framed: Framed<TcpStream, GvmServerCodec>,
}

impl Client {
    pub async fn new<A>(address: A, uid: u8) -> Result<Vec<Client>, Box<dyn std::error::Error>>
    where
        A: ToSocketAddrs + std::fmt::Display,
    {
        info!("Initialising connection! {}", address);
        let stream = TcpStream::connect(&address).await?;
        trace!("Connection: {:?}", stream);
        let mut framed = Framed::new(stream, GvmServerCodec::new());

        let mut clients = Vec::new();
        if uid == 255 {
            framed.send(GvmServerEvent::GetNodeCount).await?;
            loop {
                if let Some(frame) = framed.next().await {
                    if let Ok(GvmServerEvent::NodeCount(count)) = frame {
                        for i in 0..count {
                            let stream = TcpStream::connect(&address).await?;
                            let framed = Framed::new(stream, GvmServerCodec::new());
                            clients.push(Client { uid: i + 1, framed });
                        }
                    }
                    break;
                }
            }
        } else {
            clients.push(Client { uid, framed });
        }
        Ok(clients)
    }

    pub async fn send(
        &mut self,
        cmd: GvmServerEvent,
        get_state: bool
    ) -> Result<Option<GvmNodeStatus>, Box<dyn std::error::Error>> {
        self.framed.send(cmd).await?;
        if get_state {
            loop {
                if let Some(frame) = self.framed.next().await {
                    if let Ok(GvmServerEvent::NodeStatus(_, status)) = frame {
                        return Ok(Some(status));
                    }
                }
            }
        } else {
            Ok(None)
        }
    }
}
