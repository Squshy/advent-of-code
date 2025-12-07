#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
}

pub struct Position(usize, usize);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    fn inner(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl Position {
    pub fn new_in_dir(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => {
                if self.1 == 0 {
                    None
                } else {
                    Some(Self(self.0, self.1 - 1))
                }
            }
            Direction::UpLeft => {
                if self.1 == 0 || self.0 == 0 {
                    None
                } else {
                    Some(Self(self.0 - 1, self.1 - 1))
                }
            }
            Direction::UpRight => {
                if self.1 == 0 {
                    None
                } else {
                    Some(Self(self.0, self.1 - 1))
                }
            }
            Direction::Down => Some(Self(self.0, self.1 + 1)),
            Direction::DownLeft => {
                if self.0 == 0 {
                    None
                } else {
                    Some(Self(self.0 - 1, self.1 + 1))
                }
            }
            Direction::DownRight => Some(Self(self.0 + 1, self.1 + 1)),
            Direction::Left => {
                if self.0 == 0 {
                    None
                } else {
                    Some(Self(self.0 - 1, self.1))
                }
            }
            Direction::Right => Some(Self(self.0 + 1, self.1)),
        }
    }
}

pub struct Grid<T> {
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

pub struct GridCoordIter<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> Iterator for GridCoordIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.cells.len() {
            return None;
        }

        let width = self.grid.width;
        let idx = self.index;

        let x = idx % width;
        let y = idx / width;

        self.index += 1;

        Some((Position(x, y), &self.grid.cells[idx]))
    }
}

impl<T> Grid<T> {
    pub fn row(&self, row: usize) -> Option<&[T]> {
        if row >= self.height {
            None
        } else {
            Some(&self.cells[(self.width * row)..(self.width * (row + 1))])
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn iter_with_coords(&self) -> GridCoordIter<'_, T> {
        GridCoordIter {
            grid: self,
            index: 0,
        }
    }

    pub fn get(&self, coord: &Position) -> Option<&T> {
        let (x, y) = coord.inner();

        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }

    /// Replaces a value in the grid and returns the old value.
    /// Returns `None` if the indexes are out of bounds.
    pub fn set(&mut self, coord: &Position, value: T) -> Option<T> {
        let (x, y) = coord.inner();

        if x < self.width && y < self.height {
            Some(std::mem::replace(
                &mut self.cells[y * self.width + x],
                value,
            ))
        } else {
            None
        }
    }

    pub fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
        const DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::Left,
            Direction::Right,
            Direction::DownLeft,
            Direction::Down,
            Direction::DownRight,
        ];

        DIRECTIONS.iter().filter_map(move |dir| {
            Position(x, y)
                .new_in_dir(*dir)
                .and_then(|coord| self.get(&coord))
        })
    }
}
