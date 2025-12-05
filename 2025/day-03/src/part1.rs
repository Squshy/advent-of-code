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
    fn joltage(&self) -> u8 {
        let mut l = 0;
        let mut r = self.batteries.len() - 1;
        let mut l_max = 0;
        let mut r_max = 0;

        for (idx, battery) in self.batteries.iter().enumerate() {
            if idx == self.batteries.len() - 1 {
                break;
            }

            if battery > &l_max {
                l_max = *battery;
                l = idx;
            }
        }

        while r > l {
            r_max = r_max.max(self.batteries[r]);
            r -= 1;
        }

        l_max * 10 + r_max
    }
}

pub fn solve() -> usize {
    include_str!("../input/input.txt")
        .lines()
        .map(|line| BatteryBank::from(line).joltage() as usize)
        .sum()
}
