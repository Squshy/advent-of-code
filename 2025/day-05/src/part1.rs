struct Inventory {
    fresh_id_ranges: Vec<(usize, usize)>,
    ingredients: Vec<usize>,
}

impl From<&str> for Inventory {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let mut fresh_id_ranges = Vec::new();
        let mut ingredients = Vec::new();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let (start, end) = line.split_once('-').expect("a valid range");
            fresh_id_ranges.push((
                start.parse::<usize>().expect("a valid num"),
                end.parse::<usize>().expect("a valid num"),
            ));
        }

        for line in lines.by_ref() {
            ingredients.push(line.parse::<usize>().expect("a valid num"));
        }

        Self {
            fresh_id_ranges,
            ingredients,
        }
    }
}

impl Inventory {
    fn count_fresh_ingredient(&self) -> usize {
        let mut sum = 0;
        for ingredient in &self.ingredients {
            for range in &self.fresh_id_ranges {
                if ingredient <= &range.1 && ingredient >= &range.0 {
                    sum += 1;
                    break;
                }
            }
        }

        sum
    }
}

pub fn solve() -> usize {
    Inventory::from(include_str!("../input/input.txt")).count_fresh_ingredient()
}
