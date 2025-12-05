#[derive(Eq, PartialEq)]
enum Spot {
    Paper,
    Empty,
}

impl From<char> for Spot {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Paper,
            '.' => Self::Empty,
            _ => panic!("invalid input"),
        }
    }
}

struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T>
where
    T: From<char>,
{
    pub fn from(s: &str) -> Self {
        let rows: Vec<Vec<T>> = s
            .lines()
            .map(|line| line.chars().map(T::from).collect())
            .collect();

        let height = rows.len();
        let width = rows.first().map(|r| r.len()).unwrap_or(0);
        let cells = rows.into_iter().flatten().collect();

        Self {
            width,
            height,
            cells,
        }
    }
}

struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.cells.len() {
            return None;
        }

        let width = self.grid.width;
        let idx = self.index;

        let x = idx % width;
        let y = idx / width;

        self.index += 1;

        Some((x, y, &self.grid.cells[idx]))
    }
}

impl<T> Grid<T> {
    fn iter_with_coords(&self) -> GridIter<'_, T> {
        GridIter {
            grid: self,
            index: 0,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }

    fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        const DIRECTIONS: [(isize, isize); 8] = [
            (-1, -1), // NW
            (-1, 0),  // N
            (-1, 1),  // NE
            (0, -1),  // W
            (0, 1),   // E
            (1, -1),  // SW
            (1, 0),   // S
            (1, 1),   // SE
        ];

        DIRECTIONS.iter().filter_map(move |(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;

            if x >= 0 && y >= 0 {
                self.get(x as usize, y as usize)
            } else {
                None
            }
        })
    }
}

pub fn solve() -> usize {
    let grid: Grid<Spot> = Grid::from(include_str!("../input/input.txt"));

    grid.iter_with_coords()
        .filter(|(_, _, item)| **item == Spot::Paper)
        .filter(|(x, y, _)| {
            grid.neighbours(*x, *y)
                .filter(|i| **i == Spot::Paper)
                .count()
                < 4
        })
        .count()
}
