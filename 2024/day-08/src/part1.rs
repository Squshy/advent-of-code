use std::collections::{HashMap, HashSet};

type Frequency = char;

#[derive(Debug, Eq, PartialEq)]
enum MapObject {
    Antenna(Frequency),
    Empty,
}

impl From<char> for MapObject {
    fn from(value: char) -> Self {
        match value {
            '0'..='9' | 'A'..='Z' | 'a'..='z' => Self::Antenna(value),
            _ => Self::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn difference(&self, other: &Self) -> Self {
        Self {
            row: other.row - self.row,
            col: other.col - self.col,
        }
    }
}

#[derive(Debug)]
struct AntennaGrid {
    width: usize,
    height: usize,
    items: Vec<Vec<MapObject>>,
    freq_positions: HashMap<Frequency, Vec<Position>>,
}

impl From<&str> for AntennaGrid {
    fn from(value: &str) -> Self {
        let mut freq_positions = HashMap::new();
        let items = value
            .lines()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .into_iter()
                    .enumerate()
                    .map(|(col, freq)| {
                        let entry = freq_positions.entry(freq).or_insert(Vec::new());
                        entry.push(Position::new(row as isize, col as isize));

                        MapObject::from(freq)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            width: items.first().map_or(0, |line| line.len()),
            height: items.len(),
            items,
            freq_positions,
        }
    }
}

impl AntennaGrid {
    fn contains_position(&self, pos: &Position) -> bool {
        pos.row >= 0
            && (pos.row as usize) < self.height
            && pos.col >= 0
            && (pos.col as usize) < self.width
    }

    fn unique_antinode_count(&self) -> u32 {
        let mut antinodes: HashSet<Position> = HashSet::new();

        for row in 0..self.height {
            for col in 0..self.width {
                match self.items[row][col] {
                    MapObject::Antenna(freq) => {
                        let positions = self.freq_positions.get(&freq);

                        if positions.is_none() {
                            continue;
                        }

                        let freq_pos = Position::new(row as isize, col as isize);

                        for pos in positions.unwrap() {
                            if pos == &freq_pos {
                                continue;
                            }
                            let diff = freq_pos.difference(pos);
                            let up = Position::new(pos.row + diff.row, pos.col + diff.col);
                            let down =
                                Position::new(freq_pos.row - diff.row, freq_pos.col - diff.col);

                            if self.contains_position(&up) {
                                antinodes.insert(up);
                            }

                            if self.contains_position(&down) {
                                antinodes.insert(down);
                            }
                        }
                    }
                    _ => {}
                };
            }
        }

        antinodes.len() as u32
    }
}

pub fn solve() -> u32 {
    AntennaGrid::from(include_str!("../input/part1.txt")).unique_antinode_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
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

        assert_eq!(AntennaGrid::from(input).unique_antinode_count(), 14);
    }
}
