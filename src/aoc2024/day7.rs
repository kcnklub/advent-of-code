use std::{error::Error, fs, str::FromStr};

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut o = 0;
    fs::read_to_string("./resources/aoc2024/inputs/day7.txt")?
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let split: Vec<&str> = line.split(":").collect();
            println!("split: {:?}", split);

            let t: u64 = FromStr::from_str(split[0]).unwrap();
            let v: Vec<u64> = split[1]
                .trim()
                .split(" ")
                .map(|n| FromStr::from_str(n.trim()).unwrap())
                .collect();

            if can_be_solved(t, &v) {
                o += t
            }
        });

    println!("output: {o}");

    Ok(())
}

fn can_be_solved(t: u64, l: &[u64]) -> bool {
    let o1 = can_be_solved_internal(t, l[0], &l[1..]);
    let o2 = can_be_solved_internal(t, l[0], &l[1..]);

    return o1 || o2;
}

fn can_be_solved_internal(t: u64, middle: u64, l: &[u64]) -> bool {
    if l.len() == 1 {
        return middle * l[0] == t || middle + l[0] == t || concat_numbers(middle, l[0]) == t;
    }
    let o1 = can_be_solved_internal(t, middle * l[0], &l[1..]);
    let o2 = can_be_solved_internal(t, middle + l[0], &l[1..]);
    let o3 = can_be_solved_internal(t, concat_numbers(middle, l[0]), &l[1..]);

    return o1 || o2 || o3;
}

fn concat_numbers(l: u64, r: u64) -> u64 {
    let mut l = l.to_string();
    let r = r.to_string();
    l.push_str(&r);
    FromStr::from_str(&l).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(can_be_solved(190, &vec![10, 19]));
        assert!(can_be_solved(3267, &vec![81, 40, 27]));
        assert!(!can_be_solved(83, &vec![17, 5]));
        assert!(!can_be_solved(156, &vec![15, 6]))
    }
}
