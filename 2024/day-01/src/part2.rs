use std::collections::HashMap;

pub fn solve() -> u32 {
    include_str!("../input/part1.txt")
        .lines()
        .take_while(|line| !line.is_empty())
        .fold(HashMap::new(), |mut acc, line| {
            if let Some((left, right)) = line.split_once("  ") {
                let left = u32::from_str_radix(left.trim(), 10).unwrap();
                let right = u32::from_str_radix(right.trim(), 10).unwrap();

                acc.entry(left)
                    .and_modify(|(l, _)| {
                        *l += 1;
                    })
                    .or_insert((1u32, 0u32));

                acc.entry(right)
                    .and_modify(|(_, r)| {
                        *r += 1;
                    })
                    .or_insert((0u32, 1u32));

                acc
            } else {
                unreachable!()
            }
        })
        .iter()
        .map(|(key, (left, right))| left * right * key)
        .sum()
}
