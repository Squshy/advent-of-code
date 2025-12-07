#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<&str> for Operation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err("invalid operation"),
        }
    }
}

#[derive(Debug)]
struct Homework {
    numbers: Vec<Vec<usize>>,
    operations: Vec<Operation>,
}

impl From<&str> for Homework {
    fn from(value: &str) -> Self {
        let mut numbers = Vec::new();
        let mut operations = Vec::new();

        for line in value.lines() {
            let mut parts = line.split_whitespace();
            let first_part = parts.next().expect("at least one part");
            let mut nums = Vec::new();

            if let Ok(op) = Operation::try_from(first_part) {
                operations.push(op);
                for part in parts {
                    operations.push(Operation::try_from(part).expect("a valid operation"));
                }
            } else {
                nums.push(first_part.parse().expect("a valid number"));
                for part in parts {
                    nums.push(part.parse().expect("a valid num"));
                }

                numbers.push(nums);
            }
        }

        assert!(numbers[0].len() == operations.len());

        Self {
            numbers,
            operations,
        }
    }
}

impl Homework {
    fn total(&self) -> usize {
        let mut total = 0;

        for i in 0..self.operations.len() {
            total += match self.operations[i] {
                Operation::Add => {
                    let mut tot = 0;

                    for num_idx in 0..self.numbers.len() {
                        tot += self.numbers[num_idx][i];
                    }

                    tot
                }
                Operation::Multiply => {
                    let mut tot = 1;

                    for num_idx in 0..self.numbers.len() {
                        tot *= self.numbers[num_idx][i];
                    }

                    tot
                }
            }
        }

        total
    }
}

pub fn solve() -> usize {
    Homework::from(include_str!("../input/input.txt")).total()
}
