use std::{error::Error, fs, str::FromStr};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./resources/aoc2024/inputs/day3.txt")?;

    let reg = Regex::new(r"(do\(\)|don't\(\))|(mul\(([0-9]+),([0-9]+)\))")?;

    let mut enabled = true;
    let mut output = 0;

    reg.captures_iter(&input).for_each(|c| {
        println!("{:?}", c);
        if let Some(toggle) = c.get(0) {
            println!("{:?}", toggle.as_str());
            match toggle.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        let l = c.get(3).unwrap();
                        let l: i32 = FromStr::from_str(l.as_str()).unwrap();
                        let r = c.get(4).unwrap();
                        let r: i32 = FromStr::from_str(r.as_str()).unwrap();

                        output += l * r;
                    }
                }
            }
        }
    });

    println!("output: {}", output);

    Ok(())
}
