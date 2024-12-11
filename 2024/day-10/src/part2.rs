use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn move_in_dir(&self, dir: &Direction) -> Self {
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

#[derive(Clone, Copy)]
struct Height(u8);

impl From<char> for Height {
    fn from(value: char) -> Self {
        Self(value.to_digit(10).unwrap() as u8)
    }
}

impl Height {
    fn is_trailhead(&self) -> bool {
        self.0 == 0
    }
}

struct LavaMap {
    height: usize,
    width: usize,
    heights: Vec<Vec<Height>>,
}

impl From<&str> for LavaMap {
    fn from(value: &str) -> Self {
        let heights = value
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().map(Height::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            height: heights.len(),
            width: heights[0].len(),
            heights,
        }
    }
}

impl LavaMap {
    fn contains_position(&self, pos: &Position) -> bool {
        pos.row >= 0
            && (pos.row as usize) < self.height
            && pos.col >= 0
            && (pos.col as usize) < self.width
    }

    fn try_get(&self, pos: &Position) -> Option<&Height> {
        match self.contains_position(pos) {
            true => self
                .heights
                .get(pos.row as usize)
                .map_or(None, |line| line.get(pos.col as usize)),
            false => None,
        }
    }

    fn score_trailhead(&self, pos: &Position) -> u32 {
        Direction::iter()
            .map(|dir| self.rec(pos.move_in_dir(&dir), 1))
            .sum()
    }

    fn rec(&self, pos: Position, h: u8) -> u32 {
        let height = match self.try_get(&pos) {
            Some(height) => height,
            None => return 0,
        };

        if height.0 != h {
            return 0;
        };

        if height.0 == 9 {
            return 1;
        };

        Direction::iter()
            .map(|dir| self.rec(pos.move_in_dir(&dir), height.0 + 1))
            .sum()
    }

    fn total_trailhead_score(&self) -> u32 {
        let mut score = 0;

        for row in 0..self.height {
            for col in 0..self.width {
                let pos = Position::new(row as isize, col as isize);
                if let Some(height) = self.try_get(&pos) {
                    if height.is_trailhead() {
                        let s = self.score_trailhead(&pos);
                        score += s;
                    }
                }
            }
        }

        score
    }
}

pub fn solve() -> u32 {
    LavaMap::from(include_str!("../input/part1.txt")).total_trailhead_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let map = LavaMap::from(input);
        assert_eq!(map.score_trailhead(&Position::new(0, 2)), 20);
        assert_eq!(map.total_trailhead_score(), 81);
    }
}
