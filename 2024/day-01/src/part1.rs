use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve() -> u32 {
    let contents = include_str!("../input/part1.txt");
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    contents
        .split("\n")
        .into_iter()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let x: Vec<u32> = line
                .split("  ")
                .take_while(|line| !line.is_empty())
                .take(2)
                .map(|num| u32::from_str_radix(num.trim(), 10).unwrap())
                .collect();

            left.push(Reverse(*x.first().unwrap()));
            right.push(Reverse(*x.last().unwrap()));
        });

    assert!(left.len() == right.len());

    let mut total = 0;
    while left.len() > 0 {
        total += left.pop().unwrap().0.abs_diff(right.pop().unwrap().0)
    }

    total
}
