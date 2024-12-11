use std::time::Instant;

mod part1;
mod part2;

fn main() {
    let now = Instant::now();
    println!("Part 1: {}", part1::solve());
    let elapsed = now.elapsed();
    println!("Part 1 took: {:.2?}", elapsed);

    let now = Instant::now();
    println!("Part 2: {}", part2::solve());
    let elapsed = now.elapsed();
    println!("Part 2 took: {:.2?}", elapsed);
}
