fn process() -> u32 {
    let input = include_str!("../../data/input.txt");
    let times_ms = input
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|t| t.trim().parse::<u32>().ok());
    let distances_mm = input
        .lines()
        .nth(1)
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|t| t.trim().parse::<u32>().ok());

    times_ms
        .zip(distances_mm)
        .map(|(time, dist)| {
            (0..time)
                .map(|held_time| {
                    let time_remaining = time - held_time;
                    if dist < (time_remaining * held_time) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .product::<u32>()
}

fn main() {
    println!("{}", process());
}
