mod environment;
mod navigation;
mod notification;
mod propulsion;
mod vessel;
mod electrical;
mod definitions;
mod sources;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub use definitions::V1CommonValueFields;
pub use definitions::V1NumberValue;

// TODO: Look over if needed...
pub use environment::{
    V1Environment, V1EnvironmentCurrent, V1EnvironmentCurrentValue, V1EnvironmentDepth,
    V1EnvironmentInside, V1EnvironmentTime
};
pub use electrical::{V1ACBus, V1Electrical, V1ElectricalACQualities, V1ElectricalIdentity};
pub use navigation::{V1Navigation, V1PositionType, V1PositionValue};
pub use notification::{V1Notification, V1NotificationValue};
pub use propulsion::V1Propulsion;
pub use sources::{V1Source, V1SourceProperty, V1Sources};
pub use vessel::V1Vessel;
pub use definitions::{V1Attr, V1Meta};

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
