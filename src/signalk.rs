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
    pub environment: Option<V1Environment>,
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
pub struct V1Environment {
    pub outside: Option<V1EnvironmentOutside>,
    pub inside: Option<V1EnvironmentInside>,
    pub water: Option<V1EnvironmentWater>,
    pub depth: Option<V1EnvironmentDepth>,
    pub current: Option<V1EnvironmentCurrent>,
    pub tide: Option<V1EnvironmentTide>,
    pub heave: Option<V1NumberValue>,
    pub wind: Option<V1EnvironmentWind>,
    pub time: Option<V1EnvironmentTime>,
    pub mode: Option<V1EnvironmentMode>,
}

impl V1Environment {
    pub fn builder() -> V1EnvironmentBuilder {
        V1EnvironmentBuilder::default()
    }
}

#[derive(Default)]
pub struct V1EnvironmentBuilder {
    outside: Option<V1EnvironmentOutside>,
    inside: Option<V1EnvironmentInside>,
    water: Option<V1EnvironmentWater>,
    depth: Option<V1EnvironmentDepth>,
    current: Option<V1EnvironmentCurrent>,
    tide: Option<V1EnvironmentTide>,
    heave: Option<V1NumberValue>,
    wind: Option<V1EnvironmentWind>,
    time: Option<V1EnvironmentTime>,
    mode: Option<V1EnvironmentMode>,
}

