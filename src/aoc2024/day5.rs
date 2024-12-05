use std::{error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./resources/aoc2024/inputs/day5.txt")?;

    let split: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<(i32, i32)> = split
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
        if is_page_safe(&rules, &t) {
            println!("page is safe: {:?}", t);
            let i = t.len();
            let m = t[i / 2];

            o += m;
        }
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
                // can we reorder
            }
        }
    }

    return true;
}
