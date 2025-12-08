use std::{
    collections::HashSet,
    io::{self},
};

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Could not read line");

    let start = line.find("S").expect("Did not find S") as i32;

    let mut old_beams = HashSet::new();
    old_beams.insert(start);

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

        let mut new_beams: HashSet<i32> = HashSet::new();
        for old_beam in old_beams.iter() {
            if splitters.contains(old_beam) {
                new_beams.insert((*old_beam) - 1);
                new_beams.insert((*old_beam) + 1);
                num_splits += 1;
            } else {
                new_beams.insert(*old_beam);
            }
        }

        // num_splits += (new_beams.len() - old_beams.len()) as i64;

        old_beams = new_beams;
    }

    println!("Num splits {}", num_splits);
}
