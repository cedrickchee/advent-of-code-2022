use std::error::Error;

use shared::input::load_text_input_from_autodetect;

use day01::{parse_input, part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input(load_text_input_from_autodetect())?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));

    Ok(())
}
