use crate::gvm_node_command::{GvmNodeCommand, LightCmd, ModeCmd, SceneCmd};

pub trait NodeCommandEncoder {
    fn encode(&self) -> Vec<u8>;
}

impl NodeCommandEncoder for GvmNodeCommand {
    fn encode(&self) -> Vec<u8> {
        let (cmd, mut param) = match self {
            GvmNodeCommand::Light(param) => match param {
                LightCmd::On => (b"00", b"01".to_vec()),
                LightCmd::Off => (b"00", b"00".to_vec()),
            },
            GvmNodeCommand::RGB(rgb) => {
                // TODO: what is this (hue?)?
                let rgb = if *rgb >= 255 { 255 } else { *rgb };
                (b"01", hex::encode_upper([rgb]).into_bytes())
            }
            GvmNodeCommand::Brightness(br) => {
                let br = if *br > 100 { 100 } else { *br };
                (b"02", hex::encode_upper([br]).into_bytes())
            }
            GvmNodeCommand::Temperature(t) => {
                let t = if *t < 3200 {
                    3200
                } else {
                    if *t > 5600 {
                        5600
                    } else {
                        *t
                    }
                };
                (b"03", hex::encode_upper([(t / 100) as u8]).into_bytes())
            }
            GvmNodeCommand::Hue(hue) => {
                let hue = if *hue > 360 { 360 } else { *hue };
                (b"04", hex::encode_upper([(hue / 5) as u8]).into_bytes())
            }
            GvmNodeCommand::Saturation(sat) => {
                let sat = if *sat > 100 { 100 } else { *sat };
                (b"05", hex::encode_upper([sat]).into_bytes())
            }
            GvmNodeCommand::Mode(mode) => match mode {
                ModeCmd::ColourTemp => (b"06", b"01".to_vec()),
                ModeCmd::HueSat => (b"06", b"02".to_vec()),
                ModeCmd::Scenes => (b"06", b"03".to_vec()),
            },
            GvmNodeCommand::Scene(scene) => match scene {
                SceneCmd::Lightning => (b"07", b"01".to_vec()),
                SceneCmd::CopCar => (b"07", b"02".to_vec()),
                SceneCmd::Candle => (b"07", b"03".to_vec()),
                SceneCmd::TV => (b"07", b"04".to_vec()),
                SceneCmd::BadBulb => (b"07", b"05".to_vec()),
                SceneCmd::Party => (b"07", b"06".to_vec()),
                SceneCmd::Disco => (b"07", b"07".to_vec()),
                SceneCmd::Paparazzi => (b"07", b"08".to_vec()),
            },
        };
        let mut c = cmd.to_vec();
        c.append(&mut b"01".to_vec());
        c.append(&mut param);
        c
    }
}
