use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Copy, Clone, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn move_in_direction(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Self {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Self {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Self {
                row: self.row,
                col: self.col + 1,
            },
        }
    }

    /// Ordered as the position diagonal from self followed by two adjacent
    /// positions
    fn corners(&self) -> [[Position; 3]; 4] {
        [
            [
                Position::new(self.row - 1, self.col - 1),
                Position::new(self.row - 1, self.col),
                Position::new(self.row, self.col - 1),
            ],
            [
                Position::new(self.row + 1, self.col - 1),
                Position::new(self.row, self.col - 1),
                Position::new(self.row + 1, self.col),
            ],
            [
                Position::new(self.row + 1, self.col + 1),
                Position::new(self.row + 1, self.col),
                Position::new(self.row, self.col + 1),
            ],
            [
                Position::new(self.row - 1, self.col + 1),
                Position::new(self.row, self.col + 1),
                Position::new(self.row - 1, self.col),
            ],
        ]
    }
}

#[derive(Eq, PartialEq, Debug)]
struct GardenPlot(char);

impl TryFrom<char> for GardenPlot {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A'..='Z' => Ok(Self(value)),
            _ => Err("Character is not an uppercase letter"),
        }
    }
}

struct Garden {
    height: usize,
    width: usize,
    plots: Vec<Vec<GardenPlot>>,
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        let plots: Vec<Vec<GardenPlot>> = value
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .filter_map(|c| GardenPlot::try_from(c).ok())
                    .collect()
            })
            .collect();

        Self {
            height: plots.len(),
            width: plots.first().unwrap().len(),
            plots,
        }
    }
}

impl Garden {
    fn get(&self, pos: &Position) -> Option<&GardenPlot> {
        self.plots.get(pos.row as usize)?.get(pos.col as usize)
    }

    fn region_price(&self, plot: &GardenPlot, pos: Position, seen: &mut HashSet<Position>) -> u64 {
        let mut queue: VecDeque<Position> = VecDeque::new();
        let mut region: HashSet<Position> = HashSet::new();
        queue.push_front(pos);

        while let Some(pos) = queue.pop_front() {
            match self.get(&pos) {
                Some(new_plot) if plot == new_plot => {}
                _ => continue,
            };

            if !seen.insert(pos) {
                continue;
            };
            region.insert(pos);

            Direction::iter().for_each(|dir| {
                let new_pos = pos.move_in_direction(&dir);
                queue.push_back(new_pos);
            });
        }

        (region.len() as u64)
            * region
                .iter()
                .map(|pos| {
                    pos.corners()
                        .iter()
                        .filter(|positions| {
                            let diagonal = self.get(&positions[0]);
                            let first = self.get(&positions[1]);
                            let second = self.get(&positions[2]);

                            // If the two sides of us are different then we hit
                            // a corner.
                            // It's also a corner if we are the same as our sides
                            // but the diagonal is different
                            // AA    AB
                            // AB    BB
                            Some(plot) != first && Some(plot) != second
                                || Some(plot) == second
                                    && Some(plot) == first
                                    && Some(plot) != diagonal
                        })
                        .count()
                })
                .sum::<usize>() as u64
    }

    fn calculate_fence_price(&self) -> u64 {
        let mut seen_plots: HashSet<Position> = HashSet::new();
        let mut total = 0u64;

        for row in 0..self.height {
            for col in 0..self.width {
                let pos = Position::new(row as isize, col as isize);
                if let Some(plot) = self.get(&pos) {
                    total += self.region_price(plot, pos, &mut seen_plots);
                }
            }
        }

        total
    }
}

pub fn solve() -> u64 {
    Garden::from(include_str!("../input/part1.txt")).calculate_fence_price()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        assert_eq!(Garden::from(input).calculate_fence_price(), 1206);
    }
}
