use crate::types::*;
use petgraph::{Direction, graph::NodeIndex};
use cairo::Context;
use std::cmp::{max, min};
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
            _ => {
                ctx.set_source_rgb(0.0, 0.0, 0.0);
                ctx.rectangle(x, y, width, height);
                ctx.stroke();
            },
        }
    }
}

/// Render the whole clock tree
impl Symbol for ClockTree {
    fn render(&self, ctx: &Context, x: f64, y: f64, width: f64, height: f64) {
        let h_margin = 0.025f64 * width;
        let v_margin = 0.05f64 * height;
        let n_inputs = self.externals(Direction::Incoming).count();
        let n_outputs = self.externals(Direction::Outgoing).count();

        let max_ends = max(n_inputs, n_outputs) as f64;

        let end_height = (height - max_ends*v_margin)/max_ends;
        let end_height = min(end_height as u32, 50) as f64;

        let mut y = v_margin;
        let mut nodes: HashMap<NodeIndex<IndexType>, (f64, f64)> = HashMap::new();
        for input in self.externals(Direction::Incoming) {
            if let Some(ref n) = self.node_weight(input) {
                // Place internal node, then trace neighbours
                if n.is_source() {
                    n.render(ctx, h_margin, y, 0.15f64*width, end_height);
                    nodes.insert(input, (h_margin, y));
                    y += v_margin + end_height;
                } else {
                    // Is an orphaned internal or sink
                }
            }
        }

        for n in self.node_indices() {
            if !nodes.contains_key(&n) {
                if let Some(ref temp) = self.node_weight(n) {
                    temp.render(ctx, width/2.0, height/2.0, 30.0, 30.0);
                }
            }
        }
    }
}
