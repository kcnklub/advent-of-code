use std::{error::Error, fs, str::FromStr};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./resources/aoc2024/inputs/day3.txt")?;

    let reg = Regex::new(r"(|mul\(([0-9]+),([0-9]+)\))")?;

    let mut output = 0;

    for (_, [value, left, right]) in reg.captures_iter(&input).map(|c| c.extract()) {
        let l: i32 = FromStr::from_str(left)?;
        let r: i32 = FromStr::from_str(right)?;
        println!("{}, {}, {}", value, left, right);

        output += l * r;
    }

    println!("output: {}", output);

    Ok(())
}
