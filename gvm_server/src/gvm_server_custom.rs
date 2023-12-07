use crate::{GvmNode800D, GvmServerCodec, GvmServerEvent, GvmServerResult};
use futures::sink::SinkExt;
use log::{error, info};
use std::future::Future;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub struct Listener {
    // accept multiple connections through this listener
    listener: TcpListener,
    // shared connection to nodes.
    gvm_nodes: Vec<GvmNode800D>,

    // for graceful shutdowns:
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
}

pub struct Handler {
    // handle communication between a single client and the server
    framed: Framed<TcpStream, GvmServerCodec>,
    // shared connection to nodes.
    gvm_nodes: Vec<GvmNode800D>,
}

pub async fn run<A>(
    address: A,
    nodes: Option<String>,
    shutdown: impl Future,
) -> Result<(), Box<dyn std::error::Error>>
where
    A: ToSocketAddrs + std::fmt::Display,
{
    let gvm_nodes = Listener::connect_nodes(nodes).await?;

    let listener = TcpListener::bind(address).await?;
    info!("Opening server on interface: {:?}", listener);

    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut server = Listener {
        listener,
        gvm_nodes,
        notify_shutdown,
        shutdown_complete_rx,
        shutdown_complete_tx,
    };
    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                error!("failed to accept conncetion: {}", err);
            }
        }
        _ = shutdown => {
            info!("shutting down server");
            for mut node in server.gvm_nodes {
                let _ = node.disconnect().await?;
                drop(node);
            }
        }
    }

    let Listener {
        mut shutdown_complete_rx,
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;

    drop(notify_shutdown);
    drop(shutdown_complete_tx);

    let _ = shutdown_complete_rx.recv().await;

    Ok(())
}

impl Listener {
    // create a connection handle for each of the GVM Light devices.
    pub async fn connect_nodes(nodes: Option<String>) -> GvmServerResult<Vec<GvmNode800D>> {
        let mut gvm_nodes: Vec<GvmNode800D> = Vec::new();
        match nodes {
            Some(nodes) => {
                info!("Searching for GVM Nodes: {}", nodes);
                let mut counter = 1;
                for bt_address in nodes.split(',').collect::<Vec<_>>().into_iter() {
                    gvm_nodes.push(GvmNode800D::new_single(counter, bt_address).await?);
                    counter = counter + 1;
                }
            }
            None => {
                gvm_nodes = GvmNode800D::new().await?;
            }
        }
        Ok(gvm_nodes)
    }

    // listen for connection requests and accept them.
    pub async fn accept(&mut self) -> GvmServerResult<TcpStream> {
        // try to accept repeatedly.
        loop {
            let (socket, _) = self.listener.accept().await?;
            return Ok(socket);
        }
    }

    async fn run(&mut self) -> GvmServerResult<()> {
        loop {
            let socket = self.accept().await?;
            let mut handler = Handler {
                framed: Framed::new(socket, GvmServerCodec::new()),
                gvm_nodes: self.gvm_nodes.clone(),
            };
            tokio::spawn(async move {
                if let Err(_) = handler.run().await {
                    error!("connection error");
                }
            });
        }
    }
}

impl Handler {
    pub async fn run(&mut self) -> GvmServerResult<()> {
        loop {
            if let Some(frame) = self.framed.next().await.transpose()? {
                match frame {
                    GvmServerEvent::GetNodeCount => {
                        self.framed
                            .send(GvmServerEvent::NodeCount(self.gvm_nodes.len() as u8))
                            .await?
                    }
                    GvmServerEvent::GetNodeStatus(uid) => {
                        let id = uid as usize;
                        if id > self.gvm_nodes.len() {
                            error!("Invalid index sent");
                            break;
                        }
                        let status = self.gvm_nodes[id - 1].get_state().await?;
                        self.framed
                            .send(GvmServerEvent::NodeStatus(uid, status))
                            .await?
                    }
                    GvmServerEvent::NodeCommand(uid, command) => {
                        let id = uid as usize;
                        if id > self.gvm_nodes.len() {
                            error!("Invalid index sent");
                            break;
                        }
                        self.gvm_nodes[id - 1].send_to(command).await?;
                    }
                    _ => error!("Incorrect command sent to the server! {:?}", frame),
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}
