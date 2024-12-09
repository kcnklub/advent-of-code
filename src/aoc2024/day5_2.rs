use std::{collections::HashMap, error::Error, fs, ops::Index, str::FromStr};

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./resources/aoc2024/inputs/day5.txt")?;
    let split: Vec<&str> = input.split("\n\n").collect();

    let rules_to_check: Vec<(i32, i32)> = split
        .get(0)
        .unwrap()
        .split("\n")
        .map(|r| {
            let t: Vec<i32> = r
                .split("|")
                .map(|s| FromStr::from_str(s).unwrap())
                .collect();
            (t.get(0).unwrap().to_owned(), t.get(1).unwrap().to_owned())
        })
        .collect();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    split.get(0).unwrap().split("\n").for_each(|r| {
        let t: Vec<i32> = r
            .split("|")
            .map(|s| FromStr::from_str(s).unwrap())
            .collect();

        if !rules.contains_key(&t[0]) {
            rules.insert(t[0], vec![]);
        }
        rules.get_mut(&t[0]).unwrap().push(t[1]);
    });

    let mut o = 0;
    let page_numbers: Vec<&str> = split.get(1).unwrap().split("\n").collect();
    for page in page_numbers.into_iter() {
        if page == "" {
            continue;
        }
        let t: Vec<i32> = page
            .split(",")
            .map(|s| FromStr::from_str(s).unwrap())
            .collect();

        let mut passing: Vec<i32> = vec![];
        for i in &t {
            if passing.is_empty() {
                passing.push(i.clone());
            } else {
                let related = rules
                    .get(&i)
                    .unwrap()
                    .into_iter()
                    .map(|v| {
                        passing
                            .clone()
                            .into_iter()
                            .position(|v2| v.clone() == v2.clone())
                    })
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .min();
                match related {
                    Some(v) => passing.insert(v, i.clone()),
                    None => passing.push(i.clone()),
                }
            }
        }

        let i = passing.len();
        let m = passing[i / 2];

        o += m;
    }

    println!("Output: {}", o);

    Ok(())
}

fn is_page_safe(rules: &[(i32, i32)], page: &[i32]) -> bool {
    for r in rules.into_iter() {
        if let Some(l) = page.iter().position(|e| e == &r.0) {
            if let Some(r) = page.iter().position(|e| e == &r.1) {
                if l > r {
                    return false;
                }
            }
        }
    }

    return true;
}
