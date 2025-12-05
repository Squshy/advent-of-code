const BATTERIES_TO_COUNT: usize = 12;

struct BatteryBank {
    batteries: Vec<u8>,
}

impl From<&str> for BatteryBank {
    fn from(value: &str) -> Self {
        let batteries = value
            .chars()
            .map(|c| c.to_digit(10).expect("a digit") as u8)
            .collect::<Vec<_>>();

        Self { batteries }
    }
}

impl BatteryBank {
    fn joltage(&self) -> usize {
        let mut num_joltages_to_remove = self.batteries.len() - BATTERIES_TO_COUNT;
        let mut stack = Vec::with_capacity(BATTERIES_TO_COUNT);

        for &battery in &self.batteries {
            while !stack.is_empty()
                && num_joltages_to_remove > 0
                && stack[stack.len() - 1] < battery
            {
                stack.pop();
                num_joltages_to_remove -= 1;
            }

            stack.push(battery);
        }

        stack.truncate(BATTERIES_TO_COUNT);

        stack
            .iter()
            .fold(0, |acc, battery| acc * 10 + *battery as usize)
    }
}

pub fn solve() -> usize {
    include_str!("../input/input.txt")
        .lines()
        .map(|line| BatteryBank::from(line).joltage())
        .sum()
}
