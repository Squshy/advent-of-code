use itertools::{Itertools, Position};

fn main() {
    let input = include_bytes!("../../data/input.txt");
    let val = input
        .split(|&b| b == b'\n')
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            // Get our list of values which are separated by white space
            // Use an i32 because there can be negative numbers
            let mut vals = line
                .split(|&b| b == b' ')
                .filter_map(atoi::atoi::<i32>)
                .collect::<Vec<_>>();

            let mut start_numbers: Vec<i32> = vec![];

            loop {
                if vals.iter().all(|n| n == &0) {
                    break;
                }

                vals = vals
                    .iter()
                    .tuple_windows()
                    .with_position()
                    .map(|(pos, (l, r))| {
                        // If it is the first or the only element in the iterator
                        // we will add it to our initial numbers
                        match pos {
                            Position::First | Position::Only => {
                                start_numbers.push(*l);
                            }
                            _ => {}
                        };

                        r - l
                    })
                    .collect::<Vec<i32>>();
            }

            // Reduce over all of our numbers
            start_numbers.iter().rev().fold(0, |acc, num| num - acc)
        })
        .sum::<i32>();

    println!("Total: {}", val);
}
