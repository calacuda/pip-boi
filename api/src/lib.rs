use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Display;
pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Serialize, Deserialize, Default,
)]
pub enum MenuTab {
    #[default]
    #[serde(rename = "stats")]
    Stats,
    // Inv,
    #[serde(rename = "map")]
    Map,
    // Radio,
    // Compas,
    #[serde(rename = "com")]
    Serial,
    #[serde(rename = "ducky")]
    Ducky,
    #[serde(rename = "cal")]
    Cal,
    #[serde(rename = "todo")]
    Todo,
    // Wifi,
}

impl Display for MenuTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Stats => write!(f, "STATS"),
            // Self::Inv => write!(f, "INV"),
            Self::Map => write!(f, "MAP"),
            // Self::Radio => write!(f, "RADIO"),
            // Self::Compas => write!(f, "COMPAS"),
            Self::Serial => write!(f, "COM"),
            Self::Ducky => write!(f, "DUCKY"),
            Self::Cal => write!(f, "CAL"),
            Self::Todo => write!(f, "TODO"),
            // Self::Wifi => write!(f, "WiFi"),
            // => write!(f, ""),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MsgType {
    /// Heart rate.
    HeartRate,
    /// blood oxygen level.
    BloodO2,
    /// a button was pressed, message will contain which button and what press type.
    ButtonPress,
    /// the turning of the rotary knob, message will contain direction.
    RotaryKnob,
    /// GPS location data update.
    LocationUpdate,
    /// a message was received over the serial connection.
    SerailRx,
    /// a message to be sent.
    SerialTx,
    /// used to tell inform clients of the currently active tab.
    TabChange,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Buttons {
    Rotary,
    Stats,
    Map,
    Serial,
    Ducky,
    Cal,
    Todo,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PressType {
    Single,
    Double,
    Triple,
    Long,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PipBoiMsg {
    #[serde(rename = "type")]
    pub msg_type: MsgType,
    #[serde(default = "empty_map")]
    pub data: Value,
}

pub fn empty_map() -> Value {
    // HashMap::new()
    json!({})
}
