use std::io;

#[derive(Debug, Copy, Clone)]
enum Paren {
    Open,
    Close,
}

fn parse_input() -> Vec<Paren> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Couldn't read line");

    let line = buf.trim();
    line.chars()
        .map(|x| match x {
            '(' => Paren::Open,
            ')' => Paren::Close,
            _ => panic!("Bad symbol"),
        })
        .collect::<Vec<Paren>>()
}

fn part1(parens: &[Paren]) -> i64 {
    let result = parens
        .iter()
        .map(|p| match p {
            Paren::Open => 1i64,
            Paren::Close => -1i64,
        })
        .sum::<i64>();

    result
}

fn part2(parens: &[Paren]) -> i64 {
    let mut accum: i64 = 0;
    for (index, p) in parens.iter().enumerate() {
        match p {
            Paren::Open => {
                accum += 1;
            }
            Paren::Close => {
                accum -= 1;
                if accum == -1 {
                    return (index as i64) + 1i64;
                }
            }
        }
    }
    return -1;
}

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
