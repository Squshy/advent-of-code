use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast90,
    NorthWest90,
    SouthWest90,
    SouthEast90,
    Ground,
    Start,
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let val = match self {
            Self::Vertical => "|",
            Self::Horizontal => "-",
            Self::NorthEast90 => "L",
            Self::NorthWest90 => "J",
            Self::SouthWest90 => "7",
            Self::SouthEast90 => "F",
            Self::Ground => ".",
            Self::Start => "S",
        };
        write!(f, "{}", val)
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast90,
            'J' => Self::NorthWest90,
            '7' => Self::SouthWest90,
            'F' => Self::SouthEast90,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid pipe character encountered: {value} "),
        }
    }
}

impl Pipe {
    fn dirs(&self) -> Option<[(i32, i32); 2]> {
        match self {
            Self::Vertical => Some([(-1, 0), (1, 0)]),
            Self::Horizontal => Some([(0, -1), (0, 1)]),
            Self::NorthWest90 => Some([(-1, 0), (0, -1)]),
            Self::NorthEast90 => Some([(-1, 0), (0, 1)]),
            Self::SouthEast90 => Some([(1, 0), (0, 1)]),
            Self::SouthWest90 => Some([(1, 0), (0, -1)]),
            _ => None,
        }
    }
}

// ASSUMPTIONS:
//  - There is only one loop

// Iterate through each tile
// If we see a pipe, explore the pipes path until we find a loop
//  - If we explore a pipe and there is no loop, ignore it
//  Once we find a loop, we need to explore both paths coming from the starting node
//  and get the max distance until we meet the same node. (doesn't matter where we start)
fn main() {
    let input = include_bytes!("../../data/input.txt");
    let lines = input.split(|&b| b == b'\n');

    let mut start = (i32::MAX, i32::MAX);
    let pipes = lines
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, ch)| {
                    let pipe = Pipe::from(*ch as char);
                    if pipe == Pipe::Start {
                        start = (row as i32, col as i32)
                    }

                    pipe
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut connected = vec![];

    for (r, c) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let row = start.0 + r;
        let col = start.1 + c;

        if row < 0 || row >= pipes.len() as i32 || col < 0 || col >= pipes[0].len() as i32 {
            continue;
        }

        let pipe = &pipes[row as usize][col as usize];
        if let Some(dirs) = pipe.dirs() {
            for dir in dirs {
                // If we get back to our starting position, we know they are connected
                if start.0 == start.0 + dir.0 + r && start.1 == start.1 + dir.1 + c {
                    // Distance, indexes
                    connected.push((1, (start.0 + r, start.1 + c)));
                }
            }
        }
    }

    let mut distances: HashMap<(i32, i32), u32> = HashMap::new();
    distances.insert(start, 0);

    while let Some((dist, (row, col))) = connected.pop() {
        if distances.contains_key(&(row, col)) {
            continue;
        }

        distances.insert((row, col), dist);

        if let Some(dirs) = pipes[row as usize][col as usize].dirs() {
            for (r, c) in dirs {
                connected.push((dist + 1, (row + r, col + c)));
            }
        }
    }

    println!("{:?}", distances);
    let x = distances.iter().map(|(_, d)| d).max().unwrap() + 1;
    println!("{:?}", x / 2);
}
