#[derive(Debug, Eq, PartialEq)]
enum DiskItem {
    File(u64),
    FreeSpace,
}

#[derive(Debug)]
struct DiskMap {
    data: Vec<DiskItem>,
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let data = value
            .chars()
            .enumerate()
            .fold(Vec::new(), |mut vec, (idx, c)| {
                // Could probably be a smarter nicer way of doing this,
                // but eh this is fine for now : )
                if let Some(num) = c.to_digit(10) {
                    if idx & 1 == 0 {
                        let file_id: u64 = (idx / 2) as u64;
                        for _ in 0..num {
                            vec.push(DiskItem::File(file_id));
                        }
                    } else {
                        for _ in 0..num {
                            vec.push(DiskItem::FreeSpace);
                        }
                    };
                }

                vec
            });

        Self { data }
    }
}

impl DiskMap {
    fn find_next_file_idx(&self, start_idx: usize) -> usize {
        for (idx, item) in self
            .data
            .iter()
            .rev()
            .enumerate()
            .skip_while(|(idx, _)| self.data.len() - 1 - idx > start_idx)
        {
            if item != &DiskItem::FreeSpace {
                return self.data.len() - 1 - idx;
            }
        }

        0
    }

    fn find_next_empty_idx(&self, start_idx: usize) -> usize {
        for (idx, item) in self
            .data
            .iter()
            .enumerate()
            .skip_while(|(idx, _)| idx < &start_idx)
        {
            if item == &DiskItem::FreeSpace {
                return idx;
            }
        }

        0
    }

    fn move_files_to_start(&mut self) -> u64 {
        let mut empty_idx = self.find_next_empty_idx(0);
        let mut file_idx = self.find_next_file_idx(self.data.len() - 1);

        while empty_idx < file_idx {
            self.data.swap(empty_idx, file_idx);
            empty_idx = self.find_next_empty_idx(empty_idx + 1);
            file_idx = self.find_next_file_idx(file_idx - 1);
        }

        self.data
            .iter()
            .enumerate()
            .map(|(idx, item)| match item {
                DiskItem::File(id) => *id * idx as u64,
                _ => 0,
            })
            .sum()
    }
}

pub fn solve() -> u64 {
    let input = include_str!("../input/part1.txt");
    DiskMap::from(input).move_files_to_start()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2333133121414131402";
        let mut disk = DiskMap::from(input);
        assert_eq!(disk.move_files_to_start(), 1928);
    }
}
