#[derive(PartialEq, Eq, Clone, Copy)]
enum LevelChange {
    Increasing,
    Decreasing,
    Equal,
}

impl LevelChange {
    fn from_window(window: &[u32]) -> Self {
        if window[0] == window[1] {
            Self::Equal
        } else if window[0] < window[1] {
            Self::Increasing
        } else {
            Self::Decreasing
        }
    }
}

fn is_report_safe(data: Vec<u32>) -> bool {
    let mut level_change: Option<LevelChange> = None;

    for window in data.windows(2) {
        let diff = window[0].abs_diff(window[1]);

        if diff > 3 || diff < 1 {
            return false;
        }

        match level_change {
            Some(change) => {
                let new_change = LevelChange::from_window(window);

                if change != new_change || new_change == LevelChange::Equal {
                    return false;
                }
            }
            None => {
                let new_change = LevelChange::from_window(window);

                if new_change == LevelChange::Equal {
                    return false;
                }

                level_change = Some(new_change)
            }
        }
    }

    if let Some(level_change) = level_change {
        level_change != LevelChange::Equal
    } else {
        true
    }
}

pub fn solve() -> u32 {
    let safe_count = include_str!("../input/part1.txt")
        .lines()
        .into_iter()
        .map(|line| {
            let nums = line
                .split(" ")
                .map(|num| u32::from_str_radix(num, 10).unwrap())
                .collect::<Vec<_>>();

            if is_report_safe(nums) {
                1
            } else {
                0
            }
        })
        .sum();

    safe_count
}
