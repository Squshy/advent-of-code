fn is_special_char(ch: char) -> bool {
    if ch.is_alphanumeric() {
        return false;
    }

    if ch == '.' {
        false
    } else {
        println!("{ch} is special");
        true
    }
}

fn process(str: String) -> u32 {
    let mut total = 0;

    let lines: Vec<_> = str.lines().into_iter().collect::<Vec<&str>>();

    for (row, line) in lines.clone().into_iter().enumerate() {
        let mut is_part = false;
        let mut num_str = String::new();

        for (col, ch) in line.chars().enumerate() {
            let width = lines[0].len();
            let height = lines.len();
            let down = row + 1;
            let right = col + 1;

            if ch.is_digit(10) {
                num_str.push(ch);

                if is_part == false {
                    // Not in first row
                    if row > 0 {
                        let prev = lines[row - 1].as_bytes();
                        if col > 0 && col - 1 >= 0 && is_special_char(prev[col - 1] as char) {
                            is_part = true;
                        } else if right < width && is_special_char(prev[right] as char) {
                            is_part = true;
                        } else if is_special_char(prev[col] as char) {
                            is_part = true;
                        }
                    }

                    if down < height {
                        let prev = lines[down].as_bytes();
                        if col > 0 && col - 1 >= 0 && is_special_char(prev[col - 1] as char) {
                            is_part = true;
                        } else if right < width && is_special_char(prev[right] as char) {
                            is_part = true;
                        } else if is_special_char(prev[col] as char) {
                            is_part = true;
                        }
                    }

                    if col > 0 && col - 1 >= 0 && is_special_char(line.as_bytes()[col - 1] as char)
                    {
                        is_part = true;
                    }

                    if right < width && is_special_char(line.as_bytes()[right] as char) {
                        is_part = true;
                    }
                }
            } else {
                if is_part && num_str.len() > 0 {
                    total += num_str.parse::<u32>().unwrap();
                }
                num_str.clear();
                is_part = false;
            }
        }

        if num_str.len() > 0 && is_part {
            total += num_str.parse::<u32>().unwrap();
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
        let input = r#"...463
.*...."#;
        let total = process(input.to_string());
        println!("{total}");
        assert_eq!(464, total);
    }
}
