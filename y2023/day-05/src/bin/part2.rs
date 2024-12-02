// @see: https://github.com/timvisee/advent-of-code-2023/blob/master/day05b/src/main.rs
use std::{cell::RefCell, iter::from_fn};

#[derive(Debug)]
struct RangeMap {
    dest_start: u128,
    source_start: u128,
    range: u128,
}

impl RangeMap {
    fn map_value(&self, seed: &SeedRange) -> Vec<SeedRange> {
        let mut seeds: Vec<_> = vec![];
        // We are entirely within the start range
        if seed.start >= self.source_start && seed.max_range_val() < self.source_start + self.range
        {
            seeds.push(SeedRange::new(
                self.source_start + self.range - 1 - seed.max_range_val() + seed.start,
                self.range,
            ));
        } else
        // We are partially within the range
        // 20 55 20 -> (20, 39), (55, 74)
        // We are currently (70, 10) -> (70, 79)
        // We fall partially within (55, 74) leading us to now come out as
        // (35, 4)
        if seed.start < self.source_start + self.range && seed.start >= self.source_start {
            let diff = self.source_start + self.range - 1 - seed.start;
            seeds.push(SeedRange::new(
                self.dest_start + seed.start - self.source_start,
                diff,
            ))
        } else {
            seeds.push(seed.clone());
        }

        seeds
    }
}

#[derive(Copy, Clone, Debug)]
struct SeedRange {
    start: u128,
    range: u128,
}

impl SeedRange {
    #[inline(always)]
    fn new(start: u128, range: u128) -> Self {
        Self { start, range }
    }

    #[inline(always)]
    fn max_range_val(&self) -> u128 {
        self.range + self.start - 1
    }
}

fn process() -> u128 {
    let input = include_bytes!("../../data/input.txt");
    let sep = input.iter().position(|&b| b == b':').unwrap();
    let mut seeds = input.split(|&b| b == b'\n').next().unwrap()[sep + 1..]
        .split(|&b| b == b' ')
        .filter_map(|seed| std::str::from_utf8(seed).ok()?.parse::<u128>().ok())
        .collect::<Vec<u128>>()
        .chunks(2)
        .map(|w| SeedRange::new(w[0], w[1]))
        .collect::<Vec<_>>();

    let mut ranges: Vec<RangeMap> = vec![];
    input.split(|&b| b == b'\n').skip(1).for_each(|line| {
        if line.is_empty() || !line[0].is_ascii_digit() {
            if !ranges.is_empty() {
                let mut guys: Vec<_> = vec![];
                for seed in seeds.clone().into_iter() {
                    guys.extend(
                        ranges
                            .iter_mut()
                            .flat_map(|r| r.map_value(&seed))
                            .collect::<Vec<SeedRange>>(),
                    );
                }
                seeds = guys;
                ranges.clear();
            }
            return;
        }

        let range = line
            .split(|&b| b == b' ')
            .filter_map(|range| std::str::from_utf8(range).ok()?.parse::<u128>().ok())
            .collect::<Vec<u128>>();

        ranges.push(RangeMap {
            dest_start: range[0],
            source_start: range[1],
            range: range[2],
        });
    });

    seeds.into_iter().map(|seed| seed.start).min().unwrap()
}

fn _process() -> u64 {
    const SECTIONS: usize = 7;
    let input = include_bytes!("../../data/input.txt");

    let mut seeds = input[SECTIONS..input.iter().position(|b| b == &b'\n').unwrap()]
        .split(|b| b == &b' ')
        .flat_map(atoi::atoi::<u64>);
    let mut lines = input.split(|b| b == &b'\n').skip(2);

    let maps: Vec<Vec<(std::ops::Range<u64>, u64)>> = (0..SECTIONS)
        .map(|_| {
            let mut map = (&mut lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut entry = line
                        .splitn(3, |b| b == &b' ')
                        .map(|n| atoi::atoi(n).unwrap());
                    let el: [_; 3] = std::array::from_fn(|_| entry.next().unwrap());
                    println!("{:?}", el);
                    (el[1]..el[1] + el[2], el[0])
                })
                .collect::<Vec<_>>();
            map.sort_by_key(|(range, _)| range.start);
            map
        })
        .collect();

    from_fn(|| seeds.next().zip(seeds.next()))
        .map(|(start, len)| start..start + len)
        .flat_map(|range| {
            maps.iter().fold(vec![range], |ranges, map| {
                ranges
                    .into_iter()
                    .flat_map(|base| {
                        let base_cell = RefCell::new(base);
                        map.iter()
                            .take_while(|_| !base_cell.borrow().is_empty())
                            .fold(Vec::with_capacity(6), |mut from, (to, n)| {
                                let mut base = base_cell.borrow_mut();
                                if base.start < to.start {
                                    from.push(base.start..(base.end.min(to.start)));
                                    base.start = to.start;
                                }

                                let len = base.end.min(to.end).saturating_sub(base.start);
                                if len > 0 {
                                    let to = *n + base.start - to.start;
                                    from.push(to..to + len);
                                    base.start += len;
                                }
                                from
                            })
                    })
                    .collect()
            })
        })
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn main() {
    println!("{}", _process());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guy() {
        // 20 55 20 -> (20, 39), (55, 74)
        // We are currently (70, 10) -> (70, 79)
        // We fall partially within (55, 74) leading us to now come out as
        // (35, 4)
        let rm = RangeMap {
            source_start: 55,
            dest_start: 20,
            range: 20,
        };
        let sr = SeedRange {
            start: 70,
            range: 10,
        };

        let res = rm.map_value(&sr).unwrap();
        assert_eq!(res.start, 35);
        assert_eq!(res.range, 4);
    }
}