impl V1EnvironmentBuilder {
    pub fn outside(mut self, label: V1EnvironmentOutside) -> V1EnvironmentBuilder {
        self.outside = Some(label);
        self
    }
    pub fn inside(mut self, label: V1EnvironmentInside) -> V1EnvironmentBuilder {
        self.inside = Some(label);
        self
    }
    pub fn water(mut self, label: V1EnvironmentWater) -> V1EnvironmentBuilder {
        self.water = Some(label);
        self
    }
    pub fn depth(mut self, label: V1EnvironmentDepth) -> V1EnvironmentBuilder {
        self.depth = Some(label);
        self
    }
    pub fn current(mut self, label: V1EnvironmentCurrent) -> V1EnvironmentBuilder {
        self.current = Some(label);
        self
    }
    pub fn tide(mut self, label: V1EnvironmentTide) -> V1EnvironmentBuilder {
        self.tide = Some(label);
        self
    }
    pub fn heave(mut self, label: V1NumberValue) -> V1EnvironmentBuilder {
        self.heave = Some(label);
        self
    }
    pub fn wind(mut self, label: V1EnvironmentWind) -> V1EnvironmentBuilder {
        self.wind = Some(label);
        self
    }
    pub fn time(mut self, label: V1EnvironmentTime) -> V1EnvironmentBuilder {
        self.time = Some(label);
        self
    }
    pub fn mode(mut self, label: V1EnvironmentMode) -> V1EnvironmentBuilder {
        self.mode = Some(label);
        self
    }
    pub fn build(self) -> V1Environment {
        V1Environment {
            outside: self.outside,
            inside: self.inside,
            water: self.water,
            depth: self.depth,
            current: self.current,
            tide: self.tide,
            heave: self.heave,
            wind: self.wind,
            time: self.time,
            mode: self.mode,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentOutside {
    pub temperature: Option<V1NumberValue>,
    pub dew_point_temperature: Option<V1NumberValue>,
    pub apparent_wind_chill_temperature: Option<V1NumberValue>,
    pub theoretical_wind_chill_temperature: Option<V1NumberValue>,
    pub heat_index_temperature: Option<V1NumberValue>,
    pub pressure: Option<V1NumberValue>,
    pub humidity: Option<V1NumberValue>,
    pub relative_humidity: Option<V1NumberValue>,
    pub air_density: Option<V1NumberValue>,
    pub illuminance: Option<V1NumberValue>,
}

impl V1EnvironmentOutside {
    pub fn builder() -> V1EnvironmentOutsideBuilder {
        V1EnvironmentOutsideBuilder::default()
    }
}

#[derive(Default)]
pub struct V1EnvironmentOutsideBuilder {
    temperature: Option<V1NumberValue>,
    dew_point_temperature: Option<V1NumberValue>,
    apparent_wind_chill_temperature: Option<V1NumberValue>,
    theoretical_wind_chill_temperature: Option<V1NumberValue>,
    heat_index_temperature: Option<V1NumberValue>,
    pressure: Option<V1NumberValue>,
    humidity: Option<V1NumberValue>,
    relative_humidity: Option<V1NumberValue>,
    air_density: Option<V1NumberValue>,
    illuminance: Option<V1NumberValue>,
}

impl V1EnvironmentOutsideBuilder {
    pub fn temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn dew_point_temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.dew_point_temperature = Some(value);
        self
    }
    pub fn apparent_wind_chill_temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.apparent_wind_chill_temperature = Some(value);
        self
    }
    pub fn theoretical_wind_chill_temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.theoretical_wind_chill_temperature = Some(value);
        self
    }
    pub fn heat_index_temperature(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.heat_index_temperature = Some(value);
        self
    }
    pub fn pressure(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.pressure = Some(value);
        self
    }
    pub fn humidity(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.humidity = Some(value);
        self
    }
    pub fn relative_humidity(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.relative_humidity = Some(value);
        self
    }
    pub fn air_density(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.air_density = Some(value);
        self
    }
    pub fn illuminance(mut self, value: V1NumberValue) -> V1EnvironmentOutsideBuilder {
        self.illuminance = Some(value);
        self
    }
    pub fn build(self) -> V1EnvironmentOutside {
        V1EnvironmentOutside {
            temperature: self.temperature,
            dew_point_temperature: self.dew_point_temperature,
            apparent_wind_chill_temperature: self.apparent_wind_chill_temperature,
            theoretical_wind_chill_temperature: self.theoretical_wind_chill_temperature,
            heat_index_temperature: self.heat_index_temperature,
            pressure: self.pressure,
            humidity: self.humidity,
            relative_humidity: self.relative_humidity,
            air_density: self.air_density,
            illuminance: self.illuminance,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1EnvironmentInside {
    #[serde(flatten)]
    pub zones: HashMap<String, V1EnvironmentZone>,
}

impl V1EnvironmentInside {
    pub fn builder() -> V1EnvironmentInsideBuilder {
        V1EnvironmentInsideBuilder::default()
    }
}

#[derive(Default)]
pub struct V1EnvironmentInsideBuilder {
    pub zones: HashMap<String, V1EnvironmentZone>,
}

impl V1EnvironmentInsideBuilder {
    pub fn add_zone(mut self, key: String, value: V1EnvironmentZone) -> V1EnvironmentInsideBuilder {
        self.zones.insert(key, value);
        self
    }
    pub fn build(self) -> V1EnvironmentInside {
        V1EnvironmentInside { zones: self.zones }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentZone {
    pub temperature: Option<V1NumberValue>,
    pub heat_index_temperature: Option<V1NumberValue>,
    pub pressure: Option<V1NumberValue>,
    pub relative_humidity: Option<V1NumberValue>,
    pub dew_point: Option<V1NumberValue>,
    pub dew_point_temperature: Option<V1NumberValue>,
    pub air_density: Option<V1NumberValue>,
    pub illuminance: Option<V1NumberValue>,
}

impl V1EnvironmentZone {
    pub fn builder() -> V1EnvironmentZoneBuilder {
        V1EnvironmentZoneBuilder::default()
    }
}

#[derive(Default)]
pub struct V1EnvironmentZoneBuilder {
    temperature: Option<V1NumberValue>,
    heat_index_temperature: Option<V1NumberValue>,
    pressure: Option<V1NumberValue>,
    relative_humidity: Option<V1NumberValue>,
    dew_point: Option<V1NumberValue>,
    dew_point_temperature: Option<V1NumberValue>,
    air_density: Option<V1NumberValue>,
    illuminance: Option<V1NumberValue>,
}

impl V1EnvironmentZoneBuilder {
    pub fn temperature(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn heat_index_temperature(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.heat_index_temperature = Some(value);
        self
    }
    pub fn pressure(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.pressure = Some(value);
        self
    }
    pub fn relative_humidity(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.relative_humidity = Some(value);
        self
    }
    pub fn dew_point(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.dew_point = Some(value);
        self
    }
    pub fn dew_point_temperature(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.dew_point_temperature = Some(value);
        self
    }
    pub fn air_density(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.air_density = Some(value);
        self
    }
    pub fn illuminance(mut self, value: V1NumberValue) -> V1EnvironmentZoneBuilder {
        self.illuminance = Some(value);
        self
    }
    pub fn build(self) -> V1EnvironmentZone {
        V1EnvironmentZone {
            temperature: self.temperature,
            heat_index_temperature: self.heat_index_temperature,
            pressure: self.pressure,
            relative_humidity: self.relative_humidity,
            dew_point: self.dew_point,
            dew_point_temperature: self.dew_point_temperature,
            air_density: self.air_density,
            illuminance: self.illuminance,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentWater {
    pub temperature: Option<V1NumberValue>,
    pub salinity: Option<V1NumberValue>,
}

impl V1EnvironmentWater {
    pub fn new(temperature: Option<V1NumberValue>, salinity: Option<V1NumberValue>) -> Self {
        Self { temperature, salinity }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentDepth {
    pub below_keel: Option<V1NumberValue>,
    pub below_transducer: Option<V1NumberValue>,
    pub below_surface: Option<V1NumberValue>,
    pub transducer_to_keel: Option<V1NumberValue>,
    pub surface_to_transducer: Option<V1NumberValue>,
}

impl V1EnvironmentDepth {
    pub fn builder() -> V1EnvironmentDepthBuilder {
        V1EnvironmentDepthBuilder::default()
    }
}

#[derive(Default)]
pub struct V1EnvironmentDepthBuilder {
    below_keel: Option<V1NumberValue>,
    below_transducer: Option<V1NumberValue>,
    below_surface: Option<V1NumberValue>,
    transducer_to_keel: Option<V1NumberValue>,
    surface_to_transducer: Option<V1NumberValue>,
}

impl V1EnvironmentDepthBuilder {
    pub fn below_keel(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.below_keel = Some(value);
        self
    }
    pub fn below_transducer(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.below_transducer = Some(value);
        self
    }
    pub fn below_surface(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.below_surface = Some(value);
        self
    }
    pub fn transducer_to_keel(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.transducer_to_keel = Some(value);
        self
    }
    pub fn surface_to_transducer(mut self, value: V1NumberValue) -> V1EnvironmentDepthBuilder {
        self.surface_to_transducer = Some(value);
        self
    }
    pub fn build(self) -> V1EnvironmentDepth {
        V1EnvironmentDepth {
            below_keel: self.below_keel,
            below_transducer: self.below_transducer,
            below_surface: self.below_surface,
            transducer_to_keel: self.transducer_to_keel,
            surface_to_transducer: self.surface_to_transducer,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentCurrent {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<V1EnvironmentCurrentValue>,
    pub values: Option<HashMap<String, V1EnvironmentCurrentType>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentCurrentType {
    pub timestamp: Option<String>,
    pub pgn: Option<f64>,
    pub sentence: Option<String>,
    pub value: Option<V1EnvironmentCurrentValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentCurrentValue {
    pub drift: Option<f64>,
    pub set_true: Option<f64>,
    pub set_magnetic: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentTide {
    pub height_high: Option<V1NumberValue>,
    pub height_now: Option<V1NumberValue>,
    pub height_low: Option<V1NumberValue>,
    pub time_low: Option<String>,
    pub time_high: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentWind {
    pub angle_apparent: Option<V1NumberValue>,
    pub angle_true_ground: Option<V1NumberValue>,
    pub angle_true_water: Option<V1NumberValue>,
    pub direction_change_alarm: Option<V1NumberValue>,
    pub direction_true: Option<V1NumberValue>,
    pub direction_magnetic: Option<V1NumberValue>,
    pub speed_true: Option<V1NumberValue>,
    pub speed_over_ground: Option<V1NumberValue>,
    pub speed_apparent: Option<V1NumberValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentTime {
    pub millis: Option<i64>,
    pub timezone_offset: Option<i64>,
    pub timezone_region: Option<String>,
    pub timestamp: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1EnvironmentMode {
    pub value: Option<String>,
    pub timestamp: Option<String>,
    pub source: Option<String>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Electrical {
    pub batteries: Option<HashMap<String, V1Battery>>,
    pub inverters: Option<HashMap<String, V1Inverter>>,
    pub chargers: Option<HashMap<String, V1Charger>>,
    pub alternators: Option<HashMap<String, V1Alternator>>,
    pub solar: Option<HashMap<String, V1Solar>>,
    pub ac: Option<HashMap<String, V1ACBus>>,
}

impl V1Electrical {
    pub fn builder() -> V1ElectricalBuilder {
        V1ElectricalBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalBuilder {
    batteries: Option<HashMap<String, V1Battery>>,
    inverters: Option<HashMap<String, V1Inverter>>,
    chargers: Option<HashMap<String, V1Charger>>,
    alternators: Option<HashMap<String, V1Alternator>>,
    solar: Option<HashMap<String, V1Solar>>,
    ac: Option<HashMap<String, V1ACBus>>,
}

impl V1ElectricalBuilder {
    pub fn add_battery(mut self, key: String, value: V1Battery) -> V1ElectricalBuilder {
        if self.batteries.is_none() {
            self.batteries = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.batteries {
            x.insert(key, value);
        }
        self
    }
    pub fn add_inverter(mut self, key: String, value: V1Inverter) -> V1ElectricalBuilder {
        if self.inverters.is_none() {
            self.inverters = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.inverters {
            x.insert(key, value);
        }
        self
    }
    pub fn add_charger(mut self, key: String, value: V1Charger) -> V1ElectricalBuilder {
        if self.chargers.is_none() {
            self.chargers = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.chargers {
            x.insert(key, value);
        }
        self
    }
    pub fn add_alternator(mut self, key: String, value: V1Alternator) -> V1ElectricalBuilder {
        if self.alternators.is_none() {
            self.alternators = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.alternators {
            x.insert(key, value);
        }
        self
    }
    pub fn add_solar(mut self, key: String, value: V1Solar) -> V1ElectricalBuilder {
        if self.solar.is_none() {
            self.solar = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.solar {
            x.insert(key, value);
        }
        self
    }
    pub fn add_ac(mut self, key: String, value: V1ACBus) -> V1ElectricalBuilder {
        if self.ac.is_none() {
            self.ac = Some(HashMap::new());
        }
        if let Some(ref mut x) = self.ac {
            x.insert(key, value);
        }
        self
    }

    pub fn build(self) -> V1Electrical {
        V1Electrical {
            batteries: self.batteries,
            inverters: self.inverters,
            chargers: self.chargers,
            alternators: self.alternators,
            solar: self.solar,
            ac: self.ac,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalIdentity {
    pub name: Option<String>,
    pub location: Option<String>,
    pub date_installed: Option<String>,
    pub manufacturer: Option<V1ElectricalManufacturer>,
}

impl V1ElectricalIdentity {
    pub fn builder() -> V1ElectricalIdentityBuilder {
        V1ElectricalIdentityBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalIdentityBuilder {
    name: Option<String>,
    location: Option<String>,
    date_installed: Option<String>,
    manufacturer: Option<V1ElectricalManufacturer>,
}

impl V1ElectricalIdentityBuilder {
    pub fn name(mut self, value: String) -> V1ElectricalIdentityBuilder {
        self.name = Some(value);
        self
    }
    pub fn location(mut self, value: String) -> V1ElectricalIdentityBuilder {
        self.location = Some(value);
        self
    }
    pub fn date_installed(mut self, value: String) -> V1ElectricalIdentityBuilder {
        self.date_installed = Some(value);
        self
    }
    pub fn manufacturer(mut self, value: V1ElectricalManufacturer) -> V1ElectricalIdentityBuilder {
        self.manufacturer = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalIdentity {
        V1ElectricalIdentity {
            name: self.name,
            location: self.location,
            date_installed: self.date_installed,
            manufacturer: self.manufacturer,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1ElectricalManufacturer {
    pub name: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
}

impl V1ElectricalManufacturer {
    pub fn builder() -> V1ElectricalManufacturerBuilder {
        V1ElectricalManufacturerBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalManufacturerBuilder {
    name: Option<String>,
    model: Option<String>,
    url: Option<String>,
}

impl V1ElectricalManufacturerBuilder {
    pub fn name(mut self, value: String) -> V1ElectricalManufacturerBuilder {
        self.name = Some(value);
        self
    }
    pub fn model(mut self, value: String) -> V1ElectricalManufacturerBuilder {
        self.model = Some(value);
        self
    }
    pub fn url(mut self, value: String) -> V1ElectricalManufacturerBuilder {
        self.url = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalManufacturer {
        V1ElectricalManufacturer {
            name: self.name,
            model: self.model,
            url: self.url,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCQualities {
    pub associated_bus: Option<String>,
    pub voltage: Option<V1ElectricalDCVoltageValue>,
    pub current: Option<V1ElectricalDCCurrentValue>,
    pub temperature: Option<V1ElectricalDCTemperatureValue>,
}

impl V1ElectricalDCQualities {
    pub fn builder() -> V1ElectricalDCQualitiesBuilder {
        V1ElectricalDCQualitiesBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCQualitiesBuilder {
    associated_bus: Option<String>,
    voltage: Option<V1ElectricalDCVoltageValue>,
    current: Option<V1ElectricalDCCurrentValue>,
    temperature: Option<V1ElectricalDCTemperatureValue>,
}

impl V1ElectricalDCQualitiesBuilder {
    pub fn associated_bus(mut self, value: String) -> V1ElectricalDCQualitiesBuilder {
        self.associated_bus = Some(value);
        self
    }
    pub fn voltage(mut self, value: V1ElectricalDCVoltageValue) -> V1ElectricalDCQualitiesBuilder {
        self.voltage = Some(value);
        self
    }
    pub fn current(mut self, value: V1ElectricalDCCurrentValue) -> V1ElectricalDCQualitiesBuilder {
        self.current = Some(value);
        self
    }
    pub fn temperature(
        mut self,
        value: V1ElectricalDCTemperatureValue,
    ) -> V1ElectricalDCQualitiesBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCQualities {
        V1ElectricalDCQualities {
            associated_bus: self.associated_bus,
            voltage: self.voltage,
            current: self.current,
            temperature: self.temperature,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCVoltageValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub ripple: Option<V1NumberValue>,
    pub meta: Option<V1ElectricalDCVoltageMeta>,
}

impl V1ElectricalDCVoltageValue {
    pub fn builder() -> V1ElectricalDCVoltageValueBuilder {
        V1ElectricalDCVoltageValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCVoltageValueBuilder {
    value: Option<V1NumberValue>,
    ripple: Option<V1NumberValue>,
    meta: Option<V1ElectricalDCVoltageMeta>,
}

impl V1ElectricalDCVoltageValueBuilder {
    pub fn value(mut self, value: V1NumberValue) -> V1ElectricalDCVoltageValueBuilder {
        self.value = Some(value);
        self
    }
    pub fn ripple(mut self, value: V1NumberValue) -> V1ElectricalDCVoltageValueBuilder {
        self.ripple = Some(value);
        self
    }
    pub fn meta(mut self, value: V1ElectricalDCVoltageMeta) -> V1ElectricalDCVoltageValueBuilder {
        self.meta = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCVoltageValue {
        V1ElectricalDCVoltageValue {
            value: self.value,
            ripple: self.ripple,
            meta: self.meta,
        }
    }
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

impl V1ElectricalDCVoltageMeta {
    pub fn builder() -> V1ElectricalDCVoltageMetaBuilder {
        V1ElectricalDCVoltageMetaBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCVoltageMetaBuilder {
    nominal: Option<f64>,
    warn_upper: Option<f64>,
    warn_lower: Option<f64>,
    fault_upper: Option<f64>,
    fault_lower: Option<f64>,
}

impl V1ElectricalDCVoltageMetaBuilder {
    pub fn nominal(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.nominal = Some(value);
        self
    }
    pub fn warn_upper(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.warn_upper = Some(value);
        self
    }
    pub fn warn_lower(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.warn_lower = Some(value);
        self
    }
    pub fn fault_upper(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.fault_upper = Some(value);
        self
    }
    pub fn fault_lower(mut self, value: f64) -> V1ElectricalDCVoltageMetaBuilder {
        self.fault_lower = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCVoltageMeta {
        V1ElectricalDCVoltageMeta {
            nominal: self.nominal,
            warn_upper: self.warn_upper,
            warn_lower: self.warn_lower,
            fault_upper: self.fault_upper,
            fault_lower: self.fault_lower,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalDCCurrentValue {
    #[serde(flatten)]
    pub value: Option<V1NumberValue>,
    pub meta: Option<V1ElectricalDCCurrentMeta>,
}

impl V1ElectricalDCCurrentValue {
    pub fn builder() -> V1ElectricalDCCurrentValueBuilder {
        V1ElectricalDCCurrentValueBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalDCCurrentValueBuilder {
    value: Option<V1NumberValue>,
    meta: Option<V1ElectricalDCCurrentMeta>,
}

impl V1ElectricalDCCurrentValueBuilder {
    pub fn value(mut self, value: V1NumberValue) -> V1ElectricalDCCurrentValueBuilder {
        self.value = Some(value);
        self
    }
    pub fn meta(mut self, value: V1ElectricalDCCurrentMeta) -> V1ElectricalDCCurrentValueBuilder {
        self.meta = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalDCCurrentValue {
        V1ElectricalDCCurrentValue {
            value: self.value,
            meta: self.meta,
        }
    }
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

impl V1ElectricalDCCurrentMeta {
    pub fn new(
        warn_upper: Option<f64>,
        warn_lower: Option<f64>,
        fault_upper: Option<f64>,
        fault_lower: Option<f64>,
    ) -> Self {
        Self {
            warn_upper,
            warn_lower,
            fault_upper,
            fault_lower,
        }
    }
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

impl V1ElectricalDCTemperatureValue {
    pub fn new(
        value: Option<V1NumberValue>,
        warn_upper: Option<f64>,
        warn_lower: Option<f64>,
        fault_upper: Option<f64>,
        fault_lower: Option<f64>,
    ) -> Self {
        Self {
            value,
            warn_upper,
            warn_lower,
            fault_upper,
            fault_lower,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ElectricalACQualities {
    pub associated_bus: Option<String>,
    pub line_neutral_voltage: Option<V1NumberValue>,
    pub line_line_voltage: Option<V1NumberValue>,
    pub current: Option<V1NumberValue>,
    pub frequency: Option<V1NumberValue>,
    pub reactive_power: Option<V1NumberValue>,
    pub power_factor: Option<V1NumberValue>,
    pub power_factor_lagging: Option<String>,
    pub real_power: Option<V1NumberValue>,
    pub apparent_power: Option<V1NumberValue>,
}

impl V1ElectricalACQualities {
    pub fn builder() -> V1ElectricalACQualitiesBuilder {
        V1ElectricalACQualitiesBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ElectricalACQualitiesBuilder {
    associated_bus: Option<String>,
    line_neutral_voltage: Option<V1NumberValue>,
    line_line_voltage: Option<V1NumberValue>,
    current: Option<V1NumberValue>,
    frequency: Option<V1NumberValue>,
    reactive_power: Option<V1NumberValue>,
    power_factor: Option<V1NumberValue>,
    power_factor_lagging: Option<String>,
    real_power: Option<V1NumberValue>,
    apparent_power: Option<V1NumberValue>,
}

impl V1ElectricalACQualitiesBuilder {
    pub fn associated_bus(mut self, value: String) -> V1ElectricalACQualitiesBuilder {
        self.associated_bus = Some(value);
        self
    }
    pub fn line_neutral_voltage(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.line_neutral_voltage = Some(value);
        self
    }
    pub fn line_line_voltage(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.line_line_voltage = Some(value);
        self
    }
    pub fn current(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.current = Some(value);
        self
    }
    pub fn frequency(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.frequency = Some(value);
        self
    }
    pub fn reactive_power(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.reactive_power = Some(value);
        self
    }
    pub fn power_factor(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.power_factor = Some(value);
        self
    }
    pub fn power_factor_lagging(mut self, value: String) -> V1ElectricalACQualitiesBuilder {
        self.power_factor_lagging = Some(value);
        self
    }
    pub fn real_power(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.real_power = Some(value);
        self
    }
    pub fn apparent_power(mut self, value: V1NumberValue) -> V1ElectricalACQualitiesBuilder {
        self.apparent_power = Some(value);
        self
    }
    pub fn build(self) -> V1ElectricalACQualities {
        V1ElectricalACQualities {
            associated_bus: self.associated_bus,
            line_neutral_voltage: self.line_neutral_voltage,
            line_line_voltage: self.line_line_voltage,
            current: self.current,
            frequency: self.frequency,
            reactive_power: self.reactive_power,
            power_factor: self.power_factor,
            power_factor_lagging: self.power_factor_lagging,
            real_power: self.real_power,
            apparent_power: self.apparent_power,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1BatteryTemperature {
    pub limit_discharge_lower: Option<f64>,
    pub limit_discharge_upper: Option<f64>,
    pub limit_recharge_lower: Option<f64>,
    pub limit_recharge_upper: Option<f64>,
}

impl V1BatteryTemperature {
    pub fn new(limit_discharge_lower: Option<f64>, limit_discharge_upper: Option<f64>, limit_recharge_lower: Option<f64>, limit_recharge_upper: Option<f64>) -> Self {
        Self { limit_discharge_lower, limit_discharge_upper, limit_recharge_lower, limit_recharge_upper }
    }
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

impl V1BatteryCapacity {
    pub fn builder() -> V1BatteryCapacityBuilder {
        V1BatteryCapacityBuilder::default()
    }
}

#[derive(Default)]
pub struct V1BatteryCapacityBuilder {
    nominal: Option<f64>,
    actual: Option<f64>,
    remaining: Option<f64>,
    discharge_limit: Option<f64>,
    state_of_charge: Option<f64>,
    state_of_health: Option<f64>,
    discharge_since_full: Option<f64>,
    time_remaining: Option<f64>,
}

impl V1BatteryCapacityBuilder {
    pub fn nominal(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.nominal = Some(value);
        self
    }
    pub fn actual(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.actual = Some(value);
        self
    }
    pub fn remaining(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.remaining = Some(value);
        self
    }
    pub fn discharge_limit(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.discharge_limit = Some(value);
        self
    }
    pub fn state_of_charge(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.state_of_charge = Some(value);
        self
    }
    pub fn state_of_health(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.state_of_health = Some(value);
        self
    }
    pub fn discharge_since_full(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.discharge_since_full = Some(value);
        self
    }
    pub fn time_remaining(mut self, value: f64) -> V1BatteryCapacityBuilder {
        self.time_remaining = Some(value);
        self
    }
    pub fn build(self) -> V1BatteryCapacity {
        V1BatteryCapacity {
            nominal: self.nominal,
            actual: self.actual,
            remaining: self.remaining,
            discharge_limit: self.discharge_limit,
            state_of_charge: self.state_of_charge,
            state_of_health: self.state_of_health,
            discharge_since_full: self.discharge_since_full,
            time_remaining: self.time_remaining,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
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

impl V1Battery {
    pub fn builder() -> V1BatteryBuilder {
        V1BatteryBuilder::default()
    }
}

#[derive(Default)]
pub struct V1BatteryBuilder {
    identity: Option<V1ElectricalIdentity>,
    dc_qualities: Option<V1ElectricalDCQualities>,
    chemistry: Option<String>,
    temperature: Option<V1BatteryTemperature>,
    capacity: Option<V1BatteryCapacity>,
    lifetime_discharge: Option<f64>,
    lifetime_recharge: Option<f64>,
}

impl V1BatteryBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1BatteryBuilder {
        self.identity = Some(value);
        self
    }
    pub fn dc_qualities(mut self, value: V1ElectricalDCQualities) -> V1BatteryBuilder {
        self.dc_qualities = Some(value);
        self
    }
    pub fn chemistry(mut self, value: String) -> V1BatteryBuilder {
        self.chemistry = Some(value);
        self
    }
    pub fn temperature(mut self, value: V1BatteryTemperature) -> V1BatteryBuilder {
        self.temperature = Some(value);
        self
    }
    pub fn capacity(mut self, value: V1BatteryCapacity) -> V1BatteryBuilder {
        self.capacity = Some(value);
        self
    }
    pub fn lifetime_discharge(mut self, value: f64) -> V1BatteryBuilder {
        self.lifetime_discharge = Some(value);
        self
    }
    pub fn lifetime_recharge(mut self, value: f64) -> V1BatteryBuilder {
        self.lifetime_recharge = Some(value);
        self
    }
    pub fn build(self) -> V1Battery {
        V1Battery {
            identity: self.identity,
            dc_qualities: self.dc_qualities,
            chemistry: self.chemistry,
            temperature: self.temperature,
            capacity: self.capacity,
            lifetime_discharge: self.lifetime_discharge,
            lifetime_recharge: self.lifetime_recharge,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1ElectricalInverterMode {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<String>,
}

impl V1ElectricalInverterMode {
    pub fn new(common: Option<V1CommonValueFields>, value: Option<String>) -> Self {
        Self { common, value }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1ElectricalChargerQualities {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Inverter {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    pub dc: Option<V1ElectricalDCQualities>,
    pub ac: Option<V1ElectricalACQualities>,
    pub inverter_mode: Option<V1ElectricalInverterMode>,
}

impl V1Inverter {
    pub fn new(identity: Option<V1ElectricalIdentity>, dc: Option<V1ElectricalDCQualities>, ac: Option<V1ElectricalACQualities>, inverter_mode: Option<V1ElectricalInverterMode>) -> Self {
        Self { identity, dc, ac, inverter_mode }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1Charger {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc: Option<V1ElectricalDCQualities>,
    #[serde(flatten)]
    pub charger: Option<V1ElectricalChargerQualities>,
}

impl V1Charger {
    pub fn new(identity: Option<V1ElectricalIdentity>, dc: Option<V1ElectricalDCQualities>, charger: Option<V1ElectricalChargerQualities>) -> Self {
        Self { identity, dc, charger }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Alternator {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc: Option<V1ElectricalDCQualities>,
    #[serde(flatten)]
    pub charger: Option<V1ElectricalChargerQualities>,
    pub revolutions: Option<f64>,
    pub pulley_ratio: Option<f64>,
    pub field_drive: Option<f64>,
    pub regulator_temperature: Option<f64>,
}

impl V1Alternator {
    pub fn builder() -> V1AlternatorBuilder {
        V1AlternatorBuilder::default()
    }
}

#[derive(Default)]
pub struct V1AlternatorBuilder {
    identity: Option<V1ElectricalIdentity>,
    dc: Option<V1ElectricalDCQualities>,
    charger: Option<V1ElectricalChargerQualities>,
    revolutions: Option<f64>,
    pulley_ratio: Option<f64>,
    field_drive: Option<f64>,
    regulator_temperature: Option<f64>,
}

impl V1AlternatorBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1AlternatorBuilder {
        self.identity = Some(value);
        self
    }
    pub fn dc(mut self, value: V1ElectricalDCQualities) -> V1AlternatorBuilder {
        self.dc = Some(value);
        self
    }
    pub fn charger(mut self, value: V1ElectricalChargerQualities) -> V1AlternatorBuilder {
        self.charger = Some(value);
        self
    }
    pub fn revolutions(mut self, value: f64) -> V1AlternatorBuilder {
        self.revolutions = Some(value);
        self
    }
    pub fn pulley_ratio(mut self, value: f64) -> V1AlternatorBuilder {
        self.pulley_ratio = Some(value);
        self
    }
    pub fn field_drive(mut self, value: f64) -> V1AlternatorBuilder {
        self.field_drive = Some(value);
        self
    }
    pub fn regulator_temperature(mut self, value: f64) -> V1AlternatorBuilder {
        self.regulator_temperature = Some(value);
        self
    }
    pub fn build(self) -> V1Alternator {
        V1Alternator {
            identity: self.identity,
            dc: self.dc,
            charger: self.charger,
            revolutions: self.revolutions,
            pulley_ratio: self.pulley_ratio,
            field_drive: self.field_drive,
            regulator_temperature: self.regulator_temperature,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct V1SolarLoad {
    #[serde(flatten)]
    pub common: Option<V1CommonValueFields>,
    pub value: Option<String>,
}

impl V1SolarLoad {
    pub fn new(common: Option<V1CommonValueFields>, value: Option<String>) -> Self {
        Self { common, value }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1Solar {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    #[serde(flatten)]
    pub dc: Option<V1ElectricalDCQualities>,
    #[serde(flatten)]
    pub charger: Option<V1ElectricalChargerQualities>,
    pub value: Option<String>,
    pub panel_voltage: Option<f64>,
    pub panel_current: Option<f64>,
    pub panel_temperature: Option<f64>,
    pub load: Option<V1SolarLoad>,
    pub load_current: Option<f64>,
}

impl V1Solar {
    pub fn builder() -> V1SolarBuilder {
        V1SolarBuilder::default()
    }
}

#[derive(Default)]
pub struct V1SolarBuilder {
    identity: Option<V1ElectricalIdentity>,
    dc: Option<V1ElectricalDCQualities>,
    charger: Option<V1ElectricalChargerQualities>,
    value: Option<String>,
    panel_voltage: Option<f64>,
    panel_current: Option<f64>,
    panel_temperature: Option<f64>,
    load: Option<V1SolarLoad>,
    load_current: Option<f64>,
}

impl V1SolarBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1SolarBuilder {
        self.identity = Some(value);
        self
    }
    pub fn dc(mut self, value: V1ElectricalDCQualities) -> V1SolarBuilder {
        self.dc = Some(value);
        self
    }
    pub fn charger(mut self, value: V1ElectricalChargerQualities) -> V1SolarBuilder {
        self.charger = Some(value);
        self
    }
    pub fn value(mut self, value: String) -> V1SolarBuilder {
        self.value = Some(value);
        self
    }
    pub fn panel_voltage(mut self, value: f64) -> V1SolarBuilder {
        self.panel_voltage = Some(value);
        self
    }
    pub fn panel_current(mut self, value: f64) -> V1SolarBuilder {
        self.panel_current = Some(value);
        self
    }
    pub fn panel_temperature(mut self, value: f64) -> V1SolarBuilder {
        self.panel_temperature = Some(value);
        self
    }
    pub fn load(mut self, value: V1SolarLoad) -> V1SolarBuilder {
        self.load = Some(value);
        self
    }
    pub fn load_current(mut self, value: f64) -> V1SolarBuilder {
        self.load_current = Some(value);
        self
    }
    pub fn build(self) -> V1Solar {
        V1Solar {
            identity: self.identity,
            dc: self.dc,
            charger: self.charger,
            value: self.value,
            panel_voltage: self.panel_voltage,
            panel_current: self.panel_current,
            panel_temperature: self.panel_temperature,
            load: self.load,
            load_current: self.load_current,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct V1ACBus {
    #[serde(flatten)]
    pub identity: Option<V1ElectricalIdentity>,
    pub phase: HashMap<String, V1ElectricalACQualities>,
}

impl V1ACBus {
    pub fn builder() -> V1ACBusBuilder {
        V1ACBusBuilder::default()
    }
}

#[derive(Default)]
pub struct V1ACBusBuilder {
    identity: Option<V1ElectricalIdentity>,
    phase: HashMap<String, V1ElectricalACQualities>,
}

impl V1ACBusBuilder {
    pub fn identity(mut self, value: V1ElectricalIdentity) -> V1ACBusBuilder {
        self.identity = Some(value);
        self
    }
    pub fn add_phase(mut self, key: String, value: V1ElectricalACQualities) -> V1ACBusBuilder {
        self.phase.insert(key, value);
        self
    }
    pub fn build(self) -> V1ACBus {
        V1ACBus {
            identity: self.identity,
            phase: self.phase,
        }
    }
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
