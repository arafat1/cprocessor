mod circuit;
mod gate;
mod memory;
mod clock;

pub use crate::gate::logic_gates;

fn main() {
    let input: u8 = 0;
    println!("input = 0, output = {}", logic_gates::not(input));
    println!(
        "input = 1, output = {}",
        logic_gates::not(logic_gates::not(1))
    );
}
