use crate::gvm_server_mqtt_light_entity::{GvmStatePayload, MqttLight};
use crate::gvm_server_mqtt_options::*;
use crate::{GvmNode800D, GvmNodeCommand, GvmServerResult, MqttError, MqttGvmNode800D};
use log::{error, info, trace};
use rumqttc::{
    AsyncClient, Event, EventLoop, Incoming, LastWill, MqttOptions, Packet, Publish, QoS,
};

use std::future::poll_fn;
use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver};

pub struct Handler {
    // shared connection to nodes.
    gvm_entities: Vec<MqttGvmNode800D>,
}

impl Handler {
    fn new() -> Self {
        Self {
            gvm_entities: vec![],
        }
    }
}

pub async fn run<A, B>(
    node_id: Option<String>,
    address: A,
    credentials: Option<(B, B)>,
    nodes: Option<String>,
    shutdown: tokio::sync::oneshot::Sender<()>,
) -> GvmServerResult<()>
where
    A: ToSocketAddrs + std::fmt::Display,
    B: Into<String>,
{
    let mut server = Handler::new();
    server.connect_nodes(nodes, node_id).await?;
    let addrs = address
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("Address not configured");

    let mut eventloop = server
        .connect_broker(credentials, addrs.ip().to_string(), addrs.port() as u16)
        .await?;

    trace!("Subscribing to command topics");
    server.subscribe_commands().await?;

    // homeassistant can be really spammy!
    let (tx, rx) = mpsc::channel(20);

    tokio::spawn(async move { Handler::handle(server, rx, shutdown).await });

    loop {
        tokio::select! {
            event = eventloop.poll() => {
                match &event {
                    Ok(packet) => {
                        trace!("Received MQTT packet: {packet:?}");
                        match packet {
                            Event::Incoming(d) => {
                                let data = d.to_owned();
                                let channel = tx.clone();
                                tokio::spawn(async move {channel.send(data).await });
                            },
                            _ => trace!("Ignoring other packet"),
                        }
                    }
                    Err(e) => {
                        error!("MQTT error: {e:?}");
                        //break;
                    }
                }
            }
            _ = tx.closed() => {
                trace!("Channel is closed now!");
                break;
            }
        }
    }
    Ok(())
}

impl Handler {
    pub async fn handle(
        mut server: Self,
        mut events: Receiver<Packet>,
        mut shutdown: tokio::sync::oneshot::Sender<()>,
    ) -> GvmServerResult<()> {
        info!("Ready for MQTT messages now");
        loop {
            tokio::select! {
                _ = poll_fn(|ctx| shutdown.poll_closed(ctx)) => {
                    if shutdown.is_closed() {
                        trace!("Shutting down server");
                        server.disconnect().await?;
                    }
                    trace!("Completed");
                    break;
                }
                Some(packet) = events.recv() => {
                    trace!("Decoded packet received!");
                    match packet {
                        Incoming::Publish(msg) => server.decode(msg).await?,
                        Incoming::PingResp => server.publish_node_available().await?,
                        Incoming::ConnAck(_) => server.publish_node_available().await?,
                        _ => trace!("Ignoring other packets"),
                    }
                }
            }
        }
        Ok(())
    }
    pub async fn disconnect(&mut self) -> GvmServerResult<()> {
        for entity in &mut self.gvm_entities {
            let _ = entity.node.disconnect().await?;
        }
        Ok(())
    }
    pub async fn decode(&mut self, packet: Publish) -> GvmServerResult<()> {
        trace!("Handling packet decode");
        let topic = &packet.topic;
        let payload = packet.payload.as_ref();
        let entities = &self.gvm_entities;
        if topic.eq("homeassistant/status") {
            if let Ok(decoded) = String::from_utf8(payload.to_vec()) {
                if decoded.eq("online") {
                    info!("Sending discovery!");
                    self.send_discovery().await?;
                } else if decoded.eq("offline") {
                    trace!("Homeassistant offline?");
                }
                return Ok(());
            }
            trace!("Failed decode");
        } else if let Some(gvm_entity) = entities
            .into_iter()
            .find(|entity| topic.contains(&entity.unique_id()))
        {
            if let Ok(payload) = serde_json::from_slice::<GvmStatePayload>(&payload) {
                info!("command topic called!, {:?}", payload);
                let commands: Vec<GvmNodeCommand> = payload.try_into()?;
                info!("parsed commands: {:?}", commands);
                for command in commands {
                    gvm_entity.node.send_to(command).await?;
                }
                gvm_entity.publish_state_topics().await?;
                return Ok(());
            }
        } else {
            error!("Unexpected packet {packet:?} or topic {topic:?}");
        }
        Err(Box::new(MqttError::InvalidPayload))
    }

