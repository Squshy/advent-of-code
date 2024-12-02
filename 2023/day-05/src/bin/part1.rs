struct RangeMap {
    dest_start: u32,
    source_start: u32,
    range: u32,
}

impl RangeMap {
    fn map_value(&self, dest: u32) -> Option<u32> {
        if dest >= self.source_start && dest < self.source_start + self.range {
            Some(self.dest_start + (dest - self.source_start))
        } else {
            None
        }
    }
}

fn process() -> u32 {
    let input = include_bytes!("../../data/input.txt");
    let sep = input.iter().position(|&b| b == b':').unwrap();
    let mut seeds = input.split(|&b| b == b'\n').next().unwrap()[sep + 1..]
        .split(|&b| b == b' ')
        .filter_map(|seed| std::str::from_utf8(seed).ok()?.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let mut ranges: Vec<RangeMap> = vec![];
    input.split(|&b| b == b'\n').skip(1).for_each(|line| {
        if line.is_empty() || !line[0].is_ascii_digit() {
            if !ranges.is_empty() {
                for (idx, seed) in seeds.clone().into_iter().enumerate() {
                    let mut val: Option<u32> = None;
                    for range in ranges.iter_mut() {
                        if let Some(r) = range.map_value(seed) {
                            val = Some(r);
                            break;
                        }
                    }
                    seeds[idx] = val.unwrap_or(seed);
                }
            }
            ranges.clear();
            return;
        }

        let range = line
            .split(|&b| b == b' ')
            .filter_map(|range| std::str::from_utf8(range).ok()?.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        ranges.push(RangeMap {
            dest_start: range[0],
            source_start: range[1],
            range: range[2],
        });
    });

    seeds.into_iter().min().unwrap()
}

fn main() {
    println!("{}", process());
}
