use std::fmt;
use std::hash::{Hash, Hasher};
use petgraph::{Directed, graph::DefaultIx, stable_graph::StableGraph};
use serde_derive::{Serialize, Deserialize};
use uom::si::{f64::Frequency, frequency::millihertz};


pub type IndexType = DefaultIx;

pub type ClockTree = StableGraph<Node, (), Directed, IndexType>;

/// Used to represent a frequency value in the system.
/// Value can either be a constant, an acceptable range or for endpoints
/// where there's no desired value a Don't Care
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Value {
    /// The desired frequency of the endpoint
    Constant(Frequency),
    /// An exceptable range of frequencies for the endpoint
    Range{ 
        min: Frequency,
        max: Frequency,
    },
    /// A value that is unimportant to the designer
    DontCare,
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut float_hash = |x: f64| {
            (x.round() as i32).hash(state);
        };
        match *self {
            Value::Constant(ref f) => {
                float_hash(f.get::<millihertz>());
            },
            Value::Range{ref min, ref max} => {
                float_hash(min.get::<millihertz>());
                float_hash(max.get::<millihertz>());
            },
            Value::DontCare => {
                (-1i32).hash(state); 
            }
        }
    }
}

impl Eq for Value {}

/// Used to represent frequency sources and sinks.
/// Each one is given a unique name
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Endpoint {
    /// Endpoint name
    pub name: String,
    /// Endpoint value
    pub value: Value,
    /// Whether the endpoint is internal or external
    pub is_internal: bool,
}


#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Node {
    /// Division block. Divide input by one of the potential values
    Divide(Vec<u64>),
    /// Multiplication block. Multiply input by one of the potential values
    Multiply(Vec<u64>),
    /// Mux block, takes a set of signals and lets one pass through
    Mux,
    /// Input frequency (internal or external)
    Input(Endpoint),
    /// Output frequency (internal or external)
    Output(Endpoint),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Divide(p) | Node::Multiply(p) => {
                for x in p {
                    writeln!(f, "{} ", x)?;
                }
            },
            _ => {},
        }
        Ok(())
    }
}

impl Node {
    /// Maximum amount of inputs to a block
    pub fn max_inputs(&self) -> usize {
        match self {
            Node::Divide(_) | Node::Multiply(_) => 1,
            Node::Input(_) => 0,
            _ => usize::max_value(),
        }
    }

    /// Maximum amount of outputs to a block
    pub fn max_outputs(&self) -> usize {
        match self {
            Node::Output(_) => 0,
            _ => usize::max_value(),
        }
    }

    /// Returns true if the block is a source, false otherwise 
    pub fn is_source(&self) -> bool {
        self.max_inputs() == 0
    }

    /// Returns true if the block is a sink, false otherwise
    pub fn is_sink(&self) -> bool {
        self.max_outputs() == 0
    }
}


#[cfg(test)]
mod tests {
    use crate::types::*;
    
    #[test]
    fn test_max_outputs() {
        let dummy_end = Endpoint {
            name: "Dummy".to_string(),
            value: Value::DontCare,
            is_internal: false,
        };
        let out = Node::Output(dummy_end.clone());
        let inp = Node::Input(dummy_end);
        let mux = Node::Mux;
        let div = Node::Divide(Vec::new());
        let mul = Node::Multiply(Vec::new());

        assert_eq!(out.max_outputs(), 0);
        assert_eq!(inp.max_outputs(), usize::max_value());
        assert_eq!(mux.max_outputs(), usize::max_value());
        assert_eq!(mul.max_outputs(), usize::max_value());
        assert_eq!(div.max_outputs(), usize::max_value());
    }

    #[test]
    fn test_max_inputs() {
        let dummy_end = Endpoint {
            name: "Dummy".to_string(),
            value: Value::DontCare,
            is_internal: false,
        };
        let out = Node::Output(dummy_end.clone());
        let inp = Node::Input(dummy_end);
        let mux = Node::Mux;
        let div = Node::Divide(Vec::new());
        let mul = Node::Multiply(Vec::new());

        assert_eq!(out.max_inputs(), usize::max_value());
        assert_eq!(inp.max_inputs(), 0);
        assert_eq!(mux.max_inputs(), usize::max_value());
        assert_eq!(mul.max_inputs(), 1);
        assert_eq!(div.max_inputs(), 1);
    }
}
