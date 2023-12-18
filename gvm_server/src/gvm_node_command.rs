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

// these names match the ones from GVM app
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SceneCmd {
    Lightning = 0,
    CopCar = 1,
    Candle = 2,
    TV = 3,
    BadBulb = 4,
    Party = 5,
    Disco = 6,
    Paparazzi = 7,
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
    Scene(SceneCmd),
}
