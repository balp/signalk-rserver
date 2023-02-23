use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1RootFormat {
    pub version: String,
    #[serde(rename = "self")]
    pub self_: String,
    pub vessels: Option<HashMap<String, V1Vessel>>,
    pub sources: Option<V1Sources>,
}

impl V1RootFormat {
    pub fn builder() -> V1RootFormatBuilder {
        V1RootFormatBuilder::default()
    }
}

pub struct V1RootFormatBuilder {
    version: String,
    self_: String,
    vessels: Option<HashMap<String, V1Vessel>>,
    sources: Option<V1Sources>,
}

impl Default for V1RootFormatBuilder {
    fn default() -> Self {
        V1RootFormatBuilder {
            version: "1.7.0".to_string(),
            self_: "".to_string(),
            vessels: None,
            sources: None,
        }
    }
}

impl V1RootFormatBuilder {
    pub fn version(mut self, version: String) -> V1RootFormatBuilder {
        self.version = version;
        self
    }
    pub fn self_(mut self, self_: String) -> V1RootFormatBuilder {
        self.self_ = self_;
        self
    }
    pub fn add_vessel(mut self, key: String, vessel: V1Vessel) -> V1RootFormatBuilder {
        if self.vessels.is_none() {
            self.vessels = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.vessels {
            x.insert(key, vessel);
        }
        self
    }
    pub fn sources(mut self, sources: V1Sources) -> V1RootFormatBuilder {
        self.sources = Some(sources);
        self
    }
    pub fn build(self) -> V1RootFormat {
        V1RootFormat {
            version: self.version,
            self_: self.self_,
            vessels: self.vessels,
            sources: self.sources,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Vessel {
    pub uuid: Option<String>,
    pub mmsi: Option<String>,
    pub name: Option<String>,
    pub navigation: Option<V1Navigation>,
    pub propulsion: Option<HashMap<String, V1Propulsion>>,
}

impl V1Vessel {
    pub fn builder() -> V1VesselBuilder {
        V1VesselBuilder::default()
    }
}

#[derive(Default)]
pub struct V1VesselBuilder {
    uuid: Option<String>,
    mmsi: Option<String>,
    name: Option<String>,
    navigation: Option<V1Navigation>,
    propulsion: Option<HashMap<String, V1Propulsion>>,
}

impl V1VesselBuilder {
    pub fn uuid(mut self, uuid: String) -> V1VesselBuilder {
        self.uuid = Some(uuid);
        self
    }
    pub fn mmsi(mut self, mmsi: String) -> V1VesselBuilder {
        self.mmsi = Some(mmsi);
        self
    }
    pub fn name(mut self, name: String) -> V1VesselBuilder {
        self.name = Some(name);
        self
    }
    pub fn navigation(mut self, navigation: V1Navigation) -> V1VesselBuilder {
        self.navigation = Some(navigation);
        self
    }
    pub fn add_propulsion(mut self, key: String, propulsion: V1Propulsion) -> V1VesselBuilder {
        if self.propulsion.is_none() {
            self.propulsion = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.propulsion {
            x.insert(key, propulsion);
        }
        self
    }
    pub fn build(self) -> V1Vessel {
        V1Vessel {
            uuid: self.uuid,
            mmsi: self.mmsi,
            name: self.name,
            navigation: self.navigation,
            propulsion: self.propulsion,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Propulsion {
    pub label: String,
    pub state: Option<V1PropulsionState>,
    pub revolutions: Option<V1NumberValue>,
    pub temperature: Option<V1NumberValue>,
    pub oil_temperature: Option<V1NumberValue>,
    pub oil_pressure: Option<V1NumberValue>,
    pub alternator_voltage: Option<V1NumberValue>,
    pub run_time: Option<V1NumberValue>,
    pub coolant_temperature: Option<V1NumberValue>,
    pub coolant_pressure: Option<V1NumberValue>,
    pub boost_pressure: Option<V1NumberValue>,
    pub intake_manifold_temperature: Option<V1NumberValue>,
    pub engine_load: Option<V1NumberValue>,
    pub engine_torque: Option<V1NumberValue>,
}

impl V1Propulsion {
    pub fn builder() -> V1PropulsionBuilder {
        V1PropulsionBuilder::default()
    }
}

#[derive(Default)]
pub struct V1PropulsionBuilder {
    label: String,
    state: Option<V1PropulsionState>,
    revolutions: Option<V1NumberValue>,
    temperature: Option<V1NumberValue>,
    oil_temperature: Option<V1NumberValue>,
    oil_pressure: Option<V1NumberValue>,
    alternator_voltage: Option<V1NumberValue>,
    run_time: Option<V1NumberValue>,
    coolant_temperature: Option<V1NumberValue>,
    coolant_pressure: Option<V1NumberValue>,
    boost_pressure: Option<V1NumberValue>,
    intake_manifold_temperature: Option<V1NumberValue>,
    engine_load: Option<V1NumberValue>,
    engine_torque: Option<V1NumberValue>,
}

impl V1PropulsionBuilder {
    pub fn label(mut self, label: String) -> V1PropulsionBuilder {
        self.label = label;
        self
    }
    pub fn state(mut self, state: V1PropulsionState) -> V1PropulsionBuilder {
        self.state = Some(state);
        self
    }
    pub fn revolutions(mut self, revolutions: V1NumberValue) -> V1PropulsionBuilder {
        self.revolutions = Some(revolutions);
        self
    }
    pub fn temperature(mut self, temperature: V1NumberValue) -> V1PropulsionBuilder {
        self.temperature = Some(temperature);
        self
    }
    pub fn oil_temperature(mut self, oil_temperature: V1NumberValue) -> V1PropulsionBuilder {
        self.oil_temperature = Some(oil_temperature);
        self
    }
    pub fn oil_pressure(mut self, oil_pressure: V1NumberValue) -> V1PropulsionBuilder {
        self.oil_pressure = Some(oil_pressure);
        self
    }
    pub fn alternator_voltage(mut self, alternator_voltage: V1NumberValue) -> V1PropulsionBuilder {
        self.alternator_voltage = Some(alternator_voltage);
        self
    }
    pub fn run_time(mut self, run_time: V1NumberValue) -> V1PropulsionBuilder {
        self.run_time = Some(run_time);
        self
    }
    pub fn coolant_temperature(mut self, coolant_temperature: V1NumberValue) -> V1PropulsionBuilder {
        self.coolant_temperature = Some(coolant_temperature);
        self
    }
    pub fn coolant_pressure(mut self, coolant_pressure: V1NumberValue) -> V1PropulsionBuilder {
        self.coolant_pressure = Some(coolant_pressure);
        self
    }
    pub fn boost_pressure(mut self, boost_pressure: V1NumberValue) -> V1PropulsionBuilder {
        self.boost_pressure = Some(boost_pressure);
        self
    }
    pub fn intake_manifold_temperature(mut self, intake_manifold_temperature: V1NumberValue) -> V1PropulsionBuilder {
        self.intake_manifold_temperature = Some(intake_manifold_temperature);
        self
    }
    pub fn engine_load(mut self, engine_load: V1NumberValue) -> V1PropulsionBuilder {
        self.engine_load = Some(engine_load);
        self
    }
    pub fn engine_torque(mut self, engine_torque: V1NumberValue) -> V1PropulsionBuilder {
        self.engine_torque = Some(engine_torque);
        self
    }
    pub fn build(self) -> V1Propulsion {
        V1Propulsion {
            label: self.label,
            state: self.state,
            revolutions: self.revolutions,
            temperature: self.temperature,
            oil_temperature: self.oil_temperature,
            oil_pressure: self.oil_pressure,
            alternator_voltage: self.alternator_voltage,
            run_time: self.run_time,
            coolant_temperature: self.coolant_temperature,
            coolant_pressure: self.coolant_pressure,
            boost_pressure: self.boost_pressure,
            intake_manifold_temperature: self.intake_manifold_temperature,
            engine_load: self.engine_load,
            engine_torque: self.engine_torque,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum V1PropulsionState {
    Stopped,
    Started,
    #[default]
    Unusable,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Navigation {
    pub speed_over_ground: Option<V1NumberValue>,
    pub course_over_ground_true: Option<V1NumberValue>,
    pub heading_magnetic: Option<V1NumberValue>,
    pub position: Option<V1PositionType>,
}

impl V1Navigation {
    pub fn builder() -> V1NavigationBuilder {
        V1NavigationBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NavigationBuilder {
    pub speed_over_ground: Option<V1NumberValue>,
    pub course_over_ground_true: Option<V1NumberValue>,
    pub heading_magnetic: Option<V1NumberValue>,
    pub position: Option<V1PositionType>,
}

impl V1NavigationBuilder {
    pub fn speed_over_ground(mut self, speed_over_ground: V1NumberValue) -> V1NavigationBuilder {
        self.speed_over_ground = Some(speed_over_ground);
        self
    }
    pub fn course_over_ground_true(
        mut self,
        course_over_ground_true: V1NumberValue,
    ) -> V1NavigationBuilder {
        self.course_over_ground_true = Some(course_over_ground_true);
        self
    }
    pub fn heading_magnetic(mut self, heading_magnetic: V1NumberValue) -> V1NavigationBuilder {
        self.heading_magnetic = Some(heading_magnetic);
        self
    }
    pub fn position(mut self, position: V1PositionType) -> V1NavigationBuilder {
        self.position = Some(position);
        self
    }
    pub fn build(self) -> V1Navigation {
        V1Navigation {
            speed_over_ground: self.speed_over_ground,
            course_over_ground_true: self.course_over_ground_true,
            heading_magnetic: self.heading_magnetic,
            position: self.position,
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1PositionType {
    pub value: V1PositionValue,
    pub timestamp: String,
    #[serde(rename = "$source")]
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}
impl V1PositionType {
    pub fn builder() -> V1PositionTypeBuilder {
        V1PositionTypeBuilder::default()
    }
}

#[derive(Default)]
pub struct V1PositionTypeBuilder {
    pub value: V1PositionValue,
    pub timestamp: String,
    pub source: String,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
}

impl V1PositionTypeBuilder {
    pub fn value(mut self, value: V1PositionValue) -> V1PositionTypeBuilder {
        self.value = value;
        self
    }
    pub fn timestamp(mut self, timestamp: String) -> V1PositionTypeBuilder {
        self.timestamp = timestamp;
        self
    }
    pub fn source(mut self, source: String) -> V1PositionTypeBuilder {
        self.source = source;
        self
    }
    pub fn pgn(mut self, pgn: f64) -> V1PositionTypeBuilder {
        self.pgn = Some(pgn);
        self
    }
    pub fn sentence(mut self, sentence: String) -> V1PositionTypeBuilder {
        self.sentence = Some(sentence);
        self
    }
    pub fn build(self) -> V1PositionType {
        V1PositionType {
            value: self.value,
            timestamp: self.timestamp,
            source: self.source,
            pgn: self.pgn,
            sentence: self.sentence,
        }
    }
    
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1PositionValue {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

impl V1PositionValue {
    pub fn new_2d(latitude: f64, longitude: f64) -> V1PositionValue {
        V1PositionValue { latitude, longitude, altitude: None }
    }
    pub fn new_3d(latitude: f64, longitude: f64, altitude: f64) -> V1PositionValue {
        V1PositionValue { latitude, longitude, altitude: Some(altitude) }
    }

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Sources {
    #[serde(rename = "_attr")]
    pub type_: Option<V1Attr>,
    #[serde(flatten)]
    pub fields: HashMap<String, V1Source>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Source {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub properties: HashMap<String, V1SourceProperty>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1SourceProperty {
    // pub ais: V1SourceAIS,
    // pub n2k: V1SourceN2K,
    pub talker: Option<String>,
    pub sentences: Option<HashMap<String, String>>,
    #[serde(flatten)]
    pub extras: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Attr {
    #[serde(rename = "_mode")]
    pub mode: Option<i64>,
    #[serde(rename = "_owner")]
    pub owner: Option<String>,
    #[serde(rename = "_group")]
    pub group: Option<String>,
}
