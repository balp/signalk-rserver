use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1RootFormat {
    pub version: String,
    #[serde(rename = "self")]
    pub self_: String,
    pub vessels: Option<HashMap<String, V1Vessel>>,
    pub sources: Option<V1Sources>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1Vessel {
    pub uuid: Option<String>,
    pub mmsi: Option<String>,
    pub name: Option<String>,
    pub navigation: Option<V1Navigation>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V1Navigation {
    pub speed_over_ground: Option<V1NumberValue>,
    pub course_over_ground_true: Option<V1NumberValue>,
    pub heading_magnetic: Option<V1NumberValue>,
    pub position: Option<V1PositionType>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1NumberValue {
    pub value: f64,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1PositionType {
    pub value: V1PositionValue,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1PositionValue {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1Sources {
    #[serde(rename = "_attr")]
    pub type_: Option<V1Attr>,
    #[serde(flatten)]
    pub fields: HashMap<String, V1Source>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1Source {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub properties: HashMap<String, V1SourceProperty>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1SourceProperty {
    // pub ais: V1SourceAIS,
    // pub n2k: V1SourceN2K,
    pub talker: Option<String>,
    pub sentences: Option<HashMap<String, String>>,
    #[serde(flatten)]
    pub extras: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct V1Attr {
    #[serde(rename = "_mode")]
    pub mode: Option<i64>,
    #[serde(rename = "_owner")]
    pub owner: Option<String>,
    #[serde(rename = "_group")]
    pub group: Option<String>,
}
