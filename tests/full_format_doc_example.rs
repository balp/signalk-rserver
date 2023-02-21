use std::fs::File;
use std::io::BufReader;
use signalk_rserver::signalk::V1RootFormat;

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
    assert!(sources.fields.unwrap_ref().contains_key("ttyUSB0".into()));
    let usb_source = sources.fields.unwrap_ref().get("ttyUSB0".into()).unwrap();
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

#[test]
fn test_sample_full_0183_rmc_export() {
    let file = File::open("tests/specification/examples/full/0183-RMC-export.json").unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1RootFormat = serde_json::from_reader(reader).unwrap();
    let sample_vessel_id = "urn:mrn:imo:mmsi:366982330";
    assert_eq!(sk_data.self_, sample_vessel_id);
    let sk_vessel = sk_data.vessels.unwrap_ref().get(sample_vessel_id).unwrap();
    let navigation = sk_vessel.navigation.unwrap_ref();

    assert_eq!(navigation.position.unwrap_ref().value.latitude, -41.156426);
    assert_eq!(navigation.position.unwrap_ref().value.longitude, 173.1693);
    assert_eq!(navigation.position.unwrap_ref().value.altitude, None);
    assert_eq!(navigation.course_over_ground_true.unwrap_ref().value, 245.69);
}

#[test]
fn test_sample_full_0183_rmc_export_min() {
    let file = File::open("tests/specification/examples/full/0183-RMC-export-min.json").unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1RootFormat = serde_json::from_reader(reader).unwrap();
    let sample_vessel_id = "urn:mrn:imo:mmsi:366982330";
    assert_eq!(sk_data.self_, sample_vessel_id);
    let sk_vessel = sk_data.vessels.unwrap_ref().get(sample_vessel_id).unwrap();
    let navigation = sk_vessel.navigation.unwrap_ref();

    assert_eq!(navigation.position.unwrap_ref().value.latitude, -41.156426);
    assert_eq!(navigation.position.unwrap_ref().value.longitude, 173.1693);
    assert_eq!(navigation.position.unwrap_ref().value.altitude, Some(0.0));
    assert_eq!(navigation.course_over_ground_true.unwrap_ref().value, 245.69);

}

#[test]
fn test_sample_full_0183_rmc_full() {
    let file = File::open("tests/specification/examples/full/0183-RMC-full.json").unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1RootFormat = serde_json::from_reader(reader).unwrap();
    let sample_vessel_id = "urn:mrn:imo:mmsi:366982330";
    assert_eq!(sk_data.self_, sample_vessel_id);
    let sk_vessel = sk_data.vessels.unwrap_ref().get(sample_vessel_id).unwrap();
    let navigation = sk_vessel.navigation.unwrap_ref();

    assert_eq!(navigation.position.unwrap_ref().value.latitude, -41.156426);
    assert_eq!(navigation.position.unwrap_ref().value.longitude, 173.1693);
    assert_eq!(navigation.position.unwrap_ref().value.altitude, Some(0.0));
    assert_eq!(navigation.course_over_ground_true.unwrap_ref().value, 245.69);

}
