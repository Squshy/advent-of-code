fn process_line(str: &str) -> u32 {
    let (_, cube_sets) = str.split_once(':').unwrap();

    let mut max_blue = 1;
    let mut max_red = 1;
    let mut max_green = 1;

    for hehe in cube_sets.split(';').into_iter() {
        for str in hehe.split(",").into_iter() {
            let (num, color) = str.trim().split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();

            match color.to_lowercase().as_str() {
                "blue" => {
                    if num > max_blue {
                        max_blue = num;
                    }
                }
                "red" => {
                    if num > max_red {
                        max_red = num;
                    }
                }
                "green" => {
                    if num > max_green {
                        max_green = num;
                    }
                }
                _ => panic!("Invalid color encountered {color}"),
            };
        }
    }

    max_blue * max_red * max_green
}

fn main() {
    let total = std::fs::read_to_string("./data/input.txt")
        .unwrap()
        .lines()
        .map(process_line)
        .sum::<u32>();

    println!("Total: {total}");
}
