mod definitions;
mod electrical;
mod environment;
mod navigation;
mod notification;
mod propulsion;
mod sources;
mod vessel;

pub use definitions::V1CommonValueFields;
pub use definitions::V1NumberValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: Look over if needed...
pub use definitions::{V1Attr, V1Meta};
pub use electrical::{V1ACBus, V1Electrical, V1ElectricalACQualities, V1ElectricalIdentity};
pub use environment::{
    V1Environment, V1EnvironmentCurrent, V1EnvironmentCurrentValue, V1EnvironmentDepth,
    V1EnvironmentInside, V1EnvironmentTime,
};
pub use navigation::{V1Navigation, V1PositionType, V1PositionValue};
pub use notification::{V1Notification, V1NotificationValue};
pub use propulsion::V1Propulsion;
pub use sources::{V1Source, V1SourceProperty, V1Sources};
pub use vessel::V1Vessel;

/// Root structure for Full Signal K data
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1FullFormat {
    /// Version of the schema and APIs that this data is using in Canonical format i.e. V1.7.0.
    pub version: String,

    /// This holds the context (prefix + UUID, MMSI or URL in dot notation) of the server's self object.
    #[serde(rename = "self")]
    pub self_: String,

    /// A wrapper object for vessel objects, each describing vessels in range, including this vessel.
    pub vessels: Option<HashMap<String, V1Vessel>>,

    // TODO: Add aircraft
    // TODO: Add aton
    // TODO: Add sar
    /// Metadata about the data sources; physical interface, address, protocol, etc.
    pub sources: Option<V1Sources>,
}

impl V1FullFormat {
    /// Returns a builder for the Full Formal Signal K structure
    ///
    /// As the structure is a bit complex to create it's recommended to
    ///use this builder pattern to create new instances.
    pub fn builder() -> V1FullFormatBuilder {
        V1FullFormatBuilder::default()
    }
}

/// Builder for the Signal K Full format structure
pub struct V1FullFormatBuilder {
    version: String,
    self_: String,
    vessels: Option<HashMap<String, V1Vessel>>,
    sources: Option<V1Sources>,
}

impl Default for V1FullFormatBuilder {
    fn default() -> Self {
        V1FullFormatBuilder {
            version: "1.7.0".to_string(),
            self_: "".to_string(),
            vessels: None,
            sources: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Hello {
    pub name: Option<String>,
    pub version: String,
    pub timestamp: Option<String>,
    pub start_time: Option<String>,
    pub playback_rate: Option<f64>,
    #[serde(rename = "self")]
    pub self_: Option<String>,
    pub roles: Vec<String>,

}

impl V1Hello {
    pub fn builder() -> V1HelloBuilder {
        V1HelloBuilder::default()
    }
}

#[derive(Default)]
pub struct V1HelloBuilder {
    name: Option<String>,
    version: String,
    timestamp: Option<String>,
    start_time: Option<String>,
    playback_rate: Option<f64>,
    self_: Option<String>,
    roles: Vec<String>,
}

impl V1HelloBuilder {
    pub fn name(mut self, value: String) -> V1HelloBuilder {
        self.name = Some(value);
        self
    }
    pub fn version(mut self, value: String) -> V1HelloBuilder {
        self.version = value;
        self
    }
    pub fn timestamp(mut self, value: String) -> V1HelloBuilder {
        self.timestamp = Some(value);
        self
    }
    pub fn start_time(mut self, value: String) -> V1HelloBuilder {
        self.start_time = Some(value);
        self
    }
    pub fn playback_rate(mut self, value: f64) -> V1HelloBuilder {
        self.playback_rate = Some(value);
        self
    }
    pub fn self_(mut self, value: String) -> V1HelloBuilder {
        self.self_ = Some(value);
        self
    }
    pub fn role(mut self, value: String) -> V1HelloBuilder {
        self.roles.push(value);
        self
    }
    pub fn build(self) -> V1Hello {
        V1Hello {
            name: self.name,
            version: self.version,
            timestamp: self.timestamp,
            start_time: self.start_time,
            playback_rate: self.playback_rate,
            self_: self.self_,
            roles: self.roles,
        }
    }
}