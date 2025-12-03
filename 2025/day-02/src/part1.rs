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

fn nth_digit_from_right(n: Id, index: usize) -> usize {
    (n / 10_usize.pow(index as u32)) % 10
}

fn num_digits(id: Id) -> usize {
    if id == 0 { 1 } else { id.ilog10() as usize + 1 }
}

fn is_odd(v: usize) -> bool {
    (v & 1) > 0
}

fn is_invalid_id(id: Id) -> bool {
    let digits = num_digits(id);
    if is_odd(digits) {
        return false;
    }

    for i in 0..digits / 2 {
        let first = nth_digit_from_right(id, digits / 2 + i);
        let second = nth_digit_from_right(id, i);

        if first != second {
            return false;
        }
    }

    true
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
    fn nth_digit_from_right_is_oki() {
        assert_eq!(nth_digit_from_right(12345, 0), 5);
        assert_eq!(nth_digit_from_right(12345, 1), 4);
        assert_eq!(nth_digit_from_right(12345, 2), 3);
        assert_eq!(nth_digit_from_right(12345, 3), 2);
        assert_eq!(nth_digit_from_right(12345, 4), 1);
    }

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

        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(115));
        assert!(!is_invalid_id(1123131313123));
    }
}
