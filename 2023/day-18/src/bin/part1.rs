use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Unexpected direction string {}", value),
        }
    }
}

#[derive(Debug, Clone)]
struct DigStep<'a> {
    dir: Direction,
    dist: isize,
    _color: &'a str,
}

impl<'a> From<&'a str> for DigStep<'a> {
    fn from(value: &'a str) -> Self {
        let mut parts = value.split(" ");

        Self {
            dir: Direction::from(parts.next().unwrap()),
            dist: parts.next().unwrap().parse::<isize>().unwrap(),
            _color: parts.next().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    #[inline]
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    fn neighbours(&self) -> Vec<Point> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(ver, hor)| Point {
                x: self.x + hor,
                y: self.y + ver,
            })
            .collect::<Vec<Point>>()
    }
}

struct Grid {
    points: HashSet<Point>,
}

impl Grid {
    #[inline]
    fn new() -> Self {
        Self {
            points: HashSet::default(),
        }
    }

    fn draw_edges(&mut self, steps: &[DigStep]) {
        let mut cur = Point::default();

        for step in steps {
            // Add a new point in our grid for every step you take
            for _ in 0..step.dist {
                match step.dir {
                    Direction::Down => cur.y += 1,
                    Direction::Up => cur.y -= 1,
                    Direction::Left => cur.x -= 1,
                    Direction::Right => cur.x += 1,
                }
                self.points.insert(cur);
            }
        }
    }

    // Recursion here helps us simplify this problem using our hash set
    // We can simply recurse adding every single possible neighbour we find into our
    // set from our initial point. If we had some funky shape, it would be trickier
    // to figure out if a point is inside the shape or not
    fn fill(&mut self, start: &Point) {
        self.points.insert(*start);

        // Note: We do not go over the bounds with our neighbours as we are
        // gaurunteed to only hit our inside nodes or our outline nodes
        for point in start.neighbours() {
            if !self.points.contains(&point) {
                self.fill(&point);
            }
        }
    }

    fn find_point(&self) -> Point {
        // Get our north-most point row
        let min_y = self.points.iter().map(|p| p.y).min().unwrap();

        let mut start = None;
        // Iterate over our entire grid for the highest row
        for point in self.points.iter().filter(|p| p.y == min_y) {
            // Check for a point, one row below us
            let point = Point {
                x: point.x,
                y: point.y + 1,
            };

            // If we do not have the point in our list of points, we know it is
            // contained within our loop and can use it as a starting point
            if !self.points.contains(&point) {
                start = Some(point);
                break;
            }
        }

        start.unwrap()
    }
}

fn main() {
    let dig_plan = include_str!("../../data/input.txt");
    let plans = dig_plan.lines().map(DigStep::from).collect::<Vec<_>>();

    // Create our grid of points
    let mut grid = Grid::new();
    // Create our outlining loop from our input steps
    grid.draw_edges(&plans);
    // Get a starting node to fill our hole
    grid.fill(&grid.find_point());
    println!("{}", grid.points.len());
}
