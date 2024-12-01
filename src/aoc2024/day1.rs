use std::{error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let mut p = vec![];
    let mut q = vec![];

    fs::read_to_string("./resources/aoc2024/inputs/day1.txt")?
        .split("\n")
        .map(|l| {
            if l == "" {
                return None;
            }
            let split: Vec<i32> = l
                .split("   ")
                .map(|x| FromStr::from_str(x).unwrap())
                .collect();
            Some((split[0], split[1]))
        })
        .for_each(|optional| {
            if let Some((l, r)) = optional {
                p.push(l);
                q.push(r);
            }
        });

    let o = find_sum(&mut p, &mut q);
    println!("output: {}", o);

    Ok(())
}

fn find_sum(p: &mut Vec<i32>, q: &mut Vec<i32>) -> i32 {
    let l = p.len();
    p.sort();
    q.sort();

    let mut o = 0;
    for i in 0..l {
        let p_i = p.get(i).unwrap();
        let q_i = q.get(i).unwrap();
        o += i32::abs(p_i - q_i);
    }

    o
}

#[cfg(test)]
mod test {
    use super::find_sum;

    ///3   4
    ///4   3
    ///2   5
    ///1   3
    ///3   9
    ///3   3
    #[test]
    fn test() {
        let mut p = vec![3, 4, 2, 1, 3, 3];
        let mut q = vec![4, 3, 5, 3, 9, 3];

        let o = find_sum(&mut p, &mut q);

        assert_eq!(11, o)
    }
}
