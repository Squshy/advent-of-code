use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum Rule {
    Replace,
    Split,
    Multiply,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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

fn blink_stone_n(stone: &Stone, n: u32, map: &mut HashMap<(Stone, u32), u64>) -> u64 {
    if n == 0 {
        return 1;
    }

    if let Some(stone_count) = map.get(&(*stone, n)) {
        return *stone_count;
    }

    let count = match stone.rule() {
        Rule::Split => {
            let (f, s) = stone.split();
            blink_stone_n(&f, n - 1, map) + blink_stone_n(&s, n - 1, map)
        }
        Rule::Multiply => blink_stone_n(&stone.multiply(), n - 1, map),
        Rule::Replace => blink_stone_n(&stone.replace(), n - 1, map),
    };

    map.insert((*stone, n), count);

    count
}

fn count_blinking_stones(input: &str, n: u32) -> u64 {
    let mut stone_blinks_to_output: HashMap<(Stone, u32), u64> = HashMap::new();

    parse_stones(input)
        .iter()
        .map(|stone| blink_stone_n(stone, n, &mut stone_blinks_to_output))
        .sum()
}

pub fn solve() -> u64 {
    count_blinking_stones(include_str!("../input/part1.txt"), 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(count_blinking_stones("125 17", 1), 3);
        assert_eq!(count_blinking_stones("125 17", 2), 4);
        assert_eq!(count_blinking_stones("125 17", 3), 5);
        assert_eq!(count_blinking_stones("125 17", 4), 9);
        assert_eq!(count_blinking_stones("125 17", 5), 13);
        assert_eq!(count_blinking_stones("125 17", 6), 22);
        assert_eq!(count_blinking_stones("125 17", 25), 55312);
    }
}
