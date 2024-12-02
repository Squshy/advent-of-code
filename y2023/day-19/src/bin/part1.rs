use std::collections::HashMap;

#[derive(Debug)]
enum Ranking {
    _Cool,
    _Musical,
    _Aerodynamic,
    _Shiny,
}

#[derive(Debug, Clone)]
enum Destination {
    Accept,
    Reject,
    Rule(String),
}

impl From<String> for Destination {
    fn from(value: String) -> Self {
        match value.as_str() {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Rule(value),
        }
    }
}

#[derive(Debug)]
enum Operand {
    LT,
    GT,
}

impl From<char> for Operand {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::LT,
            '>' => Self::GT,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Part {
    X,
    M,
    A,
    S,
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'a' => Self::A,
            'm' => Self::M,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        Self::from(value.chars().next().unwrap())
    }
}

#[derive(Debug)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Condition {
    part: Part,
    value: usize,
    operand: Operand,
}

impl From<String> for Condition {
    fn from(value: String) -> Self {
        let mut value = value.chars();

        Self {
            part: value.next().unwrap().into(),
            operand: value.next().unwrap().into(),
            value: value.collect::<String>().parse::<usize>().unwrap(),
        }
    }
}

impl Condition {
    fn is_valid(&self, rating: &Rating) -> bool {
        match self.operand {
            Operand::LT => match self.part {
                Part::X => rating.x < self.value,
                Part::M => rating.m < self.value,
                Part::A => rating.a < self.value,
                Part::S => rating.s < self.value,
            },
            Operand::GT => match self.part {
                Part::X => rating.x > self.value,
                Part::M => rating.m > self.value,
                Part::A => rating.a > self.value,
                Part::S => rating.s > self.value,
            },
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    destination: Destination,
}

impl From<String> for Rule {
    fn from(value: String) -> Self {
        if !value.contains(":") {
            return Self {
                condition: None,
                destination: value.into(),
            };
        };

        let (cond, dest) = value.split_once(":").unwrap();

        Self {
            condition: Some(cond.to_string().into()),
            destination: dest.to_string().into(),
        }
    }
}

impl Rule {
    fn next_dest(&self, rating: &Rating) -> Option<Destination> {
        if let Some(cond) = &self.condition {
            if cond.is_valid(rating) {
                return Some(self.destination.clone());
            }

            return None;
        }

        Some(self.destination.clone())
    }
}

#[derive(Debug)]
struct Workflow {
    _name: String,
    rules: Vec<Rule>,
}

fn parse_input() -> (HashMap<String, Workflow>, Vec<Rating>) {
    let mut input = include_str!("../../data/input.txt").split("\n\n");
    let workflows = input
        .next()
        .unwrap()
        .split("\n")
        .map(|flow| {
            let (name, rules) = flow.split_once("{").unwrap();

            (
                name.to_string(),
                Workflow {
                    _name: name.to_string(),
                    rules: rules
                        .split(",")
                        .into_iter()
                        .map(|rule| rule.replace("}", "").into())
                        .collect::<Vec<Rule>>(),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let ratings = input
        .next()
        .unwrap()
        .split("\n")
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let ratings = &line[1..line.len() - 1]
                .split(",")
                .map(|rating| {
                    let (_, value) = rating.split_once("=").unwrap();

                    value.parse::<usize>().unwrap()
                })
                .collect::<Vec<_>>();

            Rating {
                x: ratings[0],
                m: ratings[1],
                a: ratings[2],
                s: ratings[3],
            }
        })
        .collect::<Vec<_>>();

    (workflows, ratings)
}

fn is_rating_accepted(
    rating: &Rating,
    dest: Destination,
    workflows: &HashMap<String, Workflow>,
) -> bool {
    match dest {
        Destination::Accept => return true,
        Destination::Reject => return false,
        Destination::Rule(name) => {
            let flow = workflows.get(&name).unwrap();

            for rule in &flow.rules {
                if let Some(x) = rule.next_dest(rating) {
                    return is_rating_accepted(rating, x, workflows);
                }
            }
        }
    };

    todo!()
}

// We want to parse every workflow into its name and its set of rules
//
// These rules will determine if the part tested against this workflow passes
// or not
//
// We can save the workflows in a map from its name being the key to the workflow
//
// We will parse all of the parts into a list of parts and test them against the
// workflows
fn main() {
    let (workflows, ratings) = parse_input();

    let mut good = 0;
    for rating in ratings {
        if is_rating_accepted(&rating, Destination::Rule("in".to_string()), &workflows) {
            good += rating.sum();
        }
    }

    println!("{good}");
}
