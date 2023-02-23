use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use signalk_rserver::signalk::{
    V1Attr, V1Navigation, V1NumberValue, V1PositionType, V1PositionValue, V1Propulsion,
    V1RootFormat, V1Source, V1SourceProperty, V1Sources, V1Vessel,
};

trait OptionExt {
    type Value;
    fn unwrap_ref(&self) -> &Self::Value;
}

impl<T> OptionExt for Option<T> {
    type Value = T;
    fn unwrap_ref(&self) -> &T {
        self.as_ref().unwrap()
    }
}

#[test]
fn make_structure_for_full_doc_example() {
    let data = r#"
    {
      "version": "1.0.0",
      "self": "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c",
      "vessels": {
        "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c": {
          "navigation": {
            "speedOverGround": {
              "value": 4.32693662,
              "$source": "ttyUSB0.GP",
              "sentence": "RMC",
              "timestamp": "2017-05-16T05:15:50.007Z"
            },
            "position": {
              "value": {
                "altitude": 0.0,
                "latitude": 37.81479,
                "longitude": -122.44880152
              },
              "$source": "ttyUSB0.GP",
              "sentence": "RMC",
              "timestamp": "2017-05-16T05:15:50.007Z"
            },
            "headingMagnetic": {
              "value": 5.55014702,
              "$source": "ttyUSB0.II",
              "sentence": "HDM",
              "timestamp": "2017-05-16T05:15:54.006Z"
            }
          },
          "name": "Motu",
          "uuid": "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c"
        }
      },
      "sources": {
        "ttyUSB0": {
          "label": "ttyUSB0",
          "type": "NMEA0183",
          "GP": {
            "talker": "GP",
            "sentences": {
              "RMC": "2017-04-03T06:14:04.451Z"
            }
          },
          "II": {
            "talker": "II",
            "sentences": {
              "HDM": "2017-05-16T05:15:54.006Z"
            }
          }
        }
      }
    }
    "#;
    let sample_vessel_id = "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c";

    let sk_data: V1RootFormat = serde_json::from_str(data).unwrap();
    assert_eq!(sk_data.version, "1.0.0");
    assert_eq!(sk_data.self_, sample_vessel_id);
    assert!(sk_data.vessels.unwrap_ref().contains_key(sample_vessel_id));
    let sk_vessel = sk_data.vessels.unwrap_ref().get(sample_vessel_id).unwrap();
    assert_eq!(sk_vessel.uuid, Some(sample_vessel_id.into()));
    assert_eq!(sk_vessel.name, Some("Motu".into()));
    let navigation = sk_vessel.navigation.unwrap_ref();
    assert_eq!(navigation.speed_over_ground.unwrap_ref().value, 4.32693662);
    assert_eq!(
        navigation.speed_over_ground.unwrap_ref().timestamp,
        "2017-05-16T05:15:50.007Z"
    );
    assert_eq!(
        navigation.speed_over_ground.unwrap_ref().source,
        "ttyUSB0.GP"
    );
    assert_eq!(
        navigation
            .speed_over_ground
            .unwrap_ref()
            .sentence
            .unwrap_ref(),
        "RMC"
    );

    assert_eq!(navigation.heading_magnetic.unwrap_ref().value, 5.55014702);
    assert_eq!(
        navigation.heading_magnetic.unwrap_ref().timestamp,
        "2017-05-16T05:15:54.006Z"
    );
    assert_eq!(
        navigation.heading_magnetic.unwrap_ref().source,
        "ttyUSB0.II"
    );
    assert_eq!(
        navigation
            .heading_magnetic
            .unwrap_ref()
            .sentence
            .unwrap_ref(),
        "HDM"
    );

    assert_eq!(navigation.position.unwrap_ref().value.latitude, 37.81479);
    assert_eq!(
        navigation.position.unwrap_ref().value.longitude,
        -122.44880152
    );
    assert_eq!(
        navigation.position.unwrap_ref().value.altitude.unwrap(),
        0.0
    );
    assert_eq!(
        navigation.position.unwrap_ref().timestamp,
        "2017-05-16T05:15:50.007Z"
    );
    assert_eq!(navigation.position.unwrap_ref().source, "ttyUSB0.GP");
    assert_eq!(
        navigation.position.unwrap_ref().sentence.unwrap_ref(),
        "RMC"
    );

    let sources = sk_data.sources.unwrap_ref();
    assert!(sources.fields.contains_key("ttyUSB0".into()));
    let usb_source = sources.fields.get("ttyUSB0".into()).unwrap();
    assert_eq!(usb_source.label.unwrap_ref(), "ttyUSB0");
    assert_eq!(usb_source.type_.unwrap_ref(), "NMEA0183");
    let ii_source_property = usb_source.properties.get("II").unwrap();
    assert_eq!(ii_source_property.talker.unwrap_ref(), "II");
    assert_eq!(
        ii_source_property
            .sentences
            .unwrap_ref()
            .get("HDM")
            .unwrap(),
        "2017-05-16T05:15:54.006Z"
    );
    let gp_source_property = usb_source.properties.get("GP").unwrap();
    assert_eq!(gp_source_property.talker.unwrap_ref(), "GP");
    assert_eq!(
        gp_source_property
            .sentences
            .unwrap_ref()
            .get("RMC")
            .unwrap(),
        "2017-04-03T06:14:04.451Z"
    );
}

