#[derive(Eq, PartialEq, Debug)]
enum XmasLetter {
    M,
    A,
    S,
    Unknown,
}

impl From<char> for XmasLetter {
    fn from(value: char) -> Self {
        match value {
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::Unknown,
        }
    }
}

#[derive(Eq, PartialEq)]
enum Direction {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug)]
struct Position {
    col: i32,
    row: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self { col, row }
    }

    fn new_in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::NorthEast => Self {
                row: self.row - 1,
                col: self.col + 1,
            },
            Direction::NorthWest => Self {
                row: self.row - 1,
                col: self.col - 1,
            },
            Direction::SouthEast => Self {
                col: self.col + 1,
                row: self.row + 1,
            },
            Direction::SouthWest => Self {
                col: self.col - 1,
                row: self.row + 1,
            },
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    lines: Vec<Vec<XmasLetter>>,
}

impl Grid {
    fn new(str: &str) -> Self {
        let lines = str
            .lines()
            .map(|line| line.chars().map(XmasLetter::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            width: lines.first().unwrap().len(),
            height: lines.len(),
            lines,
        }
    }

    fn contains_position(&self, pos: &Position) -> bool {
        pos.row >= 0
            && (pos.row as usize) < self.height
            && pos.col >= 0
            && (pos.col as usize) < self.width
    }

    fn try_get(&self, pos: &Position) -> Option<&XmasLetter> {
        match self.contains_position(pos) {
            true => self
                .lines
                .get(pos.row as usize)
                .map_or(None, |line| line.get(pos.col as usize)),
            false => None,
        }
    }

    fn is_xmas(&self, pos: &Position) -> bool {
        if let (Some(ne), Some(sw), Some(nw), Some(se)) = (
            self.try_get(&pos.new_in_direction(&Direction::NorthEast)),
            self.try_get(&pos.new_in_direction(&Direction::SouthWest)),
            self.try_get(&pos.new_in_direction(&Direction::NorthWest)),
            self.try_get(&pos.new_in_direction(&Direction::SouthEast)),
        ) {
            match (ne, sw, nw, se) {
                (XmasLetter::M, XmasLetter::S, XmasLetter::M, XmasLetter::S)
                | (XmasLetter::M, XmasLetter::S, XmasLetter::S, XmasLetter::M)
                | (XmasLetter::S, XmasLetter::M, XmasLetter::M, XmasLetter::S)
                | (XmasLetter::S, XmasLetter::M, XmasLetter::S, XmasLetter::M) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

pub fn solve() -> u32 {
    let puzzle = Grid::new(include_str!("../input/part1.txt"));

    let mut xmas_count = 0;
    for (line_num, line) in puzzle.lines.iter().enumerate() {
        for (letter_num, letter) in line.iter().enumerate() {
            match letter {
                XmasLetter::A => {
                    if puzzle.is_xmas(&Position::new(line_num as i32, letter_num as i32)) {
                        xmas_count += 1;
                    };
                }
                _ => {}
            }
        }
    }

    xmas_count
}
