type IdRange = (usize, usize);

fn parse_id_range(value: &str) -> IdRange {
    let (start, end) = value.split_once('-').expect("a delimiter");

    (
        start.parse().expect("a valid num"),
        end.parse().expect("a valid num"),
    )
}

struct Inventory {
    fresh_id_ranges: Vec<IdRange>,
}

impl From<&str> for Inventory {
    fn from(value: &str) -> Self {
        let mut ranges = value
            .lines()
            .take_while(|l| !l.is_empty())
            .map(parse_id_range)
            .collect::<Vec<_>>();
        ranges.sort();

        assert!(!ranges.is_empty());

        let mut out = Vec::with_capacity(ranges.len() / 2);
        out.push(ranges[0]);
        let mut last_idx = 0;

        // Can just calc all in one loop don't need to go over them again later
        // in our method
        for range in ranges.iter().skip(1) {
            let prev = out[last_idx];

            // (10-13), (14-16) can become (10-16)
            if prev.1 >= range.0 - 1 {
                out[last_idx] = (prev.0, range.1.max(prev.1));
            } else {
                out.push(*range);
                last_idx += 1;
            }
        }

        Self {
            fresh_id_ranges: out,
        }
    }
}

impl Inventory {
    fn count_fresh_ingredients(&self) -> usize {
        self.fresh_id_ranges.iter().map(|r| r.1 - r.0 + 1).sum()
    }
}

pub fn solve() -> usize {
    Inventory::from(include_str!("../input/input.txt")).count_fresh_ingredients()
}
