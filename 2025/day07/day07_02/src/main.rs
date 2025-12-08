use std::{
    collections::{HashMap, HashSet},
    io::{self},
};

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Could not read line");

    let start = line.find("S").expect("Did not find S") as i32;

    let mut old_beams: HashMap<i32, i64> = HashMap::new();
    old_beams.insert(start, 1);

    let mut num_splits: i64 = 0;

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");

        if line.len() == 0 {
            break;
        }

        let mut splitters = HashSet::new();

        for (i, char) in line.chars().enumerate() {
            match char {
                '^' => {
                    splitters.insert(i as i32);
                }
                _ => {}
            }
        }

        let mut new_beams: HashMap<i32, i64> = HashMap::new();
        for (old_beam, old_beam_worlds) in old_beams.iter() {
            if splitters.contains(old_beam) {
                let left = *old_beam - 1;
                let right = *old_beam + 1;

                *new_beams.entry(left).or_insert(0) += old_beam_worlds;
                *new_beams.entry(right).or_insert(0) += old_beam_worlds;
                num_splits += 1;
            } else {
                *new_beams.entry(*old_beam).or_insert(0) += *old_beam_worlds;
            }
        }

        // num_splits += (new_beams.len() - old_beams.len()) as i64;

        old_beams = new_beams;
    }

    println!("Num splits {}", num_splits);
    println!("Num worlds {}", old_beams.values().sum::<i64>())
}
