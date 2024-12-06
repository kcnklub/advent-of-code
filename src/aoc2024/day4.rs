use std::{char, error::Error, fmt::Display, fs, usize};

#[derive(Debug)]
enum TileValue {
    X,
    M,
    A,
    S,
    DOT,
}

impl Display for TileValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileValue::X => write!(f, "X"),
            TileValue::M => write!(f, "M"),
            TileValue::A => write!(f, "A"),
            TileValue::S => write!(f, "S"),
            TileValue::DOT => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct Tile {
    v: TileValue,
    found: bool,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        return Tile {
            v: value.into(),
            found: false,
        };
    }
}

impl From<char> for TileValue {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            '.' => Self::DOT,
            _ => panic!("Invalid state"),
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    w: usize,
    h: usize,
    tiles: Vec<Vec<Tile>>,
}

const LEN: usize = 3;

impl Puzzle {
    fn new(input: String) -> Self {
        let lines: Vec<&str> = input.split("\n").filter(|l| !l.is_empty()).collect();

        let w = lines.get(0).unwrap().len();
        let mut tiles: Vec<Vec<Tile>> = vec![];
        for line in lines {
            let mut r = vec![];
            for c in line.chars() {
                r.push(c.into());
            }
            if r.len() == w {
                tiles.push(r);
            }
        }
        let h = tiles.len();

        return Self { w, h, tiles };
    }

    fn find_xmas(&mut self) -> i32 {
        let mut o = 0;
        for h in 0..self.h {
            for w in 0..self.w {
                if let TileValue::X = &self.tiles[h][w].v {
                    if self.search_up(h, w) {
                        o += 1;
                    }
                    if self.search_down(h, w) {
                        o += 1;
                    }
                    if self.search_right(h, w) {
                        o += 1;
                    }
                    if self.search_left(h, w) {
                        o += 1;
                    }
                    if self.search_up_right(h, w) {
                        o += 1;
                    }
                    if self.search_up_left(h, w) {
                        o += 1;
                    }
                    if self.search_down_right(h, w) {
                        o += 1;
                    }
                    if self.search_down_left(h, w) {
                        o += 1;
                    }
                }
            }
        }
        o
    }

    /// this is going to be part 2 instead of creating a new file
    fn find_x_max(&mut self) -> i32 {
        0
    }

    fn found(&mut self, h: usize, w: usize) {
        let tile = &mut self.tiles[h][w];
        tile.found = true
    }

    fn search_right(&mut self, h: usize, w: usize) -> bool {
        if w + LEN > self.w - 1 {
            return false;
        }

        if let TileValue::M = &self.tiles[h][w + 1].v {
            if let TileValue::A = &self.tiles[h][w + 2].v {
                if let TileValue::S = &self.tiles[h][w + LEN].v {
                    self.found(h, w);
                    self.found(h, w + 1);
                    self.found(h, w + 2);
                    self.found(h, w + 3);
                    return true;
                }
            }
        }

        false
    }

    fn search_left(&mut self, h: usize, w: usize) -> bool {
        if w < LEN {
            return false;
        }

        if let TileValue::M = &self.tiles[h][w - 1].v {
            if let TileValue::A = &self.tiles[h][w - 2].v {
                if let TileValue::S = &self.tiles[h][w - LEN].v {
                    self.found(h, w);
                    self.found(h, w - 1);
                    self.found(h, w - 2);
                    self.found(h, w - 3);
                    return true;
                }
            }
        }
        false
    }

    fn search_down(&mut self, h: usize, w: usize) -> bool {
        if h + LEN > self.h - 1 {
            return false;
        }

        if let TileValue::M = &self.tiles[h + 1][w].v {
            if let TileValue::A = &self.tiles[h + 2][w].v {
                if let TileValue::S = &self.tiles[h + LEN][w].v {
                    self.found(h, w);
                    self.found(h + 1, w);
                    self.found(h + 2, w);
                    self.found(h + 3, w);
                    return true;
                }
            }
        }

        false
    }