fn read_signalk_from_file(path: &str) -> V1RootFormat {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1RootFormat = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_sample_full_0183_rmc_export() {
    let expected = V1RootFormat {
        version: "1.0.0".into(),
        self_: "urn:mrn:imo:mmsi:366982330".into(),
        vessels: Some(HashMap::from([(
            "urn:mrn:imo:mmsi:366982330".into(),
            V1Vessel {
                uuid: None,
                mmsi: Some("366982330".into()),
                name: None,
                navigation: Some(V1Navigation {
                    speed_over_ground: None,
                    course_over_ground_true: Some(V1NumberValue {
                        value: 245.69,
                        timestamp: "2015-03-06T16:57:53.643Z".into(),
                        source: "sources.gps_0183_RMC".into(),
                        pgn: None,
                        sentence: None,
                    }),
                    heading_magnetic: None,
                    position: Some(V1PositionType {
                        value: V1PositionValue {
                            latitude: -41.156426,
                            longitude: 173.1693,
                            altitude: None,
                        },
                        timestamp: "2015-03-06T16:57:53.643Z".into(),
                        source: "sources.gps_0183_RMC".into(),
                        pgn: None,
                        sentence: None,
                    }),
                }),
                propulsion: None,
            },
        )])),
        sources: None,
    };

    let sk_data = read_signalk_from_file("tests/specification/examples/full/0183-RMC-export.json");

    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_full_example() {
    let expected = V1RootFormat {
        version: "1.0.0".into(),
        self_: "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into(),
        vessels: Some(HashMap::from([(
            "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into(),
            V1Vessel {
                uuid: Some("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into()),
                mmsi: None,
                name: Some("Motu".into()),
                navigation: Some(V1Navigation {
                    speed_over_ground: Some(V1NumberValue {
                        value: 4.32693662,
                        timestamp: "2017-05-16T05:15:50.007Z".into(),
                        source: "ttyUSB0.GP".into(),
                        pgn: None,
                        sentence: Some("RMC".into()),
                    }),
                    course_over_ground_true: None,
                    heading_magnetic: Some(V1NumberValue {
                        value: 5.55014702,
                        timestamp: "2017-05-16T05:15:54.006Z".into(),
                        source: "ttyUSB0.II".into(),
                        pgn: None,
                        sentence: Some("HDM".into()),
                    }),
                    position: Some(V1PositionType {
                        value: V1PositionValue {
                            latitude: 37.81479,
                            longitude: -122.44880152,
                            altitude: Some(0.0),
                        },
                        timestamp: "2017-05-16T05:15:50.007Z".into(),
                        source: "ttyUSB0.GP".into(),
                        pgn: None,
                        sentence: Some("RMC".into()),
                    }),
                }),
                propulsion: None,
            },
        )])),
        sources: Some(V1Sources {
            type_: None,
            fields: HashMap::from([(
                "ttyUSB0".into(),
                V1Source {
                    label: Some("ttyUSB0".into()),
                    type_: Some("NMEA0183".into()),
                    properties: HashMap::from([
                        (
                            "II".into(),
                            V1SourceProperty {
                                talker: Some("II".into()),
                                sentences: Some(HashMap::from([(
                                    "HDM".into(),
                                    "2017-05-16T05:15:54.006Z".into(),
                                )])),
                                extras: Default::default(),
                            },
                        ),
                        (
                            "GP".into(),
                            V1SourceProperty {
                                talker: Some("GP".into()),
                                sentences: Some(HashMap::from([(
                                    "RMC".into(),
                                    "2017-04-03T06:14:04.451Z".into(),
                                )])),
                                extras: Default::default(),
                            },
                        ),
                    ]),
                },
            )]),
        }),
    };

    let sk_data =
        read_signalk_from_file("tests/specification/examples/full/docs-full-example.json");

    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_full_0183_rmc_export_min() {
    let expected = V1RootFormat {
        version: "1.0.0".into(),
        self_: "urn:mrn:imo:mmsi:366982330".into(),
        vessels: Some(HashMap::from([(
            "urn:mrn:imo:mmsi:366982330".into(),
            V1Vessel {
                uuid: None,
                mmsi: Some("230099999".into()),
                name: None,
                navigation: Some(V1Navigation {
                    speed_over_ground: None,
                    course_over_ground_true: Some(V1NumberValue {
                        value: 245.69,
                        timestamp: "2015-01-25T12:01:01Z".into(),
                        source: "a.suitable.path".into(),
                        pgn: None,
                        sentence: None,
                    }),
                    heading_magnetic: None,
                    position: Some(V1PositionType {
                        value: V1PositionValue {
                            latitude: -41.156426,
                            longitude: 173.1693,
                            altitude: Some(0.0),
                        },
                        timestamp: "2015-01-25T12:01:01Z".into(),
                        source: "a.suitable.path".into(),
                        pgn: None,
                        sentence: None,
                    }),
                }),
                propulsion: None,
            },
        )])),
        sources: None,
    };

    let sk_data =
        read_signalk_from_file("tests/specification/examples/full/0183-RMC-export-min.json");

    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_full_0183_rmc_full() {
    let expected = V1RootFormat {
        version: "0.1.0".into(),
        self_: "urn:mrn:imo:mmsi:366982330".into(),
        vessels: Some(HashMap::from([(
            "urn:mrn:imo:mmsi:366982330".into(),
            V1Vessel {
                uuid: None,
                mmsi: Some("366982330".into()),
                name: None,
                navigation: Some(V1Navigation {
                    speed_over_ground: None,
                    course_over_ground_true: Some(V1NumberValue {
                        value: 245.69,
                        timestamp: "2015-03-06T16:57:53.643Z".into(),
                        source: "sources.gps_0183_RMC".into(),
                        pgn: None,
                        sentence: None,
                    }),
                    heading_magnetic: None,
                    position: Some(V1PositionType {
                        value: V1PositionValue {
                            latitude: -41.156426,
                            longitude: 173.1693,
                            altitude: Some(0.0),
                        },
                        timestamp: "2015-03-06T16:57:53.643Z".into(),
                        source: "sources.gps_0183_RMC".into(),
                        pgn: None,
                        sentence: None,
                    }),
                }),
                propulsion: None,
            },
        )])),
        sources: None,
    };
    let sk_data = read_signalk_from_file("tests/specification/examples/full/0183-RMC-full.json");
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_data_model() {
    let expected = V1RootFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into())
        .add_vessel("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into(),
                    V1Vessel::builder()
                        .uuid("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into())
                        .name("Motu".into())
                        .navigation(V1Navigation::builder()
                            .speed_over_ground(V1NumberValue::builder()
                                .value(4.32693662)
                                .timestamp("2017-05-16T05:15:50.007Z".into())
                                .source("ttyUSB0.GP".into())
                                .sentence("RMC".into())
                                .build())
                            .heading_magnetic(V1NumberValue::builder()
                                .value(5.55014702)
                                .timestamp("2017-05-16T05:15:54.006Z".into())
                                .source("ttyUSB0.II".into())
                                .sentence("HDM".into())
                                .build())
                            .position(V1PositionType::builder()
                                .value(V1PositionValue::new_3d(37.81479, -122.44880152, 0.0))
                                .timestamp("2017-05-16T05:15:50.007Z".into())
                                .source("ttyUSB0.GP".into())
                                .sentence("RMC".into())
                                .build())
                            .build()).build())
        .sources(V1Sources {
            type_: None,
            fields: HashMap::from([(
                "ttyUSB0".into(),
                V1Source {
                    label: Some("ttyUSB0".into()),
                    type_: Some("NMEA0183".into()),
                    properties: HashMap::from([
                        (
                            "II".into(),
                            V1SourceProperty {
                                talker: Some("II".into()),
                                sentences: Some(HashMap::from([(
                                    "HDM".into(),
                                    "2017-05-16T05:15:54.006Z".into(),
                                )])),
                                extras: Default::default(),
                            },
                        ),
                        (
                            "GP".into(),
                            V1SourceProperty {
                                talker: Some("GP".into()),
                                sentences: Some(HashMap::from([(
                                    "RMC".into(),
                                    "2017-04-03T06:14:04.451Z".into(),
                                )])),
                                extras: Default::default(),
                            },
                        ),
                    ]),
                },
            )]),
        })
        .build();
    let nexpected = V1RootFormat {
        version: "1.0.0".into(),
        self_: "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into(),
        vessels: Some(HashMap::from([(
            "urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into(),
            V1Vessel {
                uuid: Some("urn:mrn:signalk:uuid:705f5f1a-efaf-44aa-9cb8-a0fd6305567c".into()),
                mmsi: None,
                name: Some("Motu".into()),
                navigation: Some(V1Navigation {
                    speed_over_ground: Some(V1NumberValue {
                        value: 4.32693662,
                        timestamp: "2017-05-16T05:15:50.007Z".into(),
                        source: "ttyUSB0.GP".into(),
                        pgn: None,
                        sentence: Some("RMC".into()),
                    }),
                    course_over_ground_true: None,
                    heading_magnetic: Some(V1NumberValue {
                        value: 5.55014702,
                        timestamp: "2017-05-16T05:15:54.006Z".into(),
                        source: "ttyUSB0.II".into(),
                        pgn: None,
                        sentence: Some("HDM".into()),
                    }),
                    position: Some(V1PositionType {
                        value: V1PositionValue {
                            latitude: 37.81479,
                            longitude: -122.44880152,
                            altitude: Some(0.0),
                        },
                        timestamp: "2017-05-16T05:15:50.007Z".into(),
                        source: "ttyUSB0.GP".into(),
                        pgn: None,
                        sentence: Some("RMC".into()),
                    }),
                }),
                propulsion: None,
            },
        )])),
        sources: Some(V1Sources {
            type_: None,
            fields: HashMap::from([(
                "ttyUSB0".into(),
                V1Source {
                    label: Some("ttyUSB0".into()),
                    type_: Some("NMEA0183".into()),
                    properties: HashMap::from([
                        (
                            "II".into(),
                            V1SourceProperty {
                                talker: Some("II".into()),
                                sentences: Some(HashMap::from([(
                                    "HDM".into(),
                                    "2017-05-16T05:15:54.006Z".into(),
                                )])),
                                extras: Default::default(),
                            },
                        ),
                        (
                            "GP".into(),
                            V1SourceProperty {
                                talker: Some("GP".into()),
                                sentences: Some(HashMap::from([(
                                    "RMC".into(),
                                    "2017-04-03T06:14:04.451Z".into(),
                                )])),
                                extras: Default::default(),
                            },
                        ),
                    ]),
                },
            )]),
        }),
    };
    let sk_data = read_signalk_from_file("tests/specification/examples/full/docs-data_model.json");
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_data_model_metadata() {
    let expected = V1RootFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_vessel("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into(),
                    V1Vessel::builder()
                        .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                        .add_propulsion("instance0".into(), V1Propulsion::builder()
                            .label("Port Engine".into())
                            .revolutions(V1NumberValue::builder()
                                .value(1280.0)
                                .timestamp("2014-08-15T19:00:15.402Z".into())
                                .source("foo.bar".into())
                                .build())
                            .build())
                        .build()).build();
    let sk_data =
        read_signalk_from_file("tests/specification/examples/full/docs-data_model_metadata.json");
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_data_model_multiple_values_metadata() {
    let expected = V1RootFormat::builder()
        .version("0.9.0".into())
        .self_("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_vessel("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into(),
                    V1Vessel::builder()
                        .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                        .navigation(
                            V1Navigation::builder()
                                .course_over_ground_true(
                                    V1NumberValue::builder()
                                        .value(3.61562407843144)
                                        .timestamp("2017-04-03T06:14:04.451Z".into())
                                        .source("ttyUSB0.GP".into())
                                        .build()
                                ).build())
                        .build(),
        )
        .build();
    let sk_data = read_signalk_from_file(
        "tests/specification/examples/full/docs-data_model_multiple_values.json",
    );
    assert_eq!(sk_data, expected);
}

#[test]
fn test_sample_docs_notifications() {
    let expected = V1RootFormat::builder()
        .version("1.0.0".into())
        .self_("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
        .add_vessel("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into(),
                    V1Vessel::builder()
                        .uuid("urn:mrn:signalk:uuid:c0d79334-4e25-4245-8892-54e8ccc8021d".into())
                        .build())
        .build();
    let sk_data =
        read_signalk_from_file("tests/specification/examples/full/docs-notifications.json");
    assert_eq!(sk_data, expected);
}
