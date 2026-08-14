use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
struct BoyerMoore<E>
where
    E: Eq,
    E: Clone,
{
    candidate: Option<E>,
    count: i64,
}

impl<E> Default for BoyerMoore<E>
where
    E: Eq,
    E: Clone,
{
    fn default() -> Self {
        BoyerMoore {
            candidate: None,
            count: 0,
        }
    }
}

impl<E> BoyerMoore<E>
where
    E: Eq,
    E: Clone,
    E: std::fmt::Debug,
{
    fn current_mode(&self) -> &Option<E> {
        &self.candidate
    }

    fn update(&mut self, element: &E) {
        println!("pre update with {:?}, {:?}", element, &self);
        (self.candidate, self.count) = match (
            &self.candidate,
            self.candidate == Some(element.clone()),
            self.count,
        ) {
            (None, _, _) => (Some(element.clone()), 1),
            (prev, true, c) => (prev.clone(), c + 1),
            (_, false, 1) => (None, 0),
            (prev, false, c) => (prev.clone(), c - 1),
        };
        println!("post update {:?}", &self);
    }
}

fn parse_input() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Could not read line").trim().to_string())
        .collect()
}

fn column_counts(input: &Vec<String>) -> Vec<HashMap<char, i64>> {
    let num_cols = input.first().expect("Should have one").chars().count();

    let mut counter_vec = Vec::<HashMap<char, i64>>::with_capacity(num_cols);
    counter_vec.resize(num_cols, HashMap::<char, i64>::default());

    for line in input.iter() {
        for (index, c) in line.chars().enumerate() {
            *counter_vec[index].entry(c).or_default() += 1;
        }
    }

    counter_vec
}

fn part1(input: &Vec<String>) -> String {
    let counter_vec = column_counts(input);

    counter_vec
        .iter()
        .map(|counter| {
            counter
                .iter()
                .max_by_key(|(_, v)| *v)
                .expect("Should have at least one")
                .0
        })
        .collect::<String>()
}

fn part2(input: &Vec<String>) -> String {
    let counter_vec = column_counts(input);

    counter_vec
        .iter()
        .map(|counter| {
            counter
                .iter()
                .min_by_key(|(_, v)| *v)
                .expect("Should have at least one")
                .0
        })
        .collect::<String>()
}

fn main() {
    let input = parse_input();

    // println!("{:#?}", input);

    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
