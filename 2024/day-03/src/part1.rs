use regex::Regex;

#[derive(Debug)]
struct Mul {
    x: u32,
    y: u32,
}

impl Mul {
    fn new_from_str<'a>(str: &'a str) -> Self {
        let mut stripped = str.chars();
        stripped.next();
        stripped.next();
        stripped.next();
        stripped.next();
        stripped.next_back();
        let (x, y) = stripped.as_str().split_once(",").unwrap();

        Self {
            x: u32::from_str_radix(x, 10).unwrap(),
            y: u32::from_str_radix(y, 10).unwrap(),
        }
    }

    fn multiply(&self) -> u32 {
        self.x * self.y
    }
}

pub fn solve() -> u32 {
    let input = include_str!("../input/part1.txt");
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();

    regex
        .captures_iter(input)
        .into_iter()
        .map(|cap| Mul::new_from_str(cap.get(1).unwrap().as_str()).multiply())
        .sum()
}
