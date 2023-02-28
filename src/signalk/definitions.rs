use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::signalk::{V1FullFormat, V1FullFormatBuilder, V1Sources, V1Vessel};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1CommonValueFields {
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    #[serde(rename = "_attr")]
    pub attr: Option<V1Attr>,
    pub meta: Option<V1Meta>,
    pub pgn: Option<i64>,
    pub sentence: Option<String>,
}

impl V1CommonValueFields {
    pub fn builder() -> V1CommonValueFieldsBuilder {
        V1CommonValueFieldsBuilder::default()
    }
}

#[derive(Default)]
pub struct V1CommonValueFieldsBuilder {
    timestamp: String,
    source: String,
    attr: Option<V1Attr>,
    meta: Option<V1Meta>,
    pgn: Option<i64>,
    sentence: Option<String>,
}

impl V1CommonValueFieldsBuilder {
    pub fn timestamp(mut self, value: String) -> V1CommonValueFieldsBuilder {
        self.timestamp = value;
        self
    }
    pub fn source(mut self, value: String) -> V1CommonValueFieldsBuilder {
        self.source = value;
        self
    }
    pub fn attr(mut self, value: V1Attr) -> V1CommonValueFieldsBuilder {
        self.attr = Some(value);
        self
    }
    pub fn meta(mut self, value: V1Meta) -> V1CommonValueFieldsBuilder {
        self.meta = Some(value);
        self
    }
    pub fn pgn(mut self, value: i64) -> V1CommonValueFieldsBuilder {
        self.pgn = Some(value);
        self
    }
    pub fn sentence(mut self, value: String) -> V1CommonValueFieldsBuilder {
        self.sentence = Some(value);
        self
    }
    pub fn build(self) -> V1CommonValueFields {
        V1CommonValueFields {
            timestamp: self.timestamp,
            source: self.source,
            attr: self.attr,
            meta: self.meta,
            pgn: self.pgn,
            sentence: self.sentence,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1NumberValue {
    pub value: f64,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
    // TODO: Add support for meta values
}

impl V1NumberValue {
    pub fn builder() -> V1NumberValueBuilder {
        V1NumberValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NumberValueBuilder {
    pub value: f64,
    pub timestamp: String,
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

impl V1NumberValueBuilder {
    pub fn value(mut self, value: f64) -> V1NumberValueBuilder {
        self.value = value;
        self
    }
    pub fn timestamp(mut self, timestamp: String) -> V1NumberValueBuilder {
        self.timestamp = timestamp;
        self
    }
    pub fn source(mut self, source: String) -> V1NumberValueBuilder {
        self.source = source;
        self
    }
    pub fn pgn(mut self, pgn: f64) -> V1NumberValueBuilder {
        self.pgn = Some(pgn);
        self
    }
    pub fn sentence(mut self, sentence: String) -> V1NumberValueBuilder {
        self.sentence = Some(sentence);
        self
    }
    pub fn build(self) -> V1NumberValue {
        V1NumberValue {
            value: self.value,
            timestamp: self.timestamp,
            source: self.source,
            pgn: self.pgn,
            sentence: self.sentence,
        }
    }
}

impl V1FullFormatBuilder {
    pub fn version(mut self, version: String) -> V1FullFormatBuilder {
        self.version = version;
        self
    }
    pub fn self_(mut self, self_: String) -> V1FullFormatBuilder {
        self.self_ = self_;
        self
    }
    pub fn add_vessel(mut self, key: String, vessel: V1Vessel) -> V1FullFormatBuilder {
        if self.vessels.is_none() {
            self.vessels = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.vessels {
            x.insert(key, vessel);
        }
        self
    }
    pub fn sources(mut self, sources: V1Sources) -> V1FullFormatBuilder {
        self.sources = Some(sources);
        self
    }
    pub fn build(self) -> V1FullFormat {
        V1FullFormat {
            version: self.version,
            self_: self.self_,
            vessels: self.vessels,
            sources: self.sources,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Meta {
    pub description: String,
    pub display_name: Option<String>,
    pub long_name: Option<String>,
    pub short_name: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<String>,
    // pub properties: Option<properties>,
    pub units: Option<String>,
    // pub display_scale: Option<V1DisplayScale>,
    pub timeout: Option<f64>,
    pub alert_method: Option<Vec<String>>,
    pub warn_method: Option<Vec<String>>,
    pub alarm_method: Option<Vec<String>>,
    pub emergency_method: Option<Vec<String>>,
    // pub zones: Option<Vec<V1Zones>>,
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

impl Default for V1Attr {
    fn default() -> Self {
        V1Attr {
            mode: Some(644),
            owner: Some("self".into()),
            group: Some("self".into()),
        }
    }
}

impl V1Attr {
    pub fn new(mode: i64, owner: String, group: String) -> V1Attr {
        V1Attr {
            mode: Some(mode),
            owner: Some(owner),
            group: Some(group),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1DefSource {
    pub label: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub src: Option<String>,
    pub can_name: Option<String>,
    pub pgn: Option<i32>,
    pub instance: Option<String>,
    pub sentence: Option<String>,
    pub talker: Option<String>,
    pub ais_type: Option<i32>,
}

impl V1DefSource {
    pub fn builder() -> V1DefSourceBuilder {
        V1DefSourceBuilder::default()
    }
}

#[derive(Default)]
pub struct V1DefSourceBuilder {
    label: String,
    type_: Option<String>,
    src: Option<String>,
    can_name: Option<String>,
    pgn: Option<i32>,
    instance: Option<String>,
    sentence: Option<String>,
    talker: Option<String>,
    ais_type: Option<i32>,
}

impl V1DefSourceBuilder {
    pub fn label(mut self, value: String) -> V1DefSourceBuilder {
        self.label = value;
        self
    }
    pub fn type_(mut self, value: String) -> V1DefSourceBuilder {
        self.type_ = Some(value);
        self
    }
    pub fn src(mut self, value: String) -> V1DefSourceBuilder {
        self.src = Some(value);
        self
    }
    pub fn can_name(mut self, value: String) -> V1DefSourceBuilder {
        self.can_name = Some(value);
        self
    }
    pub fn pgn(mut self, value: i32) -> V1DefSourceBuilder {
        self.pgn = Some(value);
        self
    }
    pub fn instance(mut self, value: String) -> V1DefSourceBuilder {
        self.instance = Some(value);
        self
    }
    pub fn sentence(mut self, value: String) -> V1DefSourceBuilder {
        self.sentence = Some(value);
        self
    }
    pub fn talker(mut self, value: String) -> V1DefSourceBuilder {
        self.talker = Some(value);
        self
    }
    pub fn ais_type(mut self, value: i32) -> V1DefSourceBuilder {
        self.ais_type = Some(value);
        self
    }
    pub fn build(self) -> V1DefSource {
        V1DefSource {
            label: self.label,
            type_: self.type_,
            src: self.src,
            can_name: self.can_name,
            pgn: self.pgn,
            instance: self.instance,
            sentence: self.sentence,
            talker: self.talker,
            ais_type: self.ais_type,
        }
    }
}
