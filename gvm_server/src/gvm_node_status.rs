use serde::{Deserialize, Serialize};

// Wrap the basic types into structs for type information
pub mod units {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct PowerState {
        pub value: bool,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct Brightness {
        pub value: u8,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct Temperature {
        pub value: u16,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct Hue {
        pub value: u16,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct Saturation {
        pub value: u8,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub struct RGB {
        pub value: u8,
    }
}

use units::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct GvmNodeStatus {
    pub power_state: PowerState,
    pub brightness: Brightness,
    pub temperature: Temperature,
    pub hue: Hue,
    pub saturation: Saturation,
    pub rgb: RGB,
}

impl GvmNodeStatus {
    pub fn new() -> GvmNodeStatus {
        GvmNodeStatus {
            power_state: PowerState { value: false },
            brightness: Brightness { value: 0 },
            temperature: Temperature { value: 0 },
            hue: Hue { value: 0 },
            saturation: Saturation { value: 0 },
            rgb: RGB { value: 0 },
        }
    }
}
