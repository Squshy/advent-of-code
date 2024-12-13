// The minimum number of presses is the inverse matrix multiplied by the
// positions. If the result is a whole number, then that is our answer.

#[inline]
fn parse_num(part: &str) -> f64 {
    let num_str = part
        .chars()
        .into_iter()
        .filter(|c| c.is_digit(10))
        .collect::<String>();

    usize::from_str_radix(&num_str, 10).unwrap() as f64
}

struct Button {
    x: f64,
    y: f64,
    token_cost: u8,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let (first, second) = value.split_once(",").unwrap();
        let token_cost = if first.contains("A") { 3 } else { 1 };
        let x = parse_num(first);
        let y = parse_num(second);
        Self { x, y, token_cost }
    }
}

struct Prize {
    x: f64,
    y: f64,
}

impl From<&str> for Prize {
    fn from(value: &str) -> Self {
        let (first, second) = value.split_once(",").unwrap();
        let x = parse_num(first) + 10000000000000f64;
        let y = parse_num(second) + 10000000000000f64;

        Self { x, y }
    }
}

struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        Self {
            button_a: Button::from(lines.next().unwrap()),
            button_b: Button::from(lines.next().unwrap()),
            prize: Prize::from(lines.next().unwrap()),
        }
    }
}

/// We can have some loss, example `39.9999999999` should probably be `40.0`,
/// so we check for these cases as well as `40.0000000001` should also be `40.0`
fn is_float_close_enough(num: f64) -> bool {
    (num - num.round()).abs() < 1e-3
}

impl Machine {
    fn inversed_button_matrix(&self) -> [[f64; 2]; 2] {
        let det =
            1f64 / ((self.button_a.x * self.button_b.y) - (self.button_a.y * self.button_b.x));

        // Determinate should not be 0
        assert!(det != 0f64);

        [
            [self.button_b.y * det, -self.button_a.y * det],
            [-self.button_b.x * det, self.button_a.x * det],
        ]
    }

    fn min_button_count(&self) -> Option<u64> {
        // We inverse the matrix and multiply that by our desired `x,y` to
        // essentially divide. This lets us see if we can press the buttons
        // enough to reach our destination. We can't half press a button, that
        // would be silly, so we only count whole numbers to count. If we have
        // any fractionals, it is not valid.
        let inversed = self.inversed_button_matrix();

        let (x, y) = (
            self.prize.x * inversed[0][0] + self.prize.y * inversed[1][0],
            self.prize.x * inversed[0][1] + self.prize.y * inversed[1][1],
        );

        if is_float_close_enough(x) && is_float_close_enough(y) {
            let a = x.round() as u64 * (self.button_a.token_cost as u64);
            let b = y.round() as u64 * (self.button_b.token_cost as u64);

            Some(a + b)
        } else {
            None
        }
    }
}

fn count_min_buttons(input: &str) -> u64 {
    input
        .split("\n\n")
        .take_while(|line| !line.is_empty())
        .filter_map(|lines| Machine::from(lines).min_button_count())
        .sum()
}

pub fn solve() -> u64 {
    count_min_buttons(include_str!("../input/part1.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let button = Button::from("Button A: X+94, Y+34");
        assert_eq!(button.x, 94f64);
        assert_eq!(button.y, 34f64);
        assert_eq!(button.token_cost, 3);
        let prize = Prize::from("Prize: X=8400, Y=5400");
        assert_eq!(prize.x, 8400f64);
        assert_eq!(prize.y, 5400f64);
        let machine = Machine::from(
            r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"#,
        );
        assert_eq!(machine.button_a.x, 94f64);
        assert_eq!(machine.button_a.y, 34f64);
        assert_eq!(machine.button_b.x, 22f64);
        assert_eq!(machine.button_b.y, 67f64);
        assert_eq!(machine.prize.x, 8400f64);
        assert_eq!(machine.prize.y, 5400f64);
    }

    #[test]
    fn first_case() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"#;

        let machine = Machine::from(input);
        assert_eq!(machine.min_button_count(), Some(280));
    }

    #[test]
    fn example() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        assert_eq!(count_min_buttons(input), 480);
    }
}
