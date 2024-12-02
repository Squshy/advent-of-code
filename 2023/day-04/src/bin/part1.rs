use std::collections::HashSet;

#[derive(Debug)]
struct ScratchCard {
    #[allow(dead_code)]
    id: usize,
    winning_numbers: HashSet<u32>,
    scratched_numbers: HashSet<u32>,
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let (id_str, rest) = value.split_once(':').unwrap();
        let id = id_str
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();

        let (win_num_str, scratch_num_str) = rest.split_once('|').unwrap();
        let winning_numbers = win_num_str
            .trim()
            .split(' ')
            .into_iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<HashSet<u32>>();
        let scratched_numbers = scratch_num_str
            .trim()
            .split(' ')
            .into_iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<HashSet<u32>>();

        Self {
            id,
            winning_numbers,
            scratched_numbers,
        }
    }
}

impl ScratchCard {
    fn calc_points(&self) -> u32 {
        let mut num_matches = 0;

        for winning_num in &self.winning_numbers {
            if self.scratched_numbers.contains(&winning_num) {
                num_matches += std::cmp::max(num_matches, 1);
            }
        }

        num_matches
    }
}

fn process(str: String) -> u32 {
    str.lines()
        .into_iter()
        .map(|line| ScratchCard::from(line).calc_points())
        .sum::<u32>()
}

#[allow(dead_code)]
fn funky_one() -> usize {
    let input = include_bytes!("../../data/input.txt");
    let col = input.iter().position(|&b| b == b':').unwrap();
    let sep = input.iter().position(|&b| b == b'|').unwrap();

    let total = input
        .split(|&b| b == b'\n')
        .map(|game| {
            let win_seq = &game[col + 1..sep];
            let win_count = game[sep + 1..]
                .chunks_exact(3)
                .map(|n| &n[1..])
                .filter(|n| win_seq.chunks_exact(3).map(|n| &n[1..]).any(|c| &c == n))
                .count() as u32;
            2usize.pow(win_count) >> 1
        })
        .sum::<usize>();

    total
}

fn main() {
    let total = process(std::fs::read_to_string("./data/input.txt").unwrap());

    println!("TOTAL: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .to_string();

        assert_eq!(13, process(input));
    }

    #[test]
    fn funky() {
        let total = process(std::fs::read_to_string("./data/input.txt").unwrap());
        assert_eq!(total, funky_one() as u32);
    }
}
