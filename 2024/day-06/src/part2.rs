use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_90_deg(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Eq, PartialEq)]
enum GridObject {
    Guard,
    Obstruction,
    Empty,
}

impl From<char> for GridObject {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Obstruction,
            '^' => Self::Guard,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
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

struct PuzzleMap {
    height: usize,
    width: usize,
    start_pos: Position,
    items: Vec<Vec<GridObject>>,
}

impl PuzzleMap {
    fn new(str: &str) -> Self {
        let mut start_pos = None;
        let items = str
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        let obj = GridObject::from(c);
                        if obj == GridObject::Guard {
                            start_pos = Some(Position::new(row as isize, col as isize));
                        };
                        obj
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            width: items.first().unwrap().len(),
            height: items.len(),
            start_pos: start_pos.unwrap(),
            items,
        }
    }

    fn contains_position(&self, pos: &Position) -> bool {
        pos.row >= 0
            && (pos.row as usize) < self.height
            && pos.col >= 0
            && (pos.col as usize) < self.width
    }

    fn try_get(&self, pos: &Position) -> Option<&GridObject> {
        match self.contains_position(pos) {
            true => self
                .items
                .get(pos.row as usize)
                .map_or(None, |line| line.get(pos.col as usize)),
            false => None,
        }
    }

    fn guard_path(&self) -> HashSet<Position> {
        let mut direction = Direction::Up;
        let mut current_pos = self.start_pos.move_in_direction(&direction);
        let mut seen_pos: HashSet<Position> = HashSet::new();

        while self.contains_position(&current_pos) {
            seen_pos.insert(current_pos);

            loop {
                let new_pos = current_pos.move_in_direction(&direction);
                match self.try_get(&new_pos) {
                    Some(GridObject::Obstruction) => {
                        direction = direction.turn_90_deg();
                    }
                    _ => {
                        current_pos = new_pos;
                        break;
                    }
                }
            }
        }

        seen_pos
    }

    fn does_guard_loop(&self, obs_pos: &Position) -> bool {
        let mut direction = Direction::Up;
        let mut current_pos = self.start_pos.clone();
        let mut seen_pos_with_dir: HashSet<(Position, Direction)> = HashSet::new();

        while let Some(_) = self.try_get(&current_pos) {
            if !seen_pos_with_dir.insert((current_pos, direction.clone())) {
                return true;
            }

            loop {
                let new_pos = current_pos.move_in_direction(&direction);
                match self.try_get(&new_pos) {
                    Some(GridObject::Obstruction) => {
                        direction = direction.turn_90_deg();
                    }
                    _ => {
                        if &new_pos == obs_pos {
                            direction = direction.turn_90_deg();
                        } else {
                            current_pos = new_pos;
                            break;
                        }
                    }
                }
            }
        }

        false
    }

    fn count_guard_loops(&self) -> u32 {
        // We know we hit a loop when we see the same position in a direction
        // 90deg from our current direction.
        //
        // It is not enough for us to only check this, we need to simulate what
        // would happen if an empty spot became an obstruction as well.

        self.guard_path()
            .iter()
            .filter(|step| {
                self.try_get(step) == Some(&GridObject::Empty) && self.does_guard_loop(step)
            })
            .count() as u32
    }
}

pub fn solve() -> u32 {
    PuzzleMap::new(include_str!("../input/part1.txt")).count_guard_loops()
}

#[cfg(test)]
mod tests {
    use super::PuzzleMap;

    #[test]
    fn test_sample() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let puzzle = PuzzleMap::new(input);
        assert_eq!(puzzle.count_guard_loops(), 6);
    }
}
