fn is_set_valid(str: &str) -> bool {
    for str in str.split(",").into_iter() {
        let (num, color) = str.trim().split_once(' ').unwrap();
        let num = num.parse::<u32>().unwrap();

        let valid = match color.to_lowercase().as_str() {
            "blue" => num <= 14,
            "red" => num <= 12,
            "green" => num <= 13,
            _ => panic!("Invalid color encountered {color}"),
        };

        if !valid {
            return valid;
        }
    }

    true
}

fn process_line(str: &str) -> u32 {
    let (game_id, cube_sets) = str.split_once(':').unwrap();
    let game_id = game_id.split(' ').last().unwrap().parse::<u32>().unwrap();

    let mut is_valid = true;
    for cube_set in cube_sets.split(';').into_iter() {
        if !is_set_valid(cube_set) {
            is_valid = false;
            break;
        }
    }

    if is_valid {
        game_id
    } else {
        0
    }
}

fn main() {
    let total = std::fs::read_to_string("./data/input.txt")
        .unwrap()
        .lines()
        .map(process_line)
        .sum::<u32>();

    println!("Total: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(8, input.lines().map(process_line).sum::<u32>());
    }
}
