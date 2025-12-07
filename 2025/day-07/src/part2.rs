use std::fmt::Debug;

use utils::{Direction, Grid, Position};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Object {
    Splitter,
    Empty,
    Start,
    Beam(usize),
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Empty => ".".to_string(),
                Object::Start => "S".to_string(),
                Object::Splitter => "^".to_string(),
                Object::Beam(c) => c.to_string(),
            }
        )
    }
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Splitter,
            '.' => Self::Empty,
            'S' => Self::Start,
            '|' => Self::Beam(1),
            _ => panic!("Invalid object"),
        }
    }
}

pub fn solve() -> usize {
    let mut grid: Grid<Object> = Grid::from(include_str!("../input/input.txt"));
    let pos = grid
        .iter_with_coords()
        .find(|(_, i)| **i == Object::Start)
        .map(|(c, _)| c)
        .and_then(|c| c.new_in_dir(Direction::Down));

    grid.set(pos.as_ref().expect("a starting pos"), Object::Beam(1));

    for y in 2..grid.height() {
        for x in 0..grid.width() {
            let pos = Position::new(x, y);

            let cur = *grid.get(&pos).expect("an obj");
            let above = *grid
                .get(&pos.new_in_dir(Direction::Up).expect("a valid pos"))
                .expect("an obj");

            if let Object::Beam(count) = above {
                match cur {
                    Object::Splitter => {
                        if let Some(left) = pos.new_in_dir(Direction::Left) {
                            if let Some(Object::Beam(c)) = grid.get(&left) {
                                grid.set(&left, Object::Beam(c + count));
                            } else {
                                grid.set(&left, Object::Beam(count));
                            }
                        }

                        if let Some(right) = pos.new_in_dir(Direction::Right) {
                            if let Some(Object::Beam(c)) = grid.get(&right) {
                                grid.set(&right, Object::Beam(c + count));
                            } else {
                                grid.set(&right, Object::Beam(count));
                            }
                        }
                    }
                    Object::Beam(c) => {
                        grid.set(&pos, Object::Beam(count + c));
                    }
                    _ => {
                        grid.set(&pos, Object::Beam(count));
                    }
                }
            }
        }
    }

    grid.row(grid.height() - 1)
        .expect("a row")
        .iter()
        .map(|o| if let Object::Beam(c) = *o { c } else { 0 })
        .sum()
}
