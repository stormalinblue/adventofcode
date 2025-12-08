use std::io;

fn main() {
    println!("Hello, world!");

    let mut current_offset: i64 = 50;
    let mut zero_crossings: i64 = 0;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.len() == 0 {
            break;
        }

        let (first, rest) = input.split_at(1);
        let number: i64 = rest.parse().expect("Failed to parse integer");

        match &first[0..1] {
            "L" => {
                if current_offset == 0 {
                    zero_crossings -= 1;
                }
                let new_offset = current_offset - number;
                // println!("new offset {}", new_offset);
                if new_offset > 0 {
                    zero_crossings += 0;
                    current_offset = new_offset;
                } else if new_offset == 0 {
                    zero_crossings += 1;
                    current_offset = new_offset;
                } else {
                    zero_crossings += (100 - new_offset) / 100;
                    current_offset = ((new_offset % 100) + 100) % 100
                }
            }
            "R" => {
                let new_offset = current_offset + number;
                zero_crossings += new_offset / 100;
                current_offset = new_offset % 100;
            }
            _ => {
                panic!("Weird letter");
            }
        }

        println!(
            "first {} rest {} current {} cross {}",
            first, rest, current_offset, zero_crossings
        );
    }

    println!("Number of zero crossings: {}", zero_crossings);
}
