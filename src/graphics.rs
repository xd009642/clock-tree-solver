use crate::types::*;
use petgraph::Direction;
use cairo::Context;
use std::cmp::max;
use std::collections::HashMap;



pub trait Symbol {
    /// Render the symbol at the given coordinates. 
    /// * x - x coordinate of origin
    /// * y - y coordinate of origin
    /// * width - symbol width
    /// * height - symbol height
    fn render(&self, ctx: &Context, x: f64, 
              y: f64, width: f64, height: f64);      
}


/// Renders a node in the clock tree
impl Symbol for Node {
    fn render(&self, ctx: &Context, x: f64, y: f64, width: f64, height: f64) {
        match *self {
            Node::Mux => {
            }, 
            _ => {},
        }
    }
}

/// Render the whole clock tree
impl Symbol for ClockTree {
    fn render(&self, ctx: &Context, x: f64, y: f64, width: f64, height: f64) {
        let v_margin = 0.05f64 * height;
        let n_inputs = self.externals(Direction::Incoming).count();
        let n_outputs = self.externals(Direction::Outgoing).count();

        let max_ends = max(n_inputs, n_outputs) as f64;

        let end_height = (height - max_ends*v_margin)/max_ends;

        let mut nodes: HashMap<Node, (f64, f64)> = HashMap::new();
        for inputs in self.externals(Direction::Incoming) {
            // Place internal node, then trace neighbours
                        
        }
    }
}
