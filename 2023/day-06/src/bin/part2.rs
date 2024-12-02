fn process() -> u64 {
    let input = include_str!("../../data/input.txt");
    let time_ms = input
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|t| t.trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance_mm = input
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|t| t.trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    (0..time_ms)
        .map(|held_time| {
            if distance_mm < (time_ms - held_time) * held_time {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
}

fn main() {
    println!("{}", process());
}
