use std::collections::{HashMap, HashSet};

fn is_special_char(ch: char) -> bool {
    if ch.is_alphanumeric() {
        return false;
    }

    if ch == '*' {
        true
    } else {
        false
    }
}

fn build_special_map(str: String) -> HashMap<(usize, usize), Vec<u32>> {
    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let lines: Vec<_> = str.lines().into_iter().collect::<Vec<&str>>();

    for (row, line) in lines.clone().into_iter().enumerate() {
        let mut parts: HashSet<(usize, usize)> = HashSet::new();
        let mut num_str = String::new();

        for (col, ch) in line.chars().enumerate() {
            let width = lines[0].len();
            let height = lines.len();
            let down = row + 1;
            let right = col + 1;

            if ch.is_digit(10) {
                num_str.push(ch);

                // Not in first row
                if row > 0 {
                    let prev = lines[row - 1].as_bytes();
                    if col > 0 && col - 1 >= 0 && is_special_char(prev[col - 1] as char) {
                        parts.insert((row - 1, col - 1));
                    } else if right < width && is_special_char(prev[right] as char) {
                        parts.insert((row - 1, right));
                    } else if is_special_char(prev[col] as char) {
                        parts.insert((row - 1, col));
                    }
                }

                if down < height {
                    let prev = lines[down].as_bytes();
                    if col > 0 && col - 1 >= 0 && is_special_char(prev[col - 1] as char) {
                        parts.insert((down, col - 1));
                    } else if right < width && is_special_char(prev[right] as char) {
                        parts.insert((down, right));
                    } else if is_special_char(prev[col] as char) {
                        parts.insert((down, col));
                    }
                }

                if col > 0 && col - 1 >= 0 && is_special_char(line.as_bytes()[col - 1] as char) {
                    parts.insert((row, col - 1));
                }

                if right < width && is_special_char(line.as_bytes()[right] as char) {
                    parts.insert((row, right));
                }
            } else {
                if !parts.is_empty() && num_str.len() > 0 {
                    let num = num_str.parse::<u32>().unwrap();
                    for part in parts.clone().into_iter() {
                        map.entry(part)
                            .and_modify(|nums| nums.push(num))
                            .or_insert_with(|| vec![num]);
                    }
                }
                num_str.clear();
                parts.clear();
            }
        }

        if num_str.len() > 0 && !parts.is_empty() {
            let num = num_str.parse::<u32>().unwrap();
            for part in parts.clone().into_iter() {
                map.entry(part)
                    .and_modify(|nums| nums.push(num))
                    .or_insert_with(|| vec![num]);
            }
        }
    }

    map.into_iter()
        .filter(|(_, entries)| entries.len() == 2)
        .collect::<HashMap<(usize, usize), Vec<u32>>>()
}

fn process(str: String) -> u32 {
    let map = build_special_map(str.clone());
    let mut total = 0;

    for (row, line) in str.lines().into_iter().enumerate() {
        for (col, ch) in line.chars().into_iter().enumerate() {
            if is_special_char(ch) {
                if map.contains_key(&(row, col)) {
                    let nums = map.get(&(row, col)).unwrap();
                    let num_tot = nums.into_iter().product::<u32>();
                    total += num_tot;
                }
            }
        }
    }

    total
}

fn main() {
    let input = std::fs::read_to_string("./data/input.txt").unwrap();
    let total = process(input);

    println!("TOTAL: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let total = process(input.to_string());
        println!("{total}");
        assert_eq!(467835, total);
    }
}
