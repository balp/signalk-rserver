use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use serde_json::Number;

use signalk_rserver::signalk::{V1DefSource, V1DeltaFormat, V1UpdateType, V1UpdateValue, V1UpdateValueType};

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

fn read_signalk_from_file(path: PathBuf) -> V1DeltaFormat {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let sk_data: V1DeltaFormat = serde_json::from_reader(reader).unwrap();
    sk_data
}

#[test]
fn test_0183_rmc_export_delta() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("0183-RMC-export-delta.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.366982330.navigation".into())
        .add(V1UpdateType::builder()
            .add(V1UpdateValue::new("position.timestamp".into(),
                                    serde_json::value::Value::String("2015-03-07T12:37:10.523+13:00".into())))
            .add(V1UpdateValue::new("position.longitude".into(), serde_json::value::Value::Number(Number::from_f64(173.1693).unwrap())))
            .add(V1UpdateValue::new("position.latitude".into(), serde_json::value::Value::Number(Number::from_f64(-41.156426).unwrap())))
            .add(V1UpdateValue::new("position.source".into(), serde_json::value::Value::String("sources.gps_0183_RMC".into())))
            .add(V1UpdateValue::new("position.altitude".into(), serde_json::value::Value::Number(Number::from(0))))
            .add(V1UpdateValue::new("courseOverGroundTrue".into(), serde_json::value::Value::Number(Number::from_f64(245.69).unwrap())))
            .ref_source("sources.gps_0183_RMC".into())
            .build())
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_0183_rmc_export_min_delta() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("0183-RMC-export-min-delta.json"));
    let expected = V1DeltaFormat::builder()
        .context("vessels.366982330.navigation".into())
        .add(V1UpdateType::builder()
            .add(V1UpdateValue::new("position.longitude".into(), serde_json::value::Value::Number(Number::from_f64(173.1693).unwrap())))
            .add(V1UpdateValue::new("position.latitude".into(), serde_json::value::Value::Number(Number::from_f64(-41.156426).unwrap())))
            .add(V1UpdateValue::new("position.altitude".into(), serde_json::value::Value::Number(Number::from(0))))
            .add(V1UpdateValue::new("courseOverGroundTrue".into(), serde_json::value::Value::Number(Number::from_f64(245.69).unwrap())))
            .build())
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_docs_data_model() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model.json"));
    let map = serde_json::Map::from_iter([("name".to_string(), serde_json::value::Value::String("WRANGO".into()))]);
    let expected = V1DeltaFormat::builder()
        .context("vessels.urn:mrn:imo:mmsi:234567890".into())
        .add(V1UpdateType::builder()
            .source(V1DefSource::default())

            .add(V1UpdateValue::new("propulsion.0.revolutions".into(), serde_json::value::Value::Number(Number::from_f64(16.341667).unwrap())))
            .add(V1UpdateValue::new("propulsion.0.boostPressure".into(), serde_json::value::Value::Number(Number::from_f64(45500.0).unwrap())))
            .timestamp("2010-01-07T07:18:44Z".into())
            .build())
         .add(V1UpdateType::builder()
            .add(V1UpdateValue::new("navigation.courseOverGroundTrue".into(), serde_json::value::Value::Number(Number::from_f64(2.971).unwrap())))
            .add(V1UpdateValue::new("navigation.speedOverGround".into(), serde_json::value::Value::Number(Number::from_f64(3.85).unwrap())))
             .timestamp("2014-08-15T16:00:00.081Z".into())
            .build())
         .add(V1UpdateType::builder()
            .add(V1UpdateValue::new("".into(), serde_json::value::Value::Object(map)))
             .timestamp("2014-08-15T19:02:31.507Z".into())
            .build())
        .build();
    assert_eq!(sk_data, expected)
}

#[test]
fn test_docs_data_model_meta_deltas() {
    let folder = Path::new("tests/specification/examples/delta/");
    let sk_data = read_signalk_from_file(folder.join("docs-data_model_meta_deltas.json"));
    let map = serde_json::Map::from_iter([("name".to_string(), serde_json::value::Value::String("WRANGO".into()))]);
    let expected = V1DeltaFormat::builder()
        .add(V1UpdateType::builder()
            .timestamp("2014-08-15T19:02:31.507Z".into())
            .build())
        .context("vessels.urn:mrn:imo:mmsi:234567890".into())
        .build();
    assert_eq!(sk_data, expected)
}

