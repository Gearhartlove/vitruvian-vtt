use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum VisionKind {
    #[serde(alias = "darkvision")]
    DarkVision,
    #[serde(alias = "low-light-vision")]
    LowLightVision,
    #[serde(alias = "normal")]
    Normal,
}

impl VisionKind {
    pub fn to_string(&self) -> String {
        match self {
            VisionKind::DarkVision => "dark vision".into(),
            VisionKind::LowLightVision => "low light vision".into(),
            VisionKind::Normal => "normal".into(),
        }
    }
}
