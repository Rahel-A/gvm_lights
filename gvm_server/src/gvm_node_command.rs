use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LightCmd {
    On,
    Off,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ModeCmd {
    ColourTemp,
    HueSat,
    Scenes,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GvmNodeCommand {
    Light(LightCmd),
    Mode(ModeCmd),
    Brightness(u8),
    Temperature(u16),
    Hue(u16),
    Saturation(u8),
    RGB(u8),
    Scene(u8),
}
