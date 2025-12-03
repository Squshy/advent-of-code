type Id = usize;

struct Range {
    first: Id,
    last: Id,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (first, last) = value.split_once('-').expect("valid parts");

        Self {
            first: first.trim().parse().expect("a valid id"),
            last: last.trim().parse().expect("a valid id"),
        }
    }
}

fn num_digits(id: Id) -> usize {
    if id == 0 { 1 } else { id.ilog10() as usize + 1 }
}

fn slice_digits(n: Id, start: usize, end: usize) -> usize {
    let len = end - start + 1;
    let drop_right = num_digits(n) - end - 1;

    n / 10_usize.pow(drop_right as u32) % 10_usize.pow(len as u32)
}

fn is_invalid_id(id: Id) -> bool {
    let digits = num_digits(id);

    // Patterns greater than half the number of digits cannot repeat
    for i in 1..=(digits / 2) {
        // If we cannot get an equal amount of slices, no need to check
        if digits % i != 0 {
            continue;
        }

        let num_patterns_to_check = digits / i;
        let pattern = slice_digits(id, 0, i - 1);

        if (1..num_patterns_to_check).all(|j| slice_digits(id, j * i, j * i + i - 1) == pattern) {
            return true;
        }
    }

    false
}

impl Range {
    fn sum_invalid_ids(&self) -> usize {
        let mut sum = 0;

        for id in self.first..=self.last {
            if is_invalid_id(id) {
                sum += id;
            }
        }

        sum
    }
}

pub fn solve() -> usize {
    include_str!("../input/input.txt")
        .split(',')
        .map(|l| Range::from(l).sum_invalid_ids())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_invalid_id_is_oki() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(446446));
        assert!(is_invalid_id(38593859));
        assert!(is_invalid_id(1111));
        assert!(is_invalid_id(2222));
        assert!(is_invalid_id(1188511885));
        assert!(is_invalid_id(1111111));

        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(115));
        assert!(!is_invalid_id(1123131313123));
    }
}
