fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|idx| {
        let reduced_line = &line[idx..];
        let res = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else if reduced_line.starts_with("zero") {
            '0'
        } else {
            reduced_line.chars().next().unwrap()
        };

        res.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a valid number")
}

fn main() {
    let total = std::fs::read_to_string("./data/input.txt")
        .unwrap()
        .lines()
        .map(process_line)
        .sum::<u32>();

    println!("{total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hehe() {
        let lines = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

        assert_eq!(lines.lines().map(process_line).sum::<u32>(), 281);
    }
}
