use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn parse(line: &str) -> Self {
        let coords: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

fn dist2(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
}

fn read_points() -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            break;
        } else {
            points.push(Point::parse(trimmed_line));
        }
    }

    points
}

#[derive(Debug)]
struct Edge {
    left: usize,
    right: usize,
    dist: i64,
}

struct UnionFind {
    parent: Vec<Option<usize>>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: vec![None; size],
        }
    }

    fn union(&mut self, left: usize, right: usize) {
        let left_parent = self.find(left);
        let right_parent = self.find(right);

        if left_parent != right_parent {
            self.parent[left_parent] = Some(right_parent)
        }
    }

    fn find(&mut self, target: usize) -> usize {
        // println!("finding {}", target);
        match self.parent[target] {
            None => target,
            Some(parent) => {
                let new_parent = self.find(parent);
                self.parent[target] = Some(new_parent);
                new_parent
            }
        }
    }
}

fn main() {
    let points = read_points();

    // println!("Points {:?}", points);

    let mut edges: Vec<Edge> = Vec::new();

    for (left_index, left_point) in points.iter().enumerate() {
        for (right_offset, right_point) in points[left_index + 1..].iter().enumerate() {
            let right_index = right_offset + left_index + 1;
            edges.push(Edge {
                left: left_index,
                right: right_index,
                dist: dist2(left_point, right_point),
            })
        }
    }

    edges.sort_by_key(|x| x.dist);

    let mut unionfind = UnionFind::new(points.len());

    const MAX_EDGES: usize = 1000;

    let mut num_edges_added: usize = 0;
    for edge in edges {
        unionfind.union(edge.left, edge.right);
        // println!(
        //     "Connect edge {:?} {:?} {}",
        //     points[edge.left], points[edge.right], edge.dist
        // );
        num_edges_added += 1;

        if num_edges_added >= MAX_EDGES {
            break;
        }
    }

    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();

    for index in 0..points.len() {
        let parent = unionfind.find(index);

        *circuit_sizes.entry(parent).or_insert(0) += 1;
    }

    let mut circuit_size_tups: Vec<_> = circuit_sizes.iter().collect();

    circuit_size_tups.sort_by(|a, b| b.1.cmp(a.1));

    let mul_size: usize = circuit_size_tups
        .into_iter()
        .take(3)
        .map(|x| *x.1)
        .product();

    // println!("Circuit sizes are: {:?}", circuit_sizes);
    println!("Mul size is: {}", mul_size)
}