    fn search_up(&mut self, h: usize, w: usize) -> bool {
        if h < LEN {
            return false;
        }

        if let TileValue::M = &self.tiles[h - 1][w].v {
            if let TileValue::A = &self.tiles[h - 2][w].v {
                if let TileValue::S = &self.tiles[h - LEN][w].v {
                    self.found(h, w);
                    self.found(h - 1, w);
                    self.found(h - 2, w);
                    self.found(h - 3, w);
                    return true;
                }
            }
        }

        false
    }

    fn search_up_right(&mut self, h: usize, w: usize) -> bool {
        if h < LEN || w + LEN > self.w - 1 {
            return false;
        }
        if let TileValue::M = &self.tiles[h - 1][w + 1].v {
            if let TileValue::A = &self.tiles[h - 2][w + 2].v {
                if let TileValue::S = &self.tiles[h - LEN][w + LEN].v {
                    self.found(h, w);
                    self.found(h - 1, w + 1);
                    self.found(h - 2, w + 2);
                    self.found(h - 3, w + 3);
                    return true;
                }
            }
        }
        return false;
    }
    fn search_up_left(&mut self, h: usize, w: usize) -> bool {
        if h < LEN || w < LEN {
            return false;
        }
        if let TileValue::M = &self.tiles[h - 1][w - 1].v {
            if let TileValue::A = &self.tiles[h - 2][w - 2].v {
                if let TileValue::S = &self.tiles[h - LEN][w - LEN].v {
                    self.found(h, w);
                    self.found(h - 1, w - 1);
                    self.found(h - 2, w - 2);
                    self.found(h - 3, w - 3);
                    return true;
                }
            }
        }
        return false;
    }
    fn search_down_right(&mut self, h: usize, w: usize) -> bool {
        if h + LEN > self.h - 1 || w + LEN > self.w - 1 {
            return false;
        }
        if let TileValue::M = &self.tiles[h + 1][w + 1].v {
            if let TileValue::A = &self.tiles[h + 2][w + 2].v {
                if let TileValue::S = &self.tiles[h + LEN][w + LEN].v {
                    self.found(h, w);
                    self.found(h + 1, w + 1);
                    self.found(h + 2, w + 2);
                    self.found(h + 3, w + 3);
                    return true;
                }
            }
        }

        return false;
    }
    fn search_down_left(&mut self, h: usize, w: usize) -> bool {
        if h + LEN > self.h - 1 || w < LEN {
            return false;
        }

        if let TileValue::M = &self.tiles[h + 1][w - 1].v {
            if let TileValue::A = &self.tiles[h + 2][w - 2].v {
                if let TileValue::S = &self.tiles[h + LEN][w - LEN].v {
                    self.found(h, w);
                    self.found(h + 1, w - 1);
                    self.found(h + 2, w - 2);
                    self.found(h + 3, w - 3);
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./resources/aoc2024/inputs/day4.txt")?;

    let mut puzzle = Puzzle::new(input);
    let output = puzzle.find_xmas();

    println!("output: {}", output);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Puzzle, TileValue};
    use colored::Colorize;

    #[test]
    fn test() {
        let input = r#"
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#;

        let mut puzzle = Puzzle::new(input.to_string());
        let output = puzzle.find_xmas();
        for row in puzzle.tiles {
            for tile in row {
                if tile.found {
                    print!("{}", something(tile.v).as_str().red())
                } else {
                    print!("{}", something(tile.v))
                }
            }
            print!("\n")
        }

        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let puzzle = Puzzle::new(input.to_string());
        //let output = puzzle.find_xmas();

        assert_eq!(18, output)
    }

    fn something(i: TileValue) -> String {
        match i {
            TileValue::X => "X".to_string(),
            TileValue::M => "M".to_string(),
            TileValue::A => "A".to_string(),
            TileValue::S => "S".to_string(),
            TileValue::DOT => ".".to_string(),
        }
    }
}
