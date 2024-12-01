use std::{cmp::Ordering, collections::BTreeMap, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome<'a> {
    Accepted,
    Rejected,
    Other(&'a str),
}

#[derive(Debug)]
struct Rule<'a> {
    category: u8,
    cmp_type: Ordering,
    threshold: usize,
    outcome: Outcome<'a>,
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    fallback: Outcome<'a>,
}

#[derive(Debug, Default)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl<'a> Outcome<'a> {
    fn from_str(s: &'a str) -> Self {
        match s {
            "A" => Outcome::Accepted,
            "R" => Outcome::Rejected,
            _ => Outcome::Other(s),
        }
    }
}

impl Rule<'_> {
    fn apply(&self, part: &Part) -> Option<Outcome<'_>> {
        (match self.category {
            b'x' => part.x,
            b'm' => part.m,
            b'a' => part.a,
            b's' => part.s,
            _ => unreachable!(),
        }
        .cmp(&self.threshold)
            == self.cmp_type)
            .then_some(self.outcome)
    }
}

impl<'a> Workflow<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut rules = s.split(',');
        let fallback = Outcome::from_str(rules.next_back().unwrap());

        let rules = rules
            .map(|rule| {
                let (predicate, outcome) = rule.split_once(':').unwrap();

                let category = predicate.as_bytes()[0];
                let cmp_type = match predicate.as_bytes()[1] {
                    b'<' => Ordering::Less,
                    b'>' => Ordering::Greater,
                    _ => unreachable!(),
                };
                let outcome = Outcome::from_str(outcome);
                let threshold = predicate[2..].parse().unwrap();

                Rule {
                    category,
                    cmp_type,
                    outcome,
                    threshold,
                }
            })
            .collect();

        Self { rules, fallback }
    }

    fn apply_to(&self, part: &Part) -> Outcome<'_> {
        self.rules
            .iter()
            .find_map(|rule| rule.apply(part))
            .unwrap_or(self.fallback)
    }
}

impl Part {
    fn rating(self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn from_str(value: &str) -> Self {
        let mut this = Part::default();
        let categories = value.trim_matches(&['{', '}'] as &[char]);

        categories.split(',').for_each(|category| {
            let (category, value) = category.split_once('=').unwrap();
            let value = value.parse().unwrap();
            match category {
                "x" => this.x = value,
                "m" => this.m = value,
                "a" => this.a = value,
                "s" => this.s = value,
                _ => unreachable!(),
            }
        });

        this
    }
}

fn main() {
    let input = fs::read_to_string("day19/input.txt").unwrap();
    part_1(&input);
}

fn part_1(input: &str) {
    let (workflows, mashine_parts) = input.split_once("\r\n\r\n").unwrap();
    let parts: Vec<_> = mashine_parts.lines().map(Part::from_str).collect();

    let workflows = workflows.lines().fold(BTreeMap::new(), |mut acc, line| {
        let (name, rules) = line.split_once('{').unwrap();
        let workflow = Workflow::from_str(&rules[..rules.len() - 1]);
        acc.insert(name, workflow);
        acc
    });

    let result: usize = parts
        .into_iter()
        .filter_map(|part| {
            let mut curr_workflow = workflows.get("in").unwrap();
            let mut curr_state = curr_workflow.apply_to(&part);

            while let Outcome::Other(other) = curr_state {
                curr_workflow = workflows.get(other).unwrap();
                curr_state = curr_workflow.apply_to(&part);
            }

            (curr_state == Outcome::Accepted).then_some(part.rating())
        })
        .sum();

    println!("Day19 Part1: {result}")
}
