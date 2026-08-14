use std::io::{self, BufRead};

type Triangle = [i64; 3];

fn parse_input() -> Vec<Triangle> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let result: [i64; 3] = line
                .expect("Could not read line")
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse().expect("Expected a number"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Expect 3");
            result
        })
        .collect()
}

fn part1(input: &Vec<Triangle>) -> i64 {
    input
        .iter()
        .map(|tri_unsorted| {
            let mut tri = *tri_unsorted;
            tri.sort();
            let result = if (tri[0] + tri[1]) > tri[2] { 1 } else { 0 };
            result
        })
        .sum()
}

fn part2(input: &Vec<Triangle>) -> i64 {
    let new_triangles = input
        .chunks_exact(3)
        .map(|chunk| {
            let result = [
                [chunk[0][0], chunk[1][0], chunk[2][0]],
                [chunk[0][1], chunk[1][1], chunk[2][1]],
                [chunk[0][2], chunk[1][2], chunk[2][2]],
            ];
            result
        })
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?}", &new_triangles[0..9]);
    part1(&new_triangles)
}

fn main() {
    let input = parse_input();

    println!("{:?}", &input[0..9]);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
