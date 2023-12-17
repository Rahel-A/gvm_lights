use crate::{
    gvm_node_consts::kelvin_to_mireds,
    gvm_node_status::{units::*, GvmNodeStatus},
    GvmServerResult, MqttError,
};
use rumqttc::AsyncClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct MqttLight {
    pub broker: Option<AsyncClient>,
    pub node_id: Option<String>,
}

impl MqttLight {
    pub fn new(node_id: Option<String>) -> Self {
        Self {
            broker: None,
            node_id,
        }
    }
}

// https://www.home-assistant.io/integrations/light.mqtt/#supported_color_modes
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum ColorMode {
    onoff,
    brightness,
    color_temp,
    hs,
    xy,
    rgb,
    rgbw,
    rgbww,
    white,
}

#[derive(Serialize, Deserialize, Debug)]
struct RGBMode {
    r: u8,
    g: u8,
    b: u8,
}
#[derive(Serialize, Deserialize, Debug)]
struct RGBWMode {
    #[serde(flatten)]
    rgb: RGBMode,
    w: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct RGBWWMode {
    #[serde(flatten)]
    rgbw: RGBWMode,
    c: u8,
}

// https://www.home-assistant.io/integrations/light.mqtt/#hs_command_topic
#[derive(Serialize, Deserialize, Debug)]
pub struct HSMode {
    pub h: f32, // 0°..360°
    pub s: f32, // 0..100
}
#[derive(Serialize, Deserialize, Debug)]
struct XYMode {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    rgb: Option<RGBMode>, // N/A?
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    rgbw: Option<RGBWMode>, // N/A
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    rgbww: Option<RGBWWMode>, // N/A
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    xy: Option<XYMode>, // N/A
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub hs: Option<HSMode>,
}

impl Colors {
    fn new() -> Colors {
        Colors {
            rgb: None,
            rgbw: None,
            rgbww: None,
            xy: None,
            hs: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    ON,
    OFF,
}

// these names are my interpretation of the effects.
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Effect {
    flicker_lightning,
    police_strobe,
    flicker_warm,
    flicker_cool,
    flicker_loose_bulb,
    cycle_colors,
    disco,
    flicker_photoshoot,
}

// https://www.home-assistant.io/integrations/light.mqtt/#json-schema
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GvmHassTopic {
    brightness,
    color_mode,
    color_temp,
    color,
    effect,
    state,
    transition,
    availability,
    node_state,
}

impl GvmHassTopic {
    pub fn topic(&self) -> String {
        match self {
            GvmHassTopic::brightness => String::from("brightness"),
            GvmHassTopic::color_mode => String::from("color_mode"),
            GvmHassTopic::color_temp => String::from("color_temp"),
            GvmHassTopic::effect => String::from("effect"),
            GvmHassTopic::color => String::from("color"),
            GvmHassTopic::state => String::from("power"),
            GvmHassTopic::availability => String::from("availability"),
            GvmHassTopic::node_state => String::from("state"),
            _ => todo!(),
        }
    }
}

// https://www.home-assistant.io/integrations/light.mqtt/#json-schema
/// ```
/// use serde_json::json;
/// use gvm_server::gvm_server_mqtt_light_entity::GvmStatePayload;
/// let data = json!({
///   "brightness": 255,
///   "color_mode":  "rgb",
///   "color_temp": 155,
///   "color": {
///     "r": 255,
///     "g": 180,
///     "b": 200,
///     "c": 100,
///     "w": 50,
///     "x": 0.406,
///     "y": 0.301,
///     "h": 344.0,
///     "s": 29.412
///  },
///  "effect": "disco",
///  "state": "ON",
///  "transition": 2,
/// });
/// let topic_payload: GvmStatePayload = serde_json::from_value(data).unwrap();
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct GvmStatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_mode: Option<ColorMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temp: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Colors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<u8>,
}

impl GvmStatePayload {
    pub fn new() -> Self {
        Self {
            brightness: None,
            color_mode: None,
            color_temp: None,
            color: None,
            effect: None,
            state: None,
            transition: None,
        }
    }
    pub fn available() -> Self {
        let mut value = Self::new();
        value.state = Some(State::ON);
        value
    }
    pub fn not_available() -> Self {
        let mut value = Self::new();
        value.state = Some(State::ON);
        value
    }
}

impl From<GvmNodeStatus> for GvmStatePayload {
    fn from(value: GvmNodeStatus) -> Self {
        let mut colors = Colors::new();
        colors.hs = Some(HSMode {
            h: value.hue.value as f32,
            s: value.saturation.value as f32,
        });
        Self {
            brightness: Some(value.brightness.value),
            color_mode: None, // TODO!
            color_temp: Some(kelvin_to_mireds(value.temperature.value as u32) as u32),
            color: Some(colors),
            effect: None, // TODO
            state: Some(value.power_state.value.into()),
            transition: None, // TODO
        }
    }
}

impl From<Brightness> for GvmStatePayload {
    fn from(value: Brightness) -> Self {
        let mut retval = GvmStatePayload::new();
        retval.brightness = Some(value.value);
        retval
    }
}

impl From<bool> for State {
    fn from(value: bool) -> Self {
        if value {
            State::ON
        } else {
            State::OFF
        }
    }
}
impl From<PowerState> for GvmStatePayload {
    fn from(value: PowerState) -> Self {
        let mut retval = GvmStatePayload::new();
        retval.state = Some(value.value.into());
        retval
    }
}

impl From<Temperature> for GvmStatePayload {
    fn from(value: Temperature) -> Self {
        let mut retval = GvmStatePayload::new();
        retval.color_temp = Some(kelvin_to_mireds(value.value as u32) as u32);
        retval
    }
}

impl From<(Hue, Saturation)> for GvmStatePayload {
    fn from(value: (Hue, Saturation)) -> Self {
        let mut colors = Colors::new();
        colors.hs = Some(HSMode {
            h: value.0.value as f32,
            s: value.1.value as f32,
        });
        let mut retval = GvmStatePayload::new();
        retval.color = Some(colors);
        retval
    }
}
