use std::{error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let mut o = 0;
    fs::read_to_string("./resources/aoc2024/inputs/day2.txt")?
        .split("\n")
        .for_each(|l| {
            if is_safe(l) {
                o += 1;
            }
        });

    println!("output: {}", o);

    Ok(())
}

fn is_safe(line: &str) -> bool {
    if line == "" {
        return false;
    }
    let entries: Vec<i32> = line
        .split(" ")
        .map(|x| FromStr::from_str(x).unwrap())
        .collect();

    let len = entries.len();
    let is_positive = (entries.get(1).unwrap() - entries.get(0).unwrap()).is_positive();
    for i in 0..len - 1 {
        let d = entries.get(i + 1).unwrap() - entries.get(i).unwrap();
        if d.abs() > 3 {
            return false;
        }
        if d == 0 {
            return false;
        }
        if is_positive && d.is_negative() {
            return false;
        }
        if !is_positive && d.is_positive() {
            return false;
        }
        print!("\n");
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::is_safe;

    // 7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9
    #[test]
    fn test() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let mut o = 0;

        input.split("\n").for_each(|l| {
            println!("{:?}", l);
            if is_safe(l) {
                o += 1;
            }
        });

        assert_eq!(2, o)
    }
}
