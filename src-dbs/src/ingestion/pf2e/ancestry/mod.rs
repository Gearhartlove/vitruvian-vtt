mod boost;
mod flaw;

use super::{
    core::{
        description::Description,
        item::Item,
        languages::{AdditionalLanguages, Languages},
        publication::Publication,
        rule::Rule,
        size::SizeKind,
        traits::Traits,
        vision::VisionKind,
    },
    Pf2eWorld,
};
use crate::ingestion::{Ingest, Named, Schema};
use boost::BoostValue;
use flaw::FlawValue;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    #[serde(rename = "additionalLanguages")]
    additional_languages: AdditionalLanguages,
    boosts: HashMap<String, BoostValue>,
    description: Description,
    flaws: HashMap<String, FlawValue>,
    hp: u16,
    items: HashMap<String, Item>,
    languages: Languages,
    publication: Publication,
    reach: u8,
    rules: Vec<Rule>,
    size: SizeKind,
    speed: u8,
    traits: Traits,
    vision: VisionKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ancestry {
    #[serde(rename = "_id")]
    id: String,
    img: String,
    pub name: String,
    system: System,
    #[serde(rename = "type")]
    data_type: String,
}

impl Ingest for Ancestry {
    type Parent = Pf2eWorld;

    fn path() -> PathBuf {
        PathBuf::from(Self::Parent::path()).join("packs/ancestries")
    }
}

impl Named for Ancestry {
    fn name() -> String {
        String::from("Ancestories")
    }
}

impl Schema for Ancestry {
    fn values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.name.clone().into(),                      // Name, string
            self.system.description.value.clone().into(),  // Description, string
            self.img.clone().into(),                       // ImagePath, string
            self.system.vision.to_string().clone().into(), // Vision, string
            self.system.size.to_string().clone().into(),   // Size, string
            self.system.hp.clone().into(),                 // Hp, integer
            self.system.reach.clone().into(),              // Reach, integer
            self.system.speed.clone().into(),              // Speed, integer
            // serde_json::to_string(&self.system.languages)  // Languages, blob
            //     .unwrap()
            //     .into(),
            // serde_json::to_string(&self.system.additional_languages) // AdditionalLanguages, blob
            //     .unwrap()
            //     .into(),
            // serde_json::to_string(&self.system.boosts) // Boosts, blob
            //     .unwrap()
            //     .into(),
            // serde_json::to_string(&self.system.flaws) // Flaws, blob
            //     .unwrap()
            //     .into(),
            // serde_json::to_string(&self.system.traits) // Traits, blob
            //     .unwrap()
            //     .into(),
            // 1.into() // TODO: understand where to submit ancestries, note this might be wrong
        ]
    }
}
