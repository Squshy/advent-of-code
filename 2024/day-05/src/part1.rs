use std::collections::{HashMap, HashSet};

type PageOrderRules = HashMap<u32, Vec<u32>>;
type PageNumbers = Vec<u32>;

fn to_u32(str: &str) -> u32 {
    u32::from_str_radix(str, 10).unwrap()
}

fn parse_input(str: &str) -> (Vec<PageNumbers>, PageOrderRules) {
    let (rules, numbers) = str.split_once("\n\n").expect("Valid input");

    let rules: PageOrderRules = rules
        .split("\n")
        .take_while(|line| !line.is_empty())
        .into_iter()
        .fold(HashMap::new(), |mut map, line| {
            let (first, second) = line.split_once("|").expect("Valid line");
            let entry = map.entry(to_u32(first)).or_insert(vec![]);
            entry.push(to_u32(second));

            map
        });

    let numbers: Vec<PageNumbers> = numbers
        .split("\n")
        .take_while(|line| !line.is_empty())
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|num| u32::from_str_radix(num, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (numbers, rules)
}

fn is_row_valid(page_numbers: &PageNumbers, page_order_rules: &PageOrderRules) -> bool {
    let mut seen_numbers: HashSet<&u32> = HashSet::new();

    for page_number in page_numbers {
        if let Some(p) = page_order_rules.get(page_number) {
            // If we have already seen a number that should come after us,
            // the row is invalid
            if p.iter().any(|num| seen_numbers.contains(num)) {
                return false;
            }
        }

        seen_numbers.insert(page_number);
    }

    true
}

pub fn solve() -> u32 {
    let (pages, rules) = parse_input(include_str!("../input/part1.txt"));
    pages
        .iter()
        .filter(|row| is_row_valid(row, &rules))
        .map(|row| row.get(row.len() / 2).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{is_row_valid, parse_input};

    #[test]
    fn test_is_row_valid() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let (nums, rules) = parse_input(input);

        let valid_count = nums.iter().filter(|row| is_row_valid(row, &rules)).count();
        assert_eq!(valid_count, 3);
        let sum: u32 = nums
            .iter()
            .filter(|row| is_row_valid(row, &rules))
            .map(|row| row.get(row.len() / 2).unwrap_or(&0))
            .sum();
        assert_eq!(sum, 143);
    }
}