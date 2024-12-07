use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Operation {
    Multiply,
    Add,
}

impl Operation {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            &Self::Multiply => left * right,
            &Self::Add => left + right,
        }
    }
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (test_value, numbers) = value.split_once(":").unwrap();
        let test_value = u64::from_str_radix(test_value, 10).unwrap();
        let numbers = numbers
            .trim()
            .split(" ")
            .take_while(|str| !str.is_empty())
            .map(|num| u64::from_str_radix(num, 10).unwrap())
            .collect::<Vec<_>>();

        Self {
            test_value,
            numbers,
        }
    }
}

impl Equation {
    fn is_valid(&self) -> bool {
        println!("{:?}", self);
        if self.numbers.len() == 0 {
            return false;
        }

        if self.numbers.len() == 1 && self.numbers[0] == self.test_value {
            return true;
        }

        for op in Operation::iter() {
            if self.meme(op.apply(self.numbers[0], self.numbers[1]), 2) {
                return true;
            }
        }

        false
    }

    fn meme(&self, tally: u64, index: usize) -> bool {
        if index >= self.numbers.len() {
            return tally == self.test_value;
        }

        let value = self.numbers.get(index).unwrap();
        for op in Operation::iter() {
            if self.meme(op.apply(tally, *value), index + 1) {
                return true;
            }
        }

        false
    }
}

fn xd(input: &str) -> u64 {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(Equation::from)
        .map(|eq| if eq.is_valid() { eq.test_value } else { 0 })
        .sum()
}

pub fn solve() -> u64 {
    xd(include_str!("../input/part1.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(xd(input), 3749);
    }
}
