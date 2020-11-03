mod circuit;
mod clock;
mod gate;
mod memory;

pub use crate::gate::logic_gates;

// TODO: change to lib.rs
fn main() {
    let input: u8 = 0;
    println!("input = 0, output = {}", logic_gates::not(input));
    println!(
        "input = 1, output = {}",
        logic_gates::not(logic_gates::not(1))
    );
}
