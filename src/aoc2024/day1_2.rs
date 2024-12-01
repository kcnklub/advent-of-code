use std::{collections::HashMap, error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let mut p = vec![];

    let mut map: HashMap<i32, i32> = HashMap::new();

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
                if let Some(v) = map.get_mut(&r) {
                    *v += 1
                } else {
                    map.insert(r, 1);
                }
            }
        });

    let o = find_sum_of_products(&mut p, &map);
    println!("output: {}", o);

    Ok(())
}

fn find_sum_of_products(p: &mut Vec<i32>, q: &HashMap<i32, i32>) -> i32 {
    let mut o = 0;

    p.into_iter().for_each(|v| {
        if let Some(val) = q.get(&v) {
            o += *v * val;
        }
    });

    o
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::find_sum_of_products;

    ///3   4
    ///4   3
    ///2   5
    ///1   3
    ///3   9
    ///3   3
    #[test]
    fn test() {
        let mut p = vec![3, 4, 2, 1, 3, 3];
        let q = vec![4, 3, 5, 3, 9, 3];

        let mut map = HashMap::new();
        q.into_iter().for_each(|i| {
            if let Some(v) = map.get_mut(&i) {
                *v += 1
            } else {
                map.insert(i, 1);
            }
        });

        let o = find_sum_of_products(&mut p, &map);

        assert_eq!(31, o)
    }
}
