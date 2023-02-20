use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct V1RootFormat {
    pub version: String,
    #[serde(rename = "self")]
    pub self_: String,
    pub vessels: Option<HashMap<String, V1Vessel>>,
    pub sources: Option<HashMap<String, V1Source>>
}


#[derive(Serialize, Deserialize)]
pub struct V1Vessel {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub navigation: Option<V1Navigation>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1Navigation {
    pub speed_over_ground: Option<V1NumberValue>,
    pub heading_magnetic: Option<V1NumberValue>,
    pub position: Option<V1PositionType>,
}

#[derive(Serialize, Deserialize)]
pub struct V1NumberValue {
    pub value: f64,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct V1PositionType {
    pub value: V1PositionValue,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct V1PositionValue {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct V1Source {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,

}
