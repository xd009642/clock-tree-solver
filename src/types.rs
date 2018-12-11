use serde_derive::{Serialize, Deserialize};
use uom::si::f64::Frequency;


/// Used to represent a frequency value in the system.
/// Value can either be a constant, an acceptable range or for endpoints
/// where there's no desired value a Don't Care
#[derive(Debug, Deserialize, Serialize)]
pub enum Value {
    Constant(Frequency),
    Range{ 
        min: Frequency,
        max: Frequency,
    },
    DontCare,
}

/// Used to represent frequency sources and sinks.
/// Each one is given a unique name
#[derive(Debug, Deserialize, Serialize)]
pub struct Endpoint {
    pub name: String,
    pub value: Value,
    pub is_internal: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Node {
    Divide(Vec<u32>),
    Multiply(Vec<u32>),
    Mux,
    Input(Endpoint),
    Output(Endpoint),
}
