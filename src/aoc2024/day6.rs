use std::{collections::HashSet, error::Error, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum MapTile {
    PLAYER(Direction),
    WALL,
    EMPTY,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3,
}

impl From<char> for MapTile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::EMPTY,
            '#' => Self::WALL,
            '>' => Self::PLAYER(Direction::RIGHT),
            '<' => Self::PLAYER(Direction::LEFT),
            '^' => Self::PLAYER(Direction::UP),
            'v' => Self::PLAYER(Direction::DOWN),
            _ => panic!("invalid map tile input: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    starting_location: (usize, usize),
    player_location: Option<(usize, usize)>,
    h: usize,
    w: usize,
    tiles: Vec<Vec<MapTile>>,
    found_tiles: HashSet<(usize, usize)>,
    found_turns: HashSet<(usize, usize, Direction)>,
}

impl Map {
    fn new(input: String) -> Self {
        let mut tiles = vec![];
        let mut player_location = (0, 0);
        for (h, line) in input.split("\n").filter(|l| !l.is_empty()).enumerate() {
            let mut row = vec![];
            for (w, c) in line.chars().enumerate() {
                let tile: MapTile = c.into();
                if let MapTile::PLAYER(_) = tile {
                    player_location = (h, w)
                }

                row.push(tile);
            }
            tiles.push(row);
        }

        Self {
            starting_location: player_location,
            player_location: Some(player_location),
            h: tiles.len(),
            w: tiles.get(0).unwrap().len(),
            tiles,
            found_tiles: HashSet::new(),
            found_turns: HashSet::new(),
        }
    }

    fn find_cycles(input: String) -> i32 {
        let initial_map = Self::new(input);
        let mut map = initial_map.clone();
        map.process_movement();

        let mut o = 0;
        let candidates = map.found_tiles.clone();
        for c in candidates {
            if c == map.starting_location {
                continue;
            }
            // determine if adding a blocker to the map at this location will cause a cycle;

            let mut new_map = initial_map.clone();
            std::mem::swap(&mut MapTile::WALL, &mut new_map.tiles[c.0][c.1]);
            if new_map.is_cyclic() {
                o += 1;
            }
        }
        o
    }

    fn process_movement(&mut self) -> usize {
        loop {
            match self.player_location {
                Some((h, w)) => {
                    let player = &self.tiles[h][w];
                    if let MapTile::PLAYER(d) = player {
                        match d {
                            Direction::UP => self.move_up(h, w),
                            Direction::DOWN => self.move_down(h, w),
                            Direction::RIGHT => self.move_right(h, w),
                            Direction::LEFT => self.move_left(h, w),
                        }
                    }
                }
                None => break,
            }
        }
        return self.found_tiles.len();
    }

    fn move_up(&mut self, h: usize, w: usize) {
        let mut current_pos = (h, w);
        loop {
            self.found_tiles.insert(current_pos);
            if current_pos.0 == 0 {
                self.player_location = None;
                break;
            }
            let next_pos = &self.tiles[current_pos.0 - 1][current_pos.1];
            match next_pos {
                MapTile::WALL => {
                    std::mem::swap(
                        &mut self.tiles[current_pos.0][current_pos.1],
                        &mut MapTile::PLAYER(Direction::RIGHT),
                    );
                    break;
                }
                MapTile::EMPTY => {
                    let (top, bottom) = self.tiles.split_at_mut(current_pos.0);
                    std::mem::swap(
                        &mut top[top.len() - 1][current_pos.1],
                        &mut bottom[0][current_pos.1],
                    );
                    current_pos = (current_pos.0 - 1, current_pos.1);
                    self.player_location = Some(current_pos);
                }
                MapTile::PLAYER(_) => panic!("invalid state"),
            }
        }
    }

    fn move_down(&mut self, h: usize, w: usize) {
        let mut current_pos = (h, w);
        loop {
            self.found_tiles.insert(current_pos);
            if current_pos.0 == self.h - 1 {
                self.player_location = None;
                break;
            }
            let next_pos = &self.tiles[current_pos.0 + 1][current_pos.1];
            match next_pos {
                MapTile::WALL => {
                    std::mem::swap(
                        &mut self.tiles[current_pos.0][current_pos.1],
                        &mut MapTile::PLAYER(Direction::LEFT),
                    );
                    break;
                }
                MapTile::EMPTY => {
                    let (top, bottom) = self.tiles.split_at_mut(current_pos.0 + 1);
                    std::mem::swap(
                        &mut top[top.len() - 1][current_pos.1],
                        &mut bottom[0][current_pos.1],
                    );
                    current_pos = (current_pos.0 + 1, current_pos.1);
                    self.player_location = Some(current_pos);
                }
                MapTile::PLAYER(_) => panic!("invalid state"),
            }
        }
    }

    fn move_right(&mut self, h: usize, w: usize) {
        let mut current_pos = (h, w);
        loop {
            self.found_tiles.insert(current_pos);
            if current_pos.1 == self.w - 1 {
                self.player_location = None;
                break;
            }
            let next_pos = &self.tiles[current_pos.0][current_pos.1 + 1];
            match next_pos {
                MapTile::WALL => {
                    std::mem::swap(
                        &mut self.tiles[current_pos.0][current_pos.1],
                        &mut MapTile::PLAYER(Direction::DOWN),
                    );
                    break;
                }
                MapTile::EMPTY => {
                    let row = &mut self.tiles[current_pos.0];
                    row.swap(current_pos.1, current_pos.1 + 1);

                    current_pos = (current_pos.0, current_pos.1 + 1);
                    self.player_location = Some(current_pos);
                }
                MapTile::PLAYER(_) => panic!("invalid state"),
            }
        }
    }

    fn move_left(&mut self, h: usize, w: usize) {
        let mut current_pos = (h, w);
        loop {
            self.found_tiles.insert(current_pos);
            if current_pos.1 == 0 {
                self.player_location = None;
                break;
            }
            let next_pos = &self.tiles[current_pos.0][current_pos.1 - 1];
            match next_pos {
                MapTile::WALL => {
                    std::mem::swap(
                        &mut self.tiles[current_pos.0][current_pos.1],
                        &mut MapTile::PLAYER(Direction::UP),
                    );
                    break;
                }
                MapTile::EMPTY => {
                    let row = &mut self.tiles[current_pos.0];
                    row.swap(current_pos.1, current_pos.1 - 1);

                    current_pos = (current_pos.0, current_pos.1 - 1);
                    self.player_location = Some(current_pos);
                }
                MapTile::PLAYER(_) => panic!("invalid state"),
            }
        }
    }

    fn is_cyclic(&mut self) -> bool {
        loop {
            match self.player_location {
                Some((h, w)) => {
                    let player = &self.tiles[h][w];
                    if let MapTile::PLAYER(d) = player {
                        if self.found_turns.contains(&(h, w, d.clone())) {
                            return true;
                        }
                        self.found_turns.insert((h, w, d.clone()));
                        match d {
                            Direction::UP => self.move_up(h, w),
                            Direction::DOWN => self.move_down(h, w),
                            Direction::RIGHT => self.move_right(h, w),
                            Direction::LEFT => self.move_left(h, w),
                        }
                    }
                }
                None => break,
            }
        }
        false
    }
}

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./resources/aoc2024/inputs/day6.txt")?;

    let output = Map::find_cycles(input);
    println!("output2: {output}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn move_up() {
        let mut map = Map::new(INPUT.to_string());
        map.move_up(6, 4);

        // 1, 4
        assert_eq!(Some((1, 4)), map.player_location);
        assert_eq!(&MapTile::PLAYER(Direction::RIGHT), &map.tiles[1][4]);
    }

    #[test]
    fn move_down() {
        let down_input: &str = r#"
....#.....
........v#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#..."#;
        let mut map = Map::new(down_input.to_string());
        map.move_down(1, 8);

        // 1, 4
        assert_eq!(Some((6, 8)), map.player_location);
        assert_eq!(&MapTile::PLAYER(Direction::LEFT), &map.tiles[6][8]);
    }

    #[test]
    fn move_right() {
        let right_input: &str = r#"
....#.....
....>....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#..."#;
        let mut map = Map::new(right_input.to_string());
        map.move_right(1, 4);

        // 1, 4
        assert_eq!(Some((1, 8)), map.player_location);
        assert_eq!(&MapTile::PLAYER(Direction::DOWN), &map.tiles[1][8]);
    }

    #[test]
    fn move_left() {
        let left_input: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#......<.
........#.
#.........
......#..."#;
        let mut map = Map::new(left_input.to_string());
        map.move_left(6, 8);

        // 1, 4
        assert_eq!(Some((6, 2)), map.player_location);
        assert_eq!(&MapTile::PLAYER(Direction::UP), &map.tiles[6][2]);
    }

    #[test]
    fn test() {
        let mut map = Map::new(INPUT.to_string());
        let output = map.process_movement();
        assert_eq!(41, output)
    }

    #[test]
    fn find_cycles() {
        let o = Map::find_cycles(INPUT.to_string());
        assert_eq!(6, o)
    }
}
