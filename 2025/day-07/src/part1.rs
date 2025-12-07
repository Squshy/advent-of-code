use utils::{Direction, Grid, Position};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Object {
    Splitter,
    Empty,
    Start,
    Beam,
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Splitter,
            '.' => Self::Empty,
            'S' => Self::Start,
            '|' => Self::Beam,
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

    grid.set(pos.as_ref().expect("a starting pos"), Object::Beam);
    let mut num_splits = 0;

    for y in 2..grid.height() {
        for x in 0..grid.width() {
            let pos = Position::new(x, y);

            let cur = grid.get(&pos).expect("an obj");
            let above = grid
                .get(&pos.new_in_dir(Direction::Up).expect("a valid pos"))
                .expect("an obj");

            if matches!(above, Object::Beam) {
                if matches!(cur, Object::Splitter) {
                    pos.new_in_dir(Direction::Left)
                        .and_then(|pos| grid.set(&pos, Object::Beam));
                    pos.new_in_dir(Direction::Right)
                        .and_then(|pos| grid.set(&pos, Object::Beam));
                    num_splits += 1;
                } else {
                    grid.set(&pos, Object::Beam);
                }
            }
        }
    }

    num_splits
}
