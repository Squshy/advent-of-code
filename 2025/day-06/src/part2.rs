const NUM_SIZE: usize = 4;

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Multiply),
            _ => Err("invalid operation"),
        }
    }
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
struct Num([usize; NUM_SIZE]);

#[derive(Debug)]
struct Homework {
    numbers: Vec<Vec<Num>>,
    operations: Vec<Operation>,
}

impl From<&str> for Homework {
    fn from(value: &str) -> Self {
        let mut numbers = Vec::new();
        let mut operations = Vec::new();
        let mut operation_idxs = Vec::new();
        let chars = value
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for (idx, c) in chars[chars.len() - 1].iter().enumerate() {
            if let Ok(op) = Operation::try_from(*c) {
                operations.push(op);
                operation_idxs.push(idx);
            }
        }

        for (i, op_idx) in operation_idxs.iter().enumerate() {
            let go_until_idx = if i >= operation_idxs.len() - 1 {
                chars[0].len()
            } else {
                operation_idxs[i + 1] - 1
            };

            let mut nums = Vec::new();
            // Ignore operations in last row
            for problem_idx in 0..chars.len() - 1 {
                let mut num = [0usize; NUM_SIZE];
                for (num_idx, idx) in (*op_idx..go_until_idx).enumerate() {
                    match chars[problem_idx][idx].to_digit(10) {
                        Some(d) => num[num_idx] = d as usize,
                        None => num[num_idx] = 0,
                    }
                }
                nums.push(Num(num));
            }

            numbers.push(nums);
        }

        assert!(numbers.len() == operations.len());

        Self {
            numbers,
            operations,
        }
    }
}

impl Homework {
    fn total(&self) -> usize {
        let mut total = 0;
        let append_digit = |acc: usize, c: usize| if c > 0 { acc * 10 + c } else { acc };

        for i in (0..self.operations.len()).rev() {
            total += match self.operations[i] {
                Operation::Add => {
                    let mut tot = 0;

                    for digit_idx in 0..NUM_SIZE {
                        let mut num = 0;
                        for num_idx in 0..self.numbers[i].len() {
                            num = append_digit(num, self.numbers[i][num_idx].0[digit_idx]);
                        }

                        tot += num;
                    }

                    tot
                }
                Operation::Multiply => {
                    let mut tot = 1;

                    for digit_idx in 0..NUM_SIZE {
                        let mut num = 0;
                        for num_idx in 0..self.numbers[i].len() {
                            num = append_digit(num, self.numbers[i][num_idx].0[digit_idx]);
                        }

                        tot *= if num == 0 { 1 } else { num };
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
