use std::env;

fn parse_target_day(target_day: &str) -> i16 {
    let mut xd = String::new();
    target_day.chars().into_iter().for_each(|cur| {
        if cur.is_numeric() {
            xd = format!("{}{}", xd, cur)
        }
    });

    xd.parse().expect("Couldn't parse your input dummy")
}

fn run_target_day(target_day: i16) {
    println!("Running day {}.", target_day);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    if len != 2 {
        panic!("Must provide 1 CLI argument for day number");
    }

    let target_day = &args[1];
    let target_day = parse_target_day(&target_day);

    match target_day {
        1 => run_target_day(target_day),
        _ => println!("No day found for provided day number: {}", target_day),
    }
}
