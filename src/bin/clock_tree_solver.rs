
use clock_solver::*;

use uom::si::f64::*;
use uom::si::frequency::hertz;
use crate::types::*;

fn main() {
    let x = Endpoint {
        name: "Crystal".to_string(),
        value: Value::Constant(Frequency::new::<hertz>(37368.0)),
        is_internal: false,
    };
    println!("Endpoint {:#?}", x);
}
