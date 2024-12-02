use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct ScratchCard {
    pub id: u32,
    #[allow(dead_code)]
    pub winning_numbers: HashSet<u32>,
    #[allow(dead_code)]
    pub scratched_numbers: HashSet<u32>,
    pub matching_numbers: u32,
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let (id_str, rest) = value.split_once(':').unwrap();
        let id = id_str
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<u32>()
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

        let mut matching_numbers = 0;

        for winning_num in &winning_numbers {
            if scratched_numbers.contains(&winning_num) {
                matching_numbers += 1;
            }
        }

        Self {
            id,
            winning_numbers,
            scratched_numbers,
            matching_numbers,
        }
    }
}

fn process(str: String) -> u32 {
    // Maps from the card ID to the card and the number of copies of that card
    let mut map: HashMap<u32, (ScratchCard, u32)> = HashMap::new();
    let cards = str
        .lines()
        .into_iter()
        .map(|line| {
            let card = ScratchCard::from(line);
            // Idk just clone
            map.insert(card.id, (card.clone(), 1));
            card
        })
        .collect::<Vec<ScratchCard>>();

    // Need to go through every card and get its point value
    // For each point, add 1 * current copy count to that scractch cards copy count
    for card in cards {
        let my_copy_count = map.get(&card.id).unwrap().1;
        for idx in (card.id + 1..card.id + 1 + card.matching_numbers).into_iter() {
            map.entry(idx).and_modify(|c| c.1 += my_copy_count);
        }
    }

    map.into_iter()
        .map(|(_, (_, num_copies))| num_copies)
        .sum::<u32>()
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

        assert_eq!(30, process(input));
    }
}
