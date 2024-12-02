use std::collections::HashMap;

enum ModuleType {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

enum Pulse {
    High,
    Low,
}

struct FlipFlop {
    pulse: Pulse,
    on: bool,
    destinations: Vec<ModuleType>,
}

struct Conjunction {
    connections: HashMap<ModuleType, Pulse>,
    destinations: Vec<ModuleType>,
}

struct Broadcaster {
    destinations: Vec<ModuleType>,
}

trait Module {
    fn communicate(&mut self) -> ();
}

fn main() {
    let input = include_str!("../../data/example.txt");
}
