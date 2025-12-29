use std::cmp;
use std::io::{self, BufRead};

struct GiftBox {
    length: i64,
    width: i64,
    height: i64,
}

impl GiftBox {
    fn surface_area(&self) -> i64 {
        2 * (self.length * self.width + self.width * self.height + self.height * self.length)
    }

    fn smallest_surface(&self) -> i64 {
        cmp::min(
            self.length * self.width,
            cmp::min(self.width * self.height, self.height * self.length),
        )
    }

    fn gift_wrap_area(&self) -> i64 {
        self.surface_area() + self.smallest_surface()
    }

    fn smallest_perimeter(&self) -> i64 {
        2 * cmp::min(
            self.length + self.width,
            cmp::min(self.width + self.height, self.height + self.length),
        )
    }

    fn volume(&self) -> i64 {
        self.length * self.width * self.height
    }

    fn ribbon_length(&self) -> i64 {
        self.smallest_perimeter() + self.volume()
    }
}

fn parse_input() -> Vec<GiftBox> {
    let locked_io = io::stdin().lock();
    let mut result = Vec::<GiftBox>::new();
    for line_result in locked_io.lines() {
        let line = line_result.expect("Could not read line");
        let dimensions = line
            .split("x")
            .map(|dim| dim.parse::<i64>().expect("Non-integral dimension"))
            .collect::<Vec<i64>>();

        result.push(GiftBox {
            length: dimensions[0],
            width: dimensions[1],
            height: dimensions[2],
        })
    }
    result
}

fn part1(boxes: &[GiftBox]) -> i64 {
    boxes.iter().map(|b| b.gift_wrap_area()).sum::<i64>()
}

fn part2(boxes: &[GiftBox]) -> i64 {
    boxes.iter().map(|b| b.ribbon_length()).sum::<i64>()
}

fn main() {
    let input = parse_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
