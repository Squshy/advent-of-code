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
        let mut area_count = 0;
        let mut perimiter_count = 0;
        queue.push_front(pos);

        while let Some(pos) = queue.pop_front() {
            match self.get(&pos) {
                Some(new_plot) if plot == new_plot => {}
                _ => continue,
            };

            if !seen.insert(pos) {
                continue;
            };

            let touch_count: u64 = Direction::iter()
                .map(|dir| {
                    if Some(plot) != self.get(&pos.move_in_direction(&dir)) {
                        1
                    } else {
                        0
                    }
                })
                .sum();

            perimiter_count += touch_count;
            area_count += 1;

            for dir in Direction::iter() {
                queue.push_back(pos.move_in_direction(&dir));
            }
        }

        (perimiter_count * area_count) as u64
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

        assert_eq!(Garden::from(input).calculate_fence_price(), 1930);
    }
}
