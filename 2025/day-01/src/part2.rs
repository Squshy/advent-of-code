#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(ch: char) -> Self {
        match ch {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: u32,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let direction = Direction::from(chars.next().expect("a direction"));
        let distance = chars.fold(0u32, |acc, c| {
            c.to_digit(10).map(|d| acc * 10 + d).expect("a valid digit")
        });

        Self {
            direction,
            distance,
        }
    }
}

fn rotate(distance: u32, rotation: &Rotation) -> (u32, u32) {
    let cycles = rotation.distance / 100;
    let step = rotation.distance % 100;

    match rotation.direction {
        Direction::Left => (
            (distance + 100 - step) % 100,
            cycles
                + if step >= distance && distance > 0 {
                    1
                } else {
                    0
                },
        ),
        Direction::Right => (
            (distance + rotation.distance) % 100,
            cycles + if distance + step >= 100 { 1 } else { 0 },
        ),
    }
}

pub fn solve() -> usize {
    let mut distance = 50;
    let mut times = 0usize;

    for line in include_str!("../input/source.txt").lines() {
        let rotation = Rotation::from(line);
        let (new_distance, extra_times) = rotate(distance, &rotation);
        distance = new_distance;
        times += extra_times as usize;
    }

    times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_works() {
        assert_eq!(rotate(0, &Rotation::from("L200")), (0, 2));
        assert_eq!(rotate(50, &Rotation::from("L20")), (30, 0));
        assert_eq!(rotate(50, &Rotation::from("L200")), (50, 2));
        assert_eq!(rotate(50, &Rotation::from("L668")), (82, 7));

        assert_eq!(rotate(50, &Rotation::from("L68")), (82, 1));
        assert_eq!(rotate(82, &Rotation::from("L30")), (52, 0));
        assert_eq!(rotate(52, &Rotation::from("R48")), (0, 1));
        assert_eq!(rotate(0, &Rotation::from("L5")), (95, 0));
        assert_eq!(rotate(95, &Rotation::from("R60")), (55, 1));
        assert_eq!(rotate(55, &Rotation::from("L55")), (0, 1));
        assert_eq!(rotate(0, &Rotation::from("L1")), (99, 0));
        assert_eq!(rotate(99, &Rotation::from("L99")), (0, 1));
        assert_eq!(rotate(0, &Rotation::from("R14")), (14, 0));
        assert_eq!(rotate(14, &Rotation::from("L82")), (32, 1));
    }
}
