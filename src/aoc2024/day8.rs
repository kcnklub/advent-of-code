use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs, i64, usize,
};

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./resources/aoc2024/inputs/day8.txt")?;

    let o = number_of_antinodes(&input);
    println!("Output: {o}");

    let o = number_of_antinodes_part_2(&input);
    println!("Output (part_2): {o}");
    Ok(())
}

fn number_of_antinodes(input: &str) -> usize {
    let (map, h, w) = build_map(input);
    println!("Dimensions: {h}x{w}");

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for (key, positions) in map.into_iter() {
        println!("\nProcessing {key}");
        for i in 0..positions.len() {
            let pos = positions[i];
            for j in 0..positions.len() {
                if i == j {
                    // do not need to compare these
                    continue;
                }

                let other = positions[j];
                println!("Pos: {:?}", pos);
                println!("Other: {:?}", other);
                let dy = pos.0 - other.0;
                let dx = pos.1 - other.1;

                println!("Vec (dy, dx): ({dy}, {dx})");

                let other_ant = (other.0 - dy, other.1 - dx);
                println!("other: {:?}, other_ant {:?}", other, other_ant);
                if other_ant.0 >= 0
                    && other_ant.0 < h as i64
                    && other_ant.1 >= 0
                    && other_ant.1 < w as i64
                {
                    antinodes.insert(other_ant);
                }
                let pos_ant = (pos.0 + dy, pos.1 + dx);
                println!("pos: {:?}, pos_ant {:?}", pos, pos_ant);
                if pos_ant.0 >= 0 && pos_ant.0 < h as i64 && pos_ant.1 >= 0 && pos_ant.1 < w as i64
                {
                    antinodes.insert(pos_ant);
                }
            }
        }
    }
    println!("antinodes: {:?}", antinodes);

    antinodes.len()
}

fn build_map(input: &str) -> (HashMap<char, Vec<(i64, i64)>>, usize, usize) {
    let mut map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let h = input.split("\n").filter(|l| !l.is_empty()).count();
    let mut w = 0;
    for (i, line) in input.split("\n").filter(|l| !l.is_empty()).enumerate() {
        w = line.chars().count();
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if let Some(list) = map.get_mut(&c) {
                list.push((i as i64, j as i64));
            } else {
                let list = vec![(i as i64, j as i64)];
                map.insert(c, list);
            }
        }
    }
    (map, h, w)
}

fn number_of_antinodes_part_2(input: &str) -> usize {
    let (map, h, w) = build_map(input);
    println!("Dimensions: {h}x{w}");

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for (key, positions) in map.into_iter() {
        println!("\nProcessing {key}");
        for i in 0..positions.len() {
            let pos = positions[i];
            for j in 0..positions.len() {
                if i == j {
                    // do not need to compare these
                    continue;
                }

                let other = positions[j];
                let dy = pos.0 - other.0;
                let dx = pos.1 - other.1;

                println!("Pos: {:?}, Other: {:?}", pos, other);
                antinodes.insert(pos);
                antinodes.insert(other);
                println!("Vec (dy, dx): ({dy}, {dx})");
                let mut inbounds = true;
                let mut curr = other.clone();
                while inbounds {
                    let other_ant = (curr.0 - dy, curr.1 - dx);
                    println!("other: {:?}, other_ant {:?}", other, other_ant);
                    if other_ant.0 >= 0
                        && other_ant.0 < h as i64
                        && other_ant.1 >= 0
                        && other_ant.1 < w as i64
                    {
                        antinodes.insert(other_ant);
                        curr = other_ant;
                    } else {
                        inbounds = false;
                    }
                }

                let mut inbounds = true;
                let mut curr = pos.clone();
                while inbounds {
                    let pos_ant = (curr.0 + dy, curr.1 + dx);
                    println!("pos: {:?}, pos_ant {:?}", pos, pos_ant);
                    if pos_ant.0 >= 0
                        && pos_ant.0 < h as i64
                        && pos_ant.1 >= 0
                        && pos_ant.1 < w as i64
                    {
                        antinodes.insert(pos_ant);
                        curr = pos_ant;
                    } else {
                        inbounds = false;
                    }
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if antinodes.contains(&(i as i64, j as i64)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!("")
    }
    println!("antinodes: {:?}", antinodes);

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let o = number_of_antinodes(input);

        assert_eq!(14, o);
    }

    #[test]
    fn part_2() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let o = number_of_antinodes_part_2(input);

        assert_eq!(34, o);
    }
}
