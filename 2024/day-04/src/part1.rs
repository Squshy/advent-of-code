#[derive(Eq, PartialEq, Debug)]
enum XmasLetter {
    X,
    M,
    A,
    S,
    Unknown,
}

impl From<char> for XmasLetter {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
struct Position {
    col: usize,
    row: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { col, row }
    }
}

struct Puzzle {
    width: usize,
    height: usize,
    lines: Vec<Vec<XmasLetter>>,
}

impl Puzzle {
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

    fn is_letter_at_pos(&self, pos: &Position, letter: &XmasLetter) -> bool {
        match self.lines.get(pos.row).and_then(|line| line.get(pos.col)) {
            Some(l) => letter == l,
            None => false,
        }
    }

    fn do_positions_spell_xmas(&self, positions: &[&Position]) -> bool {
        assert!(positions.len() == 3);
        self.is_letter_at_pos(positions[0], &XmasLetter::M)
            && self.is_letter_at_pos(positions[1], &XmasLetter::A)
            && self.is_letter_at_pos(positions[2], &XmasLetter::S)
    }

    fn is_pos_valid_to_left(&self, pos: &Position) -> bool {
        pos.col >= 3
    }

    fn is_pos_valid_to_right(&self, pos: &Position) -> bool {
        pos.col + 3 < self.width
    }

    fn is_pos_valid_to_top(&self, pos: &Position) -> bool {
        pos.row >= 3
    }

    fn is_pos_valid_to_bottom(&self, pos: &Position) -> bool {
        pos.row < self.height
    }

    /// Expected to only be called on an `X` letter
    fn count_diagonal(&self, pos: &Position) -> u32 {
        let mut count = 0;

        // Check down
        if self.is_pos_valid_to_bottom(pos) {
            if self.is_pos_valid_to_right(pos)
                && self.do_positions_spell_xmas(&[
                    &Position::new(pos.row + 1, pos.col + 1),
                    &Position::new(pos.row + 2, pos.col + 2),
                    &Position::new(pos.row + 3, pos.col + 3),
                ])
            {
                count += 1;
            }

            if self.is_pos_valid_to_left(pos)
                && self.do_positions_spell_xmas(&[
                    &Position::new(pos.row + 1, pos.col - 1),
                    &Position::new(pos.row + 2, pos.col - 2),
                    &Position::new(pos.row + 3, pos.col - 3),
                ])
            {
                count += 1;
            }
        }

        // Check up
        if self.is_pos_valid_to_top(pos) {
            if self.is_pos_valid_to_right(pos)
                && self.do_positions_spell_xmas(&[
                    &Position::new(pos.row - 1, pos.col + 1),
                    &Position::new(pos.row - 2, pos.col + 2),
                    &Position::new(pos.row - 3, pos.col + 3),
                ])
            {
                count += 1;
            }

            if self.is_pos_valid_to_left(pos)
                && self.do_positions_spell_xmas(&[
                    &Position::new(pos.row - 1, pos.col - 1),
                    &Position::new(pos.row - 2, pos.col - 2),
                    &Position::new(pos.row - 3, pos.col - 3),
                ])
            {
                count += 1;
            }
        }

        count
    }

    fn count_horizontal(&self, pos: &Position) -> u32 {
        let mut count = 0;

        // To the right
        if self.is_pos_valid_to_right(pos)
            && self.do_positions_spell_xmas(&[
                &Position::new(pos.row, pos.col + 1),
                &Position::new(pos.row, pos.col + 2),
                &Position::new(pos.row, pos.col + 3),
            ])
        {
            count += 1;
        }

        // To the left
        if self.is_pos_valid_to_left(pos)
            && self.do_positions_spell_xmas(&[
                &Position::new(pos.row, pos.col - 1),
                &Position::new(pos.row, pos.col - 2),
                &Position::new(pos.row, pos.col - 3),
            ])
        {
            count += 1;
        }

        count
    }

    fn count_vertical(&self, pos: &Position) -> u32 {
        let mut count = 0;

        // Up
        if self.is_pos_valid_to_top(&pos)
            && self.do_positions_spell_xmas(&[
                &Position::new(pos.row - 1, pos.col),
                &Position::new(pos.row - 2, pos.col),
                &Position::new(pos.row - 3, pos.col),
            ])
        {
            count += 1;
        }

        // Down
        if self.is_pos_valid_to_bottom(pos)
            && self.do_positions_spell_xmas(&[
                &Position::new(pos.row + 1, pos.col),
                &Position::new(pos.row + 2, pos.col),
                &Position::new(pos.row + 3, pos.col),
            ])
        {
            count += 1;
        }

        count
    }
}

pub fn solve() -> u32 {
    let puzzle = Puzzle::new(include_str!("../input/part1.txt"));

    let mut xmas_count = 0;
    for (line_num, line) in puzzle.lines.iter().enumerate() {
        for (letter_num, letter) in line.iter().enumerate() {
            match letter {
                XmasLetter::X => {
                    let pos = Position::new(line_num, letter_num);
                    xmas_count += puzzle.count_diagonal(&pos);
                    xmas_count += puzzle.count_horizontal(&pos);
                    xmas_count += puzzle.count_vertical(&pos);
                }
                _ => {}
            }
        }
    }

    xmas_count
}

#[cfg(test)]
mod tests {
    // These tests are not good and complete, only test very happy paths
    use crate::part1::Position;

    use super::Puzzle;

    #[test]
    fn diagonal_works() {
        let input = r#"S.....S
.A...A.
..M.M..
...X...
..M.M..
.A...A.
S.....S"#;
        let count = Puzzle::new(input).count_diagonal(&Position::new(3, 3));
        assert_eq!(count, 4);
    }

    #[test]
    fn horizontal_works() {
        let input = r#"SAMXMAS"#;
        let count = Puzzle::new(input).count_horizontal(&Position::new(0, 3));
        assert_eq!(count, 2);
    }

    #[test]
    fn vertical_works() {
        let input = r#"S
A
M
X
M
A
S"#;
        let count = Puzzle::new(input).count_vertical(&Position::new(3, 0));
        assert_eq!(count, 2);
    }

    #[test]
    fn all_work_together() {
        let input = r#"S..S..S
.A.A.A.
..MMM..
SAMXMAS
..MMM..
.A.A.A.
S..S..S"#;
        let mut count = 0;
        let puzzle = Puzzle::new(input);
        let pos = Position::new(3, 3);
        count += puzzle.count_diagonal(&pos);
        count += puzzle.count_horizontal(&pos);
        count += puzzle.count_vertical(&pos);
        assert_eq!(count, 4 + 2 + 2);
    }
}
