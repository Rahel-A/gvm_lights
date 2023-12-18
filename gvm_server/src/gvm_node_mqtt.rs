use crate::gvm_node::GvmNode800D;
use crate::gvm_node_consts::*;
use crate::gvm_server_mqtt_light_entity::{Effect, GvmHassTopic, GvmStatePayload, MqttLight};
use crate::{gvm_server_mqtt_options, MqttError};
use log::trace;
use rumqttc::QoS;
use serde_json::json;

// A pair that bridges the gap between broker and the hardware
pub struct MqttGvmNode800D {
    pub node: GvmNode800D,
    pub mqtt_light: MqttLight,
}

macro_rules! subscribe {
    ($v:ident, $topic:expr, $qos:expr) => {
        trace!("subscribing to: {:?}", $topic);
        $v.mqtt_light
            .broker
            .as_ref()
            .expect("broker not configured")
            .subscribe($topic, $qos)
            .await
            .expect("mqtt subscribe failed")
    };
}

macro_rules! publish {
    ($v:ident, $topic:expr, $qos:expr, $retain:expr, $data:expr) => {
        trace!("Publishing on {:?}: {:?}", $topic, $data);
        $v.mqtt_light
            .broker
            .as_ref()
            .expect("broker not configured")
            .publish($topic, $qos, $retain, $data)
            .await
            .expect("mqtt publish failed")
    };
}

impl MqttGvmNode800D {
    pub fn unique_id(&self) -> String {
        format!("gvm_{}", self.node.id())
    }
    fn device_id(&self) -> String {
        self.node.hid()
    }
    // requires node_id and unique_id
    // unique_id generated with each hardware node
    // node_id is the server's location/id
    fn topic_base_for(&self, shorten: bool) -> Result<String, MqttError> {
        let base = if shorten {
            String::from("~")
        } else {
            format!(
                "homeassistant/light/{}/{}",
                self.mqtt_light.node_id.as_ref().unwrap(),
                self.unique_id()
            )
        };
        Ok(base)
    }
    pub fn create_state_topic(
        &self,
        state_type: &GvmHassTopic,
        shorten: bool,
    ) -> Result<String, MqttError> {
        let base = self
            .topic_base_for(shorten)
            .expect("failed base topic generation");
        let topic_type = state_type.topic();
        let topic = format!("{base}/{topic_type}");
        Ok(topic)
    }
    pub async fn publish_state_topics(&self) -> Result<(), MqttError> {
        if let Ok(node_status) = self.node.get_state().await {
            let state: GvmStatePayload = node_status.into();
            publish!(
                self,
                self.create_state_topic(&GvmHassTopic::node_state, false)?,
                QoS::AtLeastOnce,
                false,
                serde_json::to_string(&state).unwrap()
            );
            Ok(())
        } else {
            Err(MqttError::UnavailableNodeStatus)
        }
    }
    pub fn create_command_topic(
        &self,
        command_type: &GvmHassTopic,
        shorten: bool,
    ) -> Result<String, MqttError> {
        let base = self
            .topic_base_for(shorten)
            .expect("failed base topic generation");
        let topic_type = command_type.topic();
        let topic = format!("{base}/{topic_type}/set");
        Ok(topic)
    }
    pub async fn subscribe_command_topics(&self) -> Result<(), MqttError> {
        subscribe!(
            self,
            self.create_command_topic(&GvmHassTopic::state, false)?,
            QoS::AtLeastOnce
        );
        Ok(())
    }
    // manage availability separately (and not per gvm node) because of how last_will works
    pub fn create_availability_topic(&self) -> Result<String, MqttError> {
        if let Some(node_id) = self.mqtt_light.node_id.clone() {
            Ok(format!("homeassistant/light/{node_id}/status"))
        } else {
            Ok(format!("homeassistant/light/status"))
        }
    }
    /// warning: ATM this doesn't publish that each entity is available,
    /// rather the status is shared, and defines that the mqttclient is available
    pub async fn publish_node_available(&self) -> Result<(), MqttError> {
        publish!(
            self,
            self.create_availability_topic()?,
            QoS::AtLeastOnce,
            false,
            gvm_server_mqtt_options::payload_available()
        );
        Ok(())
    }
    // Discovery config
    // <discovery_prefix>/<component>/[<node_id>/]<object_id>/config
    pub async fn publish_discovery_topic(&self) -> Result<(), MqttError> {
        let topic = format!(
            "{}/config",
            self.topic_base_for(false)
                .expect("failed base topic generation")
        );
        // TODO: min_mireds and max_mireds?
        // availability?
        if let Ok(discovery_config_payload) = serde_json::to_vec(&json!({
            "name": null,
            "device_class": "gvm_smart_lights",
            "~": self.topic_base_for(false).expect("failed base topic generation"),

            // TODO better way to do this?
            // config/topic builders?
            "state_topic": self
                .create_state_topic(&GvmHassTopic::node_state, true)
                .expect("failed state topic generation"),

            "payload_available": gvm_server_mqtt_options::payload_available(),
            "payload_not_available": gvm_server_mqtt_options::payload_not_available(),
            "availability_topic": self.create_availability_topic()?,

            // optimistic mode:
            // "optimistic": true,
            "command_topic": self
                .create_command_topic(&GvmHassTopic::state, true)
                .expect("failed command topic generation"),

            "brightness": true,
            "brightness_command_topic": self
                .create_command_topic(&GvmHassTopic::brightness, true)
                .expect("failed command topic generation"),
            "color_temp_command_topic": self
                .create_command_topic(&GvmHassTopic::color_temp, true)
                .expect("failed command topic generation"),
            "effect_command_topic": self
                .create_command_topic(&GvmHassTopic::effect, true)
                .expect("failed command topic generation"),
            "hs_command_topic": self
                .create_command_topic(&GvmHassTopic::color_mode, true)
                .expect("failed command topic generation"),

            "effect": true,
            "effect_list": [
                Effect::flicker_lightning,
                Effect::police_strobe,
                Effect::flicker_warm,
                Effect::flicker_cool,
                Effect::flicker_loose_bulb,
                Effect::cycle_colors,
                Effect::disco,
                Effect::flicker_photoshoot,
             ],
            "icon": "mdi:spotlight",
            "color_mode": true,
            "supported_color_modes": [ "color_temp", "hs"],
            "max_mireds": kelvin_to_mireds(MAX_TEMPERATURE),
            "min_mireds": kelvin_to_mireds(MIN_TEMPERATURE),
            "unique_id": self.unique_id(),
            "schema": "json",
            "device": {
                "name": format!("gvm_node_{}", self.unique_id()),
                "connections": [ ["mac", self.device_id()] ],
                // TODO?
                "mf": "GVM",
                "mdl": "GVM 800D",
            },
            "origin": {
                "name": "gvm2mqtt",
                "sw": format!("{}", env!("CARGO_PKG_VERSION")),
                "url": format!("https://github.com/Rahel-A/gvm_server"),
            },
        })) {
            publish!(
                self,
                topic,
                QoS::AtLeastOnce,
                false,
                discovery_config_payload
            );
        } else {
            trace!("Failed to create discovery payload")
        }
        Ok(())
    }
}
