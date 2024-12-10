#[derive(Debug)]
struct DiskMap {
    data: Vec<Option<usize>>,
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let data = value
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                if let Some(num) = c.to_digit(10) {
                    std::iter::repeat(if idx & 1 == 0 { Some(idx / 2) } else { None })
                        .take(num as usize)
                } else {
                    std::iter::repeat(Some(0)).take(0)
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        Self { data }
    }
}

impl DiskMap {
    fn checksum(&mut self) -> u64 {
        let mut idx = self.data.len();

        loop {
            let end_chunk_idx = self.data[0..idx]
                .iter()
                .rposition(|item| item.is_some())
                .unwrap();

            let Some(start_chunk_idx) = self.data[0..end_chunk_idx]
                .iter()
                .rposition(|id| id != &self.data[end_chunk_idx])
                .map(|idx| idx + 1)
            else {
                break;
            };

            let chunk_size = (start_chunk_idx..=end_chunk_idx).count();

            let Some(slot) = self
                .data
                .windows(chunk_size)
                .position(|slice| slice.iter().all(|item| item.is_none()))
            else {
                idx = start_chunk_idx;
                continue;
            };

            if slot < start_chunk_idx {
                let (left, right) = self.data.split_at_mut(start_chunk_idx);
                left[slot..(slot + chunk_size)].copy_from_slice(&right[..chunk_size]);

                for i in 0..chunk_size {
                    right[i] = None;
                }
            }

            idx = start_chunk_idx;
        }

        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, id)| id.map(|id| (id * idx) as u64))
            .sum()
    }
}

pub fn solve() -> u64 {
    DiskMap::from(include_str!("../input/part1.txt")).checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2333133121414131402";
        let mut disk = DiskMap::from(input);
        assert_eq!(disk.checksum(), 2858);
    }
}
