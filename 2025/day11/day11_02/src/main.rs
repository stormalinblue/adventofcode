use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Graph {
    node_outs: HashMap<String, Vec<String>>,
}

fn parse_input() -> Graph {
    let mut result = Graph {
        node_outs: HashMap::new(),
    };

    result
        .node_outs
        .insert(String::from_str("out").unwrap(), vec![]);

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line.");

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            break;
        }

        let (node_name, neighbor_string) = trimmed_line
            .split_once(": ")
            .expect("Invalid node definition");
        let neighbors = neighbor_string
            .split_whitespace()
            .map(|x| String::from_str(x).unwrap())
            .collect();

        result
            .node_outs
            .insert(String::from_str(node_name).unwrap(), neighbors);
    }

    result
}

fn num_paths(graph: &Graph, start: &str) -> i64 {
    let mut queue = VecDeque::<&str>::new();
    let mut num_paths: HashMap<&str, i64> = HashMap::new();
    let mut in_degree: HashMap<&str, i64> = HashMap::new();

    for (node, neighbors) in graph.node_outs.iter() {
        in_degree.entry(node).or_insert(0);
        num_paths.entry(node).or_insert(0);
        for neighbor in neighbors.iter() {
            *in_degree.entry(neighbor).or_insert(0) += 1;
        }
    }

    for (node, indegree) in in_degree.iter() {
        if *indegree == 0 {
            queue.push_back(node);
        }
    }

    num_paths.insert(start, 1);

    // for (node, indegree) in in_degree.iter() {
    //     println!("Indegree of node {} is {}", node, indegree);
    // }

    while let Some(current) = queue.pop_front() {
        // println!("Considering node {}", current);
        let current_paths = *num_paths.get(current).expect("No paths to current");

        for neighbor in graph.node_outs.get(current).expect("Dangling node") {
            *num_paths.entry(neighbor).or_insert(0) += current_paths;

            let neighbor_degree: &mut i64 = in_degree
                .get_mut(neighbor.as_str())
                .expect("Missing neighbor");
            // println!(
            //     "Considering neighbor {} with indegree {}",
            //     neighbor, neighbor_degree
            // );

            *neighbor_degree -= 1;
            if *neighbor_degree == 0 {
                // println!("Neighbor {} freed!", neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    *num_paths.get("out").expect("No path to out")
}

fn remove_node(graph: &mut Graph, node: &str) {
    graph.node_outs.remove(node);
    graph
        .node_outs
        .values_mut()
        .for_each(|v| v.retain(|x| *x != node));
}

fn main() {
    let graph = parse_input();

    let mut graph_without_dac = graph.clone();
    remove_node(&mut graph_without_dac, "dac");
    let mut graph_without_fft = graph.clone();
    remove_node(&mut graph_without_fft, "fft");
    let mut graph_without_fft_and_dac = graph.clone();
    remove_node(&mut graph_without_fft_and_dac, "fft");
    remove_node(&mut graph_without_fft_and_dac, "dac");

    // println!("Graph is {:#?}", graph);
    let start = "svr";

    let all = num_paths(&graph, start);
    println!("Num paths all {}", all);
    let d = num_paths(&graph_without_fft, start);
    println!("Num paths without fft {}", d);
    let f = num_paths(&graph_without_dac, start);
    println!("Num paths without fft {}", f);
    let df = num_paths(&graph_without_fft_and_dac, start);
    println!("Num paths without fft and dac {}", df);

    let num_use_d = all - d;
    let num_use_f = all - f;
    let num_use_d_or_f = all - df;
    let num_use_d_and_f = num_use_d + num_use_f - num_use_d_or_f;

    println!("Answer should be {}", num_use_d_and_f);
}
