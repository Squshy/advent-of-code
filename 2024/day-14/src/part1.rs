use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl TryFrom<&str> for Position {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let nums = value
            .strip_prefix("p=")
            .ok_or("Invalid input".to_string())?
            .split_once(",")
            .ok_or("Invalid input".to_string())?;

        Ok(Self {
            x: isize::from_str_radix(nums.0, 10).unwrap(),
            y: isize::from_str_radix(nums.1, 10).unwrap(),
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Velocity {
    x: isize,
    y: isize,
}

impl Velocity {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl TryFrom<&str> for Velocity {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let nums = value
            .strip_prefix("v=")
            .ok_or("Invalid input".to_string())?
            .split_once(",")
            .ok_or("Invalid input".to_string())?;

        Ok(Self {
            x: isize::from_str_radix(nums.0, 10).unwrap(),
            y: isize::from_str_radix(nums.1, 10).unwrap(),
        })
    }
}

fn quad_for_pos(pos: &Position, width: isize, height: isize) -> Option<usize> {
    let mid_width = width / 2;
    let mid_height = height / 2;

    if pos.x < mid_width {
        if pos.y < mid_height {
            return Some(1);
        } else if pos.y > mid_height {
            return Some(2);
        }
    }

    if pos.x > mid_width {
        if pos.y < mid_height {
            return Some(3);
        } else if pos.y > mid_height {
            return Some(4);
        }
    }

    None
}

fn compute(pos: &Position, vel: &Velocity, width: isize, height: isize, num: isize) -> Position {
    let x = (pos.x + (vel.x * num % width)) % width;
    let x = if x.is_negative() { width + x } else { x };
    let y = (pos.y + (vel.y * num % height)) % height;
    let y = if y.is_negative() { height + y } else { y };
    Position::new(x, y)
}

fn it(input: &str, width: isize, height: isize, num: isize) -> u32 {
    let map =
        input
            .lines()
            .take_while(|line| !line.is_empty())
            .fold(HashMap::new(), |mut map, line| {
                let split = line.split_once(" ").unwrap();
                let pos = Position::try_from(split.0).unwrap();
                let vel = Velocity::try_from(split.1).unwrap();
                let new_pos = compute(&pos, &vel, width, height, num);
                if let Some(quad) = quad_for_pos(&new_pos, width, height) {
                    let vec = map.entry(quad).or_insert(Vec::new());
                    vec.push(new_pos);
                };

                map
            });

    map.iter().map(|(_, vec)| vec.len() as u32).product()
}

pub fn solve() -> u32 {
    it(include_str!("../input/part1.txt"), 101, 103, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Position::try_from("p=0,4"), Ok(Position { x: 0, y: 4 }));
        assert_eq!(Velocity::try_from("v=3,-3"), Ok(Velocity { x: 3, y: -3 }));
    }

    #[test]
    fn stuff() {
        let test_cases = vec![
            (
                Position::new(2, 4),
                Velocity::new(2, -3),
                11,
                7,
                5,
                Position::new(1, 3),
            ),
            (
                Position::new(0, 0),
                Velocity::new(1, 1),
                4,
                4,
                4,
                Position::new(0, 0),
            ),
            (
                Position::new(0, 0),
                Velocity::new(-1, -1),
                4,
                4,
                4,
                Position::new(0, 0),
            ),
            (
                Position::new(0, 0),
                Velocity::new(-1, -1),
                4,
                4,
                3,
                Position::new(1, 1),
            ),
        ];

        for (idx, (pos, vel, width, height, num, expected)) in test_cases.iter().enumerate() {
            assert_eq!(
                compute(&pos, &vel, *width as isize, *height as isize, *num as isize),
                *expected,
                "Failed at test case {}",
                idx + 1
            );
        }
    }

    #[test]
    fn example() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

        let val = it(input, 11, 7, 100);
        assert_eq!(val, 12);
    }
}
