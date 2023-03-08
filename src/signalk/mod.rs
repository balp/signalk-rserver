//! # Signal K
//!
//! `signalk` is a collections of types to serialize and deserialize the
//! signal-k protocol.

pub use definitions::{
    V1Attr, V1CommonValueFields, V1DefSource, V1Meta, V1MetaZone, V1NumberValue,
};
pub use delta::{V1DeltaFormat, V1UpdateMeta, V1UpdateType, V1UpdateValue, V1UpdateValueType};
pub use discovery::{V1Discovery, V1DiscoveryEndpoint, V1DiscoveryServer};
pub use electrical::{V1ACBus, V1Electrical, V1ElectricalACQualities, V1ElectricalIdentity};
pub use environment::{
    V1Environment, V1EnvironmentCurrent, V1EnvironmentCurrentValue, V1EnvironmentDepth,
    V1EnvironmentInside, V1EnvironmentTime,
};
pub use full::V1FullFormat;
pub use hello::V1Hello;
pub use navigation::{V1Navigation, V1PositionType, V1PositionValue};
pub use notification::{V1Notification, V1NotificationValue};
pub use propulsion::V1Propulsion;
pub use put::{V1Put, V1PutValue};
pub use sources::{V1Source, V1SourceProperty, V1Sources};
pub use subscribe::{V1Subscribe, V1Subscription};
pub use unsubscribe::{V1Unsubscribe, V1Unsubscription};
pub use vessel::V1Vessel;

pub mod definitions;
pub mod delta;
pub mod discovery;
pub mod electrical;
pub mod environment;
pub mod full;
pub mod hello;
pub mod navigation;
pub mod notification;
pub mod propulsion;
pub mod put;
pub mod sources;
pub mod subscribe;
pub mod unsubscribe;
pub mod vessel;
