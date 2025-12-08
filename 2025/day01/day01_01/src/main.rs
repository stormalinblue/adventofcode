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
        // println!("first {} rest {}", first, rest);

        match &first[0..1] {
            "L" => {
                current_offset -= number;
            }
            "R" => {
                current_offset += number;
            }
            _ => {
                panic!("Weird letter");
            }
        }

        current_offset = (current_offset + 100) % 100;
        // println!("Current offset: {}", current_offset);

        if current_offset == 0 {
            zero_crossings += 1;
        }
    }

    println!("Number of zero crossings: {}", zero_crossings);
}
