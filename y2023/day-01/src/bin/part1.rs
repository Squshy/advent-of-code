fn get_nums_from_str(line: &str) -> u64 {
    let line_len = line.len();

    if line_len == 0 {
        return 0;
    }

    let mut left = 0;
    let mut right = line_len - 1;
    let mut last_digit: Option<u32> = None;
    let mut first_digit: Option<u32> = None;

    while last_digit.is_none() || first_digit.is_none() {
        let lc = line.as_bytes()[left] as char;
        let cc = line.as_bytes()[right] as char;

        if lc.is_digit(10) && first_digit.is_none() {
            first_digit = Some(lc.to_digit(10).unwrap());
        }

        if cc.is_digit(10) && last_digit.is_none() {
            last_digit = Some(cc.to_digit(10).unwrap());
        }

        if last_digit.is_some() && first_digit.is_some() {
            break;
        }

        left += 1;
        right -= 1;
    }

    if last_digit.is_none() || first_digit.is_none() {
        panic!("Invalid string encountered: {}", line);
    }

    println!(
        "Line: {} | {}{}",
        line,
        first_digit.unwrap(),
        last_digit.unwrap()
    );
    format!("{}{}", first_digit.unwrap(), last_digit.unwrap())
        .parse::<u64>()
        .unwrap()
}

fn get_hehe(str: String) -> u128 {
    let mut total = 0;

    for line in str.lines() {
        total += get_nums_from_str(line) as u128;
    }

    total
}

fn main() {
    let total = get_hehe(std::fs::read_to_string("./data/input.txt").unwrap());

    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let lines = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
three8seven
four24qphdrxfsf
9sevenvlttm
treb7uchet"#;

        assert_eq!(get_hehe(lines.to_string()), 142 + 88 + 99 + 24);
    }
}
