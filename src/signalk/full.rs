use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::signalk::{V1Sources, V1Vessel};

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
