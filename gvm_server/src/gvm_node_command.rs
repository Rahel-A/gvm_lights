use serde::{Deserialize, Serialize};

use crate::{
    gvm_node_consts::mireds_to_kelvin,
    gvm_server_mqtt_light_entity::{ColorMode, Effect, GvmStatePayload, HSMode, State},
    GvmNodeError,
};

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

impl From<Effect> for SceneCmd {
    fn from(value: Effect) -> Self {
        match value {
            Effect::flicker_lightning => SceneCmd::Lightning,
            Effect::police_strobe => SceneCmd::CopCar,
            Effect::flicker_warm => SceneCmd::Candle,
            Effect::flicker_cool => SceneCmd::TV,
            Effect::flicker_loose_bulb => SceneCmd::BadBulb,
            Effect::cycle_colors => SceneCmd::Party,
            Effect::disco => SceneCmd::Disco,
            Effect::flicker_photoshoot => SceneCmd::Paparazzi,
        }
    }
}

impl From<State> for LightCmd {
    fn from(value: State) -> Self {
        match value {
            State::ON => LightCmd::On,
            State::OFF => LightCmd::Off,
        }
    }
}

impl TryFrom<GvmStatePayload> for Vec<GvmNodeCommand> {
    type Error = GvmNodeError;

    fn try_from(value: GvmStatePayload) -> Result<Self, Self::Error> {
        //let mut commands: Vec<GvmNodeCommand> = vec![];
        let mut commands: Vec<Option<GvmNodeCommand>> = vec![];
        commands.push(value.brightness.map(|v| GvmNodeCommand::Brightness(v)));
        commands.push(value.color_mode.map(|v| match v {
            ColorMode::hs => GvmNodeCommand::Mode(ModeCmd::HueSat),
            ColorMode::color_temp => GvmNodeCommand::Mode(ModeCmd::ColourTemp),
            _ => todo!(),
        }));
        commands.push(
            value
                .color_temp
                .map(|v| GvmNodeCommand::Temperature(mireds_to_kelvin(v as f32) as u16)),
        );

        if let Some(mut map_colors) = value.color.map(|v| {
            if let Some(HSMode { h, s }) = v.hs {
                vec![
                    Some(GvmNodeCommand::Hue(h as u16)),
                    Some(GvmNodeCommand::Saturation(s as u8)),
                ]
            } else {
                vec![]
            }
        }) {
            commands.append(&mut map_colors);
        }

        commands.push(value.effect.map(|v| GvmNodeCommand::Scene(v.into())));
        commands.push(value.state.map(|v| GvmNodeCommand::Light(v.into())));

        Ok(commands.iter().filter_map(|c| c.to_owned()).collect())
    }
}
