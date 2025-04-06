mod compound_data_types;
mod functions;
mod primitive_data_types;

use crate::compound_data_types::compounds;
use crate::functions::functions;
use crate::primitive_data_types::primitives;

pub fn main() {
    //run: cargo run
    primitives();
    compounds();
    functions();
}
