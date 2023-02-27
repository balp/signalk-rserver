use serde::{Deserialize, Serialize};

use crate::signalk::electrical::V1Electrical;
use crate::signalk::environment::V1Environment;
use crate::signalk::notification::V1Notification;
use crate::signalk::{V1Navigation, V1Propulsion};
use std::collections::HashMap;

/// An object describing an individual vessel. It should be an object in vessels,
/// named using MMSI or a UUID
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Vessel {
    /// MMSI number of the vessel, if available.
    pub mmsi: Option<String>,

    /// URL based identity of the vessel, if available.
    pub url: Option<String>,

    /// A unique Signal K flavoured maritime resource identifier, assigned by the server.
    pub uuid: Option<String>,

    /// MMSI number of the mothership of this vessel, if available.
    pub mothership_mmsi: Option<String>,

    /// The common name of the vessel
    pub name: Option<String>,

    /// The home port of the vessel
    pub port: Option<String>,

    /// The country of ship registration, or flag state of the vessel
    pub flag: Option<String>,

    /// Navigation data including Position, Course to next WP information, etc.
    pub navigation: Option<V1Navigation>,

    // pub registrations: Option<HashMap<String, V1Registration>>,
    // pub communication: Option<V1Communication>,
    /// Environmental data measured locally including Depth, Wind, Temp, etc.
    pub environment: Option<V1Environment>,

    /// Electrical data, each electrical device indentified by a unique name i.e. Battery_1
    pub electrical: Option<V1Electrical>,

    /// Notifications currently raised. Major categories have well-defined names, but the tree can be extended by any hierarchical structure
    pub notifications: Option<V1Notification>,

    // pub steering: Option<V1Steering>,
    // pub tanks: Option<V1Tanks>,
    // pub design: Option<V1Design>,
    // pub sails: Option<V1Sails>,
    // pub sensors: Option<V1Sensors>,
    // pub performance: Option<V1Performance>,
    /// Engine data, each engine identified by a unique name i.e. Port_Engine
    pub propulsion: Option<HashMap<String, V1Propulsion>>,
}

impl V1Vessel {
    pub fn builder() -> V1VesselBuilder {
        V1VesselBuilder::default()
    }
}

#[derive(Default)]
pub struct V1VesselBuilder {
    mmsi: Option<String>,
    url: Option<String>,
    uuid: Option<String>,
    mothership_mmsi: Option<String>,
    name: Option<String>,
    flag: Option<String>,
    port: Option<String>,
    navigation: Option<V1Navigation>,
    environment: Option<V1Environment>,
    electrical: Option<V1Electrical>,
    notifications: Option<V1Notification>,
    propulsion: Option<HashMap<String, V1Propulsion>>,
}

impl V1VesselBuilder {
    pub fn uuid(mut self, value: String) -> V1VesselBuilder {
        self.uuid = Some(value);
        self
    }
    pub fn url(mut self, value: String) -> V1VesselBuilder {
        self.url = Some(value);
        self
    }
    pub fn mmsi(mut self, value: String) -> V1VesselBuilder {
        self.mmsi = Some(value);
        self
    }
    pub fn mothership_mmsi(mut self, value: String) -> V1VesselBuilder {
        self.mothership_mmsi = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> V1VesselBuilder {
        self.name = Some(value);
        self
    }
    pub fn port(mut self, value: String) -> V1VesselBuilder {
        self.port = Some(value);
        self
    }
    pub fn flag(mut self, value: String) -> V1VesselBuilder {
        self.flag = Some(value);
        self
    }
    pub fn navigation(mut self, value: V1Navigation) -> V1VesselBuilder {
        self.navigation = Some(value);
        self
    }
    pub fn electrical(mut self, value: V1Electrical) -> V1VesselBuilder {
        self.electrical = Some(value);
        self
    }
    pub fn environment(mut self, value: V1Environment) -> V1VesselBuilder {
        self.environment = Some(value);
        self
    }
    pub fn notifications(mut self, value: V1Notification) -> V1VesselBuilder {
        self.notifications = Some(value);
        self
    }
    pub fn add_propulsion(mut self, key: String, value: V1Propulsion) -> V1VesselBuilder {
        if self.propulsion.is_none() {
            self.propulsion = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.propulsion {
            x.insert(key, value);
        }
        self
    }
    pub fn build(self) -> V1Vessel {
        V1Vessel {
            uuid: self.uuid,
            mmsi: self.mmsi,
            name: self.name,
            port: self.port,
            flag: self.flag,
            navigation: self.navigation,
            environment: self.environment,
            electrical: self.electrical,
            notifications: self.notifications,
            propulsion: self.propulsion,
            url: self.url,
            mothership_mmsi: self.mothership_mmsi,
        }
    }
}
