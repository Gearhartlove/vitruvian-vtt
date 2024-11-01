use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SizeKind {
    #[serde(alias = "tiny")]
    Tiny,
    #[serde(alias = "sm")]
    Small,
    #[serde(alias = "med")]
    Medium,
    #[serde(alias = "lg")]
    Large,
    Huge,
    Gargantuan,
}

impl SizeKind {

    pub fn to_string(&self) -> String {
        match self {
            SizeKind::Tiny => "tiny".into(),
            SizeKind::Small => "small".into(),
            SizeKind::Medium => "medium".into(),
            SizeKind::Large => "large".into(),
            SizeKind::Huge => "huge".into(),
            SizeKind::Gargantuan => "gargantuan".into(),
        }
    }
}

