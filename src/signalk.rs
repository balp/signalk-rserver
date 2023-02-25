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
#[serde(rename_all = "camelCase")]
pub struct V1Vessel {
    pub mmsi: Option<String>,
    pub url: Option<String>,
    pub uuid: Option<String>,
    pub mothership_mmsi: Option<String>,
    pub name: Option<String>,
    pub port: Option<String>,
    pub flag: Option<String>,
    pub navigation: Option<V1Navigation>,
    // pub registrations: Option<HashMap<String, V1Registration>>,
    // pub communication: Option<V1Communication>,
    // pub environment: Option<V1Environment>,
    pub electrical: Option<V1Electrical>,
    pub notifications: Option<V1Notification>,
    // pub steering: Option<V1Steering>,
    // pub tanks: Option<V1Tanks>,
    // pub design: Option<V1Design>,
    // pub sails: Option<V1Sails>,
    // pub sensors: Option<V1Sensors>,
    // pub performance: Option<V1Performance>,
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
            electrical: self.electrical,
            notifications: self.notifications,
            propulsion: self.propulsion,
            url: self.url,
            mothership_mmsi: self.mothership_mmsi,
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
    // pub transmission: Option<V1Transmission>,
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
    pub fn coolant_temperature(
        mut self,
        coolant_temperature: V1NumberValue,
    ) -> V1PropulsionBuilder {
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
    pub fn intake_manifold_temperature(
        mut self,
        intake_manifold_temperature: V1NumberValue,
    ) -> V1PropulsionBuilder {
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
    // pub course: Option<V1Course>,
    // pub lights: Option<V1Lights>,
    pub course_over_ground_magnetic: Option<V1NumberValue>,
    pub course_over_ground_true: Option<V1NumberValue>,
    pub course_rhumbline: Option<V1NumberValue>,
    pub course_great_circle: Option<V1NumberValue>,
    // pub closest_approach: Option<V1ClosestApproach>,
    // pub racing: Option<V1Racing>,
    pub magnetic_variation: Option<V1NumberValue>,
    pub magnetic_variation_age_of_service: Option<V1NumberValue>,
    // pub destination: Option<V1Destination>,
    // pub gnss: Option<V1gnss>,
    pub heading_magnetic: Option<V1NumberValue>,
    pub magnetic_deviation: Option<V1NumberValue>,
    pub heading_compass: Option<V1NumberValue>,
    pub heading_true: Option<V1NumberValue>,
    pub position: Option<V1PositionType>,
    // pub attitude: Option<V1AttitudeType>,
    // pub maneuver: Option<V1Maneuver>,
    pub rate_of_turn: Option<V1NumberValue>,
    pub speed_over_ground: Option<V1NumberValue>,
    pub speed_through_water: Option<V1NumberValue>,
    pub speed_through_water_transverse: Option<V1NumberValue>,
    pub speed_through_water_longitudinal: Option<V1NumberValue>,
    pub leeway_angle: Option<V1NumberValue>,
    pub log: Option<V1NumberValue>,
    // pub trip: Option<V1Trip>,
    // pub state: Option<V1State>,
    // pub anchor: Option<V1Anchor>,
    // pub datetime: Option<V1Datetime>,
}

impl V1Navigation {
    pub fn builder() -> V1NavigationBuilder {
        V1NavigationBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NavigationBuilder {
    // pub course: Option<V1Course>,
    // pub lights: Option<V1Lights>,
    course_over_ground_magnetic: Option<V1NumberValue>,
    course_over_ground_true: Option<V1NumberValue>,
    course_rhumbline: Option<V1NumberValue>,
    course_great_circle: Option<V1NumberValue>,
    // pub closest_approach: Option<V1ClosestApproach>,
    // pub racing: Option<V1Racing>,
    magnetic_variation: Option<V1NumberValue>,
    magnetic_variation_age_of_service: Option<V1NumberValue>,
    // pub destination: Option<V1Destination>,
    // pub gnss: Option<V1gnss>,
    heading_magnetic: Option<V1NumberValue>,
    magnetic_deviation: Option<V1NumberValue>,
    heading_compass: Option<V1NumberValue>,
    heading_true: Option<V1NumberValue>,
    position: Option<V1PositionType>,
    // pub attitude: Option<V1AttitudeType>,
    // pub maneuver: Option<V1Maneuver>,
    rate_of_turn: Option<V1NumberValue>,
    speed_over_ground: Option<V1NumberValue>,
    speed_through_water: Option<V1NumberValue>,
    speed_through_water_transverse: Option<V1NumberValue>,
    speed_through_water_longitudinal: Option<V1NumberValue>,
    leeway_angle: Option<V1NumberValue>,
    log: Option<V1NumberValue>,
    // pub trip: Option<V1Trip>,
    // pub state: Option<V1State>,
    // pub anchor: Option<V1Anchor>,
    // pub datetime: Option<V1Datetime>,
}

impl V1NavigationBuilder {
    pub fn course_over_ground_magnetic(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.course_over_ground_magnetic = Some(value);
        self
    }
    pub fn course_over_ground_true(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.course_over_ground_true = Some(value);
        self
    }
    pub fn course_rhumbline(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.course_rhumbline = Some(value);
        self
    }
    pub fn course_great_circle(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.course_great_circle = Some(value);
        self
    }
    pub fn magnetic_variation(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.magnetic_variation = Some(value);
        self
    }
    pub fn magnetic_variation_age_of_service(
        mut self,
        value: V1NumberValue,
    ) -> V1NavigationBuilder {
        self.magnetic_variation_age_of_service = Some(value);
        self
    }
    pub fn heading_magnetic(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.heading_magnetic = Some(value);
        self
    }
    pub fn magnetic_deviation(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.magnetic_deviation = Some(value);
        self
    }
    pub fn heading_compass(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.heading_compass = Some(value);
        self
    }
    pub fn heading_true(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.heading_true = Some(value);
        self
    }
    pub fn position(mut self, position: V1PositionType) -> V1NavigationBuilder {
        self.position = Some(position);
        self
    }
    pub fn rate_of_turn(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.rate_of_turn = Some(value);
        self
    }
    pub fn speed_over_ground(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_over_ground = Some(value);
        self
    }
    pub fn speed_through_water(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_through_water = Some(value);
        self
    }
    pub fn speed_through_water_transverse(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_through_water_transverse = Some(value);
        self
    }
    pub fn speed_through_water_longitudinal(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.speed_through_water_longitudinal = Some(value);
        self
    }
    pub fn leeway_angle(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.leeway_angle = Some(value);
        self
    }
    pub fn log(mut self, value: V1NumberValue) -> V1NavigationBuilder {
        self.log = Some(value);
        self
    }
    pub fn build(self) -> V1Navigation {
        V1Navigation {
            course_over_ground_magnetic: self.course_over_ground_magnetic,
            speed_over_ground: self.speed_over_ground,
            speed_through_water: self.speed_through_water,
            speed_through_water_transverse: self.speed_through_water_transverse,
            speed_through_water_longitudinal: self.speed_through_water_longitudinal,
            leeway_angle: self.leeway_angle,
            course_over_ground_true: self.course_over_ground_true,
            course_rhumbline: self.course_rhumbline,
            course_great_circle: self.course_great_circle,
            magnetic_variation: self.magnetic_variation,
            magnetic_variation_age_of_service: self.magnetic_variation_age_of_service,
            heading_magnetic: self.heading_magnetic,
            magnetic_deviation: self.magnetic_deviation,
            heading_compass: self.heading_compass,
            heading_true: self.heading_true,
            position: self.position,
            rate_of_turn: self.rate_of_turn,
            log: self.log,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Notification {
    pub value: Option<V1NotificationValue>,
    #[serde(flatten)]
    pub common_value_fields: Option<V1CommonValueFields>,
    #[serde(flatten)]
    pub childs: HashMap<String, V1Notification>,
}

impl V1Notification {
    pub fn builder() -> V1NotificationBuilder {
        V1NotificationBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NotificationBuilder {
    value: Option<V1NotificationValue>,
    common_value_fields: Option<V1CommonValueFields>,
    childs: HashMap<String, V1Notification>,
}

impl V1NotificationBuilder {
    pub fn value(mut self, value: V1NotificationValue) -> V1NotificationBuilder {
        self.value = Some(value);
        self
    }
    pub fn common_value_fields(mut self, value: V1CommonValueFields) -> V1NotificationBuilder {
        self.common_value_fields = Some(value);
        self
    }
    pub fn add_child(mut self, key: String, value: V1Notification) -> V1NotificationBuilder {
        self.childs.insert(key, value);
        self
    }
    pub fn build(self) -> V1Notification {
        V1Notification {
            value: self.value,
            common_value_fields: self.common_value_fields,
            childs: self.childs,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1NotificationValue {
    pub method: Vec<String>,
    pub state: String,
    pub message: String,
}

impl V1NotificationValue {
    pub fn builder() -> V1NotificationValueBuilder {
        V1NotificationValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1NotificationValueBuilder {
    method: Vec<String>,
    state: String,
    message: String,
}

impl V1NotificationValueBuilder {
    pub fn method(mut self, value: String) -> V1NotificationValueBuilder {
        self.method.push(value);
        self
    }
    pub fn state(mut self, value: String) -> V1NotificationValueBuilder {
        self.state = value;
        self
    }
    pub fn message(mut self, value: String) -> V1NotificationValueBuilder {
        self.message = value;
        self
    }
    pub fn build(self) -> V1NotificationValue {
        V1NotificationValue {
            method: self.method,
            state: self.state,
            message: self.message,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Electrical {
    pub batteries: Option<HashMap<String, V1Battery>>,
    pub inverters: Option<HashMap<String, V1Inverter>>,
    // pub chargers: HashMap<String,V1Chargers>,
    // pub alternators: HashMap<String,V1Alternators>,
    // pub solar: HashMap<String,V1Solar>,
    // pub ac: HashMap<String,V1ACBuses>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalIdentity {
    pub name: Option<String>,
    pub location: Option<String>,
    pub date_installed: Option<String>,
    pub manufacturer: Option<V1ElectricalManufacturer>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1ElectricalManufacturer {
    pub name: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCQualities {
    pub associated_bus: Option<String>,
    pub voltage: Option<V1ElectricalDCVoltageValue>,
    pub current: Option<V1ElectricalDCCurrentValue>,
    pub temperature: Option<V1ElectricalDCTemperatureValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCVoltageValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub ripple: Option<V1NumberValue>,
    pub meta: Option<V1ElectricalDCVoltageMeta>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCVoltageMeta {
    pub nominal: Option<f64>,
    pub warn_upper: Option<f64>,
    pub warn_lower: Option<f64>,
    pub fault_upper: Option<f64>,
    pub fault_lower: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCCurrentValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub meta: Option<V1ElectricalDCCurrentMeta>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCCurrentMeta {
    #[serde(flatten)]
    pub warn_upper: Option<f64>,
    pub warn_lower: Option<f64>,
    pub fault_upper: Option<f64>,
    pub fault_lower: Option<f64>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCTemperatureValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub warn_upper: Option<f64>,
    pub warn_lower: Option<f64>,
    pub fault_upper: Option<f64>,
    pub fault_lower: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalACQualities {
    pub associated_bus: Option<String>,
    pub line_neutral_voltage: Option<f64>,
    pub line_line_voltage: Option<f64>,
    pub current: Option<f64>,
    pub frequency: Option<f64>,
    pub reactive_power: Option<f64>,
    pub power_factor: Option<f64>,
    pub power_factor_lagging: Option<String>,
    pub real_power: Option<f64>,
    pub apparent_power: Option<f64>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1BatteryTemperature {
    pub limit_discharge_lower: Option<f64>,
    pub limit_discharge_upper: Option<f64>,
    pub limit_recharge_lower: Option<f64>,
    pub limit_recharge_upper: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1BatteryCapacity {
    pub nominal: Option<f64>,
    pub actual: Option<f64>,
    pub remaining: Option<f64>,
    pub discharge_limit: Option<f64>,
    pub state_of_charge: Option<f64>,
    pub state_of_health: Option<f64>,
    pub discharge_since_full: Option<f64>,
    pub time_remaining: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Battery {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc_qualities: Option<V1ElectricalDCQualities>,
    pub chemistry: Option<String>,
    pub temperature: Option<V1BatteryTemperature>,
    pub capacity: Option<V1BatteryCapacity>,
    pub lifetime_discharge: Option<f64>,
    pub lifetime_recharge: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1ElectricalInverterMode {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<String>,

}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Inverter {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    pub dc: Option<V1ElectricalDCQualities>,
    pub ac: Option<V1ElectricalACQualities>,
    pub inverter_mode: Option<V1ElectricalInverterMode>,
}



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
        V1PositionValue {
            latitude,
            longitude,
            altitude: None,
        }
    }
    pub fn new_3d(latitude: f64, longitude: f64, altitude: f64) -> V1PositionValue {
        V1PositionValue {
            latitude,
            longitude,
            altitude: Some(altitude),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Sources {
    #[serde(rename = "_attr")]
    pub type_: Option<V1Attr>,
    #[serde(flatten)]
    pub fields: HashMap<String, V1Source>,
}

impl V1Sources {
    pub fn builder() -> V1SourcesBuilder {
        V1SourcesBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SourcesBuilder {
    type_: Option<V1Attr>,
    pub fields: HashMap<String, V1Source>,
}

impl V1SourcesBuilder {
    pub fn t(mut self, type_: V1Attr) -> V1SourcesBuilder {
        self.type_ = Some(type_);
        self
    }
    pub fn add_field(mut self, key: String, value: V1Source) -> V1SourcesBuilder {
        self.fields.insert(key, value);
        self
    }
    pub fn build(self) -> V1Sources {
        V1Sources {
            type_: self.type_,
            fields: self.fields,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Source {
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub properties: HashMap<String, V1SourceProperty>,
}

impl V1Source {
    pub fn builder() -> V1SourceBuilder {
        V1SourceBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SourceBuilder {
    label: Option<String>,
    type_: Option<String>,
    properties: HashMap<String, V1SourceProperty>,
}

impl V1SourceBuilder {
    pub fn label(mut self, label: String) -> V1SourceBuilder {
        self.label = Some(label);
        self
    }
    pub fn type_(mut self, type_: String) -> V1SourceBuilder {
        self.type_ = Some(type_);
        self
    }
    pub fn add_property(mut self, key: String, property: V1SourceProperty) -> V1SourceBuilder {
        self.properties.insert(key, property);
        self
    }
    pub fn build(self) -> V1Source {
        V1Source {
            label: self.label,
            type_: self.type_,
            properties: self.properties,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1SourceProperty {
    // pub ais: V1SourceAIS,
    // pub n2k: V1SourceN2K,
    pub talker: Option<String>,
    pub sentences: Option<HashMap<String, String>>,
    // TODO: Not optional?
    #[serde(flatten)]
    pub extras: HashMap<String, String>,
}

impl V1SourceProperty {
    pub fn builder() -> V1SourcePropertyBuilder {
        V1SourcePropertyBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SourcePropertyBuilder {
    talker: Option<String>,
    sentences: Option<HashMap<String, String>>,
    extras: HashMap<String, String>,
}

impl V1SourcePropertyBuilder {
    pub fn talker(mut self, talker: String) -> V1SourcePropertyBuilder {
        self.talker = Some(talker);
        self
    }
    pub fn add_sentence(mut self, key: String, sentence: String) -> V1SourcePropertyBuilder {
        if self.sentences.is_none() {
            self.sentences = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.sentences {
            x.insert(key, sentence);
        }
        self
    }
    pub fn add_extra(mut self, key: String, value: String) -> V1SourcePropertyBuilder {
        self.extras.insert(key, value);
        self
    }
    pub fn build(self) -> V1SourceProperty {
        V1SourceProperty {
            talker: self.talker,
            sentences: self.sentences,
            extras: self.extras,
        }
    }
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
