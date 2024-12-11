#[derive(Eq, PartialEq, Debug)]
enum Rule {
    Replace,
    Split,
    Multiply,
}

#[derive(Debug, Eq, PartialEq)]
struct Stone(u64);

impl TryFrom<&str> for Stone {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(num) = u64::from_str_radix(value, 10).ok() {
            Ok(Self(num))
        } else {
            Err("Invalid num")
        }
    }
}

impl Stone {
    fn new(value: u64) -> Self {
        Self(value)
    }

    fn rule(&self) -> Rule {
        if self.0 == 0 {
            Rule::Replace
        } else if self.0.to_string().len() % 2 == 0 {
            Rule::Split
        } else {
            Rule::Multiply
        }
    }

    fn split(&self) -> (Self, Self) {
        let str = self.0.to_string();
        let parts = str.split_at(str.len() / 2);
        (
            Self::try_from(parts.0).unwrap(),
            Self::try_from(parts.1).unwrap(),
        )
    }

    fn multiply(&self) -> Self {
        Self(self.0 * 2024)
    }

    fn replace(&self) -> Self {
        Self(1)
    }
}

fn parse_stones(input: &str) -> Vec<Stone> {
    input
        .split(" ")
        .take_while(|num| !num.trim().is_empty())
        .filter_map(|num| Stone::try_from(num).ok())
        .collect()
}

fn blink_stones(stones: &mut Vec<Stone>) {
    let mut idx = 0;
    while idx < stones.len() {
        match stones[idx].rule() {
            Rule::Multiply => stones[idx] = stones[idx].multiply(),
            Rule::Replace => stones[idx] = stones[idx].replace(),
            Rule::Split => {
                let (first, second) = stones[idx].split();
                stones[idx] = first;
                stones.insert(idx + 1, second);
                idx += 1;
            }
        }
        idx += 1;
    }
}

fn count_blinking_stones(input: &str, n: u8) -> u64 {
    let mut stones = parse_stones(input);

    for _ in 0..n {
        blink_stones(&mut stones);
    }

    stones.len() as u64
}

pub fn solve() -> u64 {
    count_blinking_stones(include_str!("../input/part1.txt"), 25)
}

#[cfg(test)]
mod tests {
    use crate::part1::blink_stones;

    use super::*;

    #[test]
    fn stone_rule() {
        assert_eq!(Stone::new(0).rule(), Rule::Replace);
        assert_eq!(Stone::new(10).rule(), Rule::Split);
        assert_eq!(Stone::new(1).rule(), Rule::Multiply);
    }

    #[test]
    fn stone_split() {
        assert_eq!(Stone::new(10).split(), (Stone::new(1), Stone::new(0)));
        assert_eq!(Stone::new(1000).split(), (Stone::new(10), Stone::new(0)));
    }

    #[test]
    fn test_blink_stones() {
        let mut stones = parse_stones("0 1 10 99 999");
        blink_stones(&mut stones);
        assert_eq!(
            stones,
            vec![
                Stone::new(1),
                Stone::new(2024),
                Stone::new(1),
                Stone::new(0),
                Stone::new(9),
                Stone::new(9),
                Stone::new(2021976),
            ]
        );
    }

    #[test]
    fn example() {
        assert_eq!(count_blinking_stones("125 17", 25), 55312);
    }
}
