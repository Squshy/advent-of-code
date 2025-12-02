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

fn rotate(distance: u32, rotation: &Rotation) -> u32 {
    match rotation.direction {
        Direction::Left => (distance + 100 - (rotation.distance % 100)) % 100,
        Direction::Right => (distance + rotation.distance) % 100,
    }
}

pub fn solve() -> usize {
    let mut distance = 50;
    let mut times = 0;

    for line in include_str!("../input/source.txt").lines() {
        let rotation = Rotation::from(line);
        distance = rotate(distance, &rotation);

        if distance == 0 {
            times += 1;
        }
    }

    times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_works() {
        assert_eq!(rotate(50, &Rotation::from("L886")), 64);
        assert_eq!(rotate(50, &Rotation::from("L68")), 82);
        assert_eq!(rotate(50, &Rotation::from("L568")), 82);
        assert_eq!(rotate(50, &Rotation::from("R68")), 18);
        assert_eq!(rotate(50, &Rotation::from("R268")), 18);
        assert_eq!(rotate(0, &Rotation::from("L50")), 50);
        assert_eq!(rotate(0, &Rotation::from("L250")), 50);
        assert_eq!(rotate(0, &Rotation::from("R250")), 50);
        assert_eq!(rotate(0, &Rotation::from("R50")), 50);
        assert_eq!(rotate(50, &Rotation::from("R50")), 0);
        assert_eq!(rotate(99, &Rotation::from("R1")), 0);
        assert_eq!(rotate(99, &Rotation::from("R101")), 0);
        assert_eq!(rotate(0, &Rotation::from("L1")), 99);
        assert_eq!(rotate(0, &Rotation::from("L101")), 99);
    }
}