    pub async fn connect_broker<A>(
        &mut self,
        credentials: Option<(A, A)>,
        address: String,
        port: u16,
    ) -> GvmServerResult<EventLoop>
    where
        A: Into<String>,
    {
        let mut mqttoptions = MqttOptions::new("gvm2mqtt", address, port);
        mqttoptions.set_keep_alive(Duration::from_secs(60));
        mqttoptions.set_last_will(LastWill::new(
            self.gvm_entities
                .first()
                .unwrap()
                .create_availability_topic()
                .expect("node_id to be set"),
            payload_not_available(),
            QoS::AtMostOnce,
            false,
        ));
        if let Some((user, pass)) = credentials {
            mqttoptions.set_credentials(user, pass);
        }
        // leaks credentials!
        //trace!("Connecting to mqtt broker {mqttoptions:?}");

        trace!("Connecting to mqtt broker");
        let (broker, eventloop) = AsyncClient::new(mqttoptions, 10);
        for entity in &mut self.gvm_entities {
            entity.mqtt_light.broker = Some(broker.clone());
        }
        let hass_status_topic = "homeassistant/status";
        trace!("Subscribing to {hass_status_topic}");
        broker
            .subscribe(hass_status_topic, QoS::AtLeastOnce)
            .await?;
        Ok(eventloop)
    }

    pub async fn connect_nodes(
        &mut self,
        nodes: Option<String>,
        server_node_id: Option<String>,
    ) -> GvmServerResult<()> {
        let mut gvm_nodes: Vec<GvmNode800D> = Vec::new();
        match nodes {
            Some(nodes) => {
                trace!("Searching for specific GVM Nodes: {}", nodes);
                let mut counter = 1;
                for bt_address in nodes.split(',').collect::<Vec<_>>().into_iter() {
                    gvm_nodes.push(GvmNode800D::new_single(counter, bt_address).await?);
                    counter = counter + 1;
                }
            }
            None => {
                trace!("Searching for all GVM Nodes");
                gvm_nodes = GvmNode800D::new().await?;
            }
        }
        let mut gvm_entities: Vec<MqttGvmNode800D> = Vec::new();
        for node in gvm_nodes {
            let mqtt_light = MqttLight::new(server_node_id.clone());
            gvm_entities.push(MqttGvmNode800D { node, mqtt_light });
        }
        self.gvm_entities = gvm_entities;
        Ok(())
    }

    pub async fn subscribe_commands(&self) -> GvmServerResult<()> {
        for gvm_entity in &self.gvm_entities {
            trace!("Subscribing to commands");
            gvm_entity.subscribe_command_topics().await?;
        }
        Ok(())
    }
    pub async fn send_discovery(&self) -> GvmServerResult<()> {
        for gvm_entity in &self.gvm_entities {
            trace!("Publishing Discovery config");
            gvm_entity.publish_discovery_topic().await?;
            trace!("Publishing state");
            if gvm_entity.publish_state_topics().await.is_err() {
                // TODO error is ignored for now
                error!("Failed to get or publish state")
            }
        }
        trace!("Publishing Birth (Availability)");
        self.publish_node_available().await?;
        Ok(())
    }
    async fn publish_node_available(&self) -> Result<(), MqttError> {
        for gvm_entity in &self.gvm_entities {
            gvm_entity.publish_node_available().await?;
        }
        Ok(())
    }
}
