use std::collections::HashMap;

fn process() -> u32 {
    let input = include_str!("../../data/input.txt");
    let mut lines = input.split("\n");
    let mut dirs = lines.next().unwrap().chars().map(|ch| ch).cycle();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    lines.for_each(|line| {
        if !line.is_empty() {
            let key = &line[..3];
            let r = &line[7..10];
            let l = &line[12..15];

            map.insert(key, (r, l));
        }
    });

    let mut nodes = map
        .iter()
        .filter_map(|node| {
            if node.0.ends_with("A") {
                return Some(*node.1);
            }

            None
        })
        .collect::<Vec<(&str, &str)>>();

    let mut count = 1;

    loop {
        let dir = dirs.next().unwrap();

        let mut is_oki = true;

        println!("{count}");
        for node_idx in 0..nodes.len() {
            let node = nodes[node_idx];
            match dir {
                'R' => {
                    if !node.1.ends_with("Z") {
                        is_oki = false;
                    }

                    nodes[node_idx] = *map.get(node.1).unwrap();
                }
                'L' => {
                    if !node.0.ends_with("Z") {
                        is_oki = false;
                    }

                    nodes[node_idx] = *map.get(node.0).unwrap();
                }
                _ => panic!("Invalid direction encountered"),
            }
        }

        if is_oki {
            return count;
        }

        count += 1;
    }
}

fn main() {
    println!("{}", process());
}
