use std::io;

struct CombinationProducer {
    current_k: usize,
    n: usize,
    last_combo: Vec<usize>,
}

impl CombinationProducer {
    fn new(n: usize) -> Self {
        Self {
            current_k: 0,
            n,
            last_combo: vec![],
        }
    }
}

impl Iterator for CombinationProducer {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        loop {
            if self.current_k >= self.n {
                return None;
            } else {
                let mut rightmost_replacable: Option<usize> = None;
                for (index, element) in self.last_combo.iter().enumerate() {
                    if *element < ((self.n - self.current_k) + index) {
                        rightmost_replacable = Some(index);
                    }
                }

                match rightmost_replacable {
                    Some(index) => {
                        let next_value = self.last_combo[index] + 1;
                        let last_value = self.last_combo[index] + 1 + self.current_k - index;
                        self.last_combo
                            .splice(index..self.current_k, next_value..last_value);
                        return Some(self.last_combo.clone());
                    }
                    None => {
                        self.current_k += 1;
                        self.last_combo.clear();
                        self.last_combo.extend(0..self.current_k);
                        return Some(self.last_combo.clone());
                    }
                }
            }
        }
    }
}

fn bitpack(offsets: &Vec<u64>) -> u64 {
    let mut result: u64 = 0;
    for element in offsets {
        result |= 1 << element;
    }
    result
}

#[derive(Debug)]
struct ProblemData {
    target_mask: u64,
    switch_masks: Vec<u64>,
    weird_numbers: Vec<u64>,
}

fn parse_input() -> Vec<ProblemData> {
    let mut result = Vec::<ProblemData>::new();

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            return result;
        }

        let (target_mask_string, rest) = trimmed_line.split_once(" ").expect("Invalid line");
        let (switch_mask_string, weird_num_string) = rest.rsplit_once(" ").expect("Invalid line");

        let target_mask_vec = target_mask_string[1..(target_mask_string.len() - 1)]
            .chars()
            .enumerate()
            .filter_map(|(i, x)| match x {
                '.' => None,
                '#' => Some(i as u64),
                _ => unreachable!(),
            })
            .collect::<Vec<u64>>();
        let target_mask = bitpack(&target_mask_vec);

        let switch_masks: Vec<u64> = switch_mask_string
            .split_ascii_whitespace()
            .map(|sw_mask| {
                sw_mask[1..(sw_mask.len() - 1)]
                    .split(",")
                    .map(|num_str| num_str.parse::<u64>().expect("Invalid number"))
                    .collect()
            })
            .map(|nums| bitpack(&nums))
            .collect();

        let weird_nums = weird_num_string[1..(weird_num_string.len() - 1)]
            .split(",")
            .map(|x| x.parse::<u64>().expect("Invalid number"))
            .collect();

        result.push(ProblemData {
            target_mask,
            switch_masks,
            weird_numbers: weird_nums,
        });
    }
}

fn main() {
    let data = parse_input();
    // println!("Data is {:#?}", data);

    let mut tot_switches: u64 = 0;
    for row in data.iter() {
        let iterator = CombinationProducer::new(row.switch_masks.len());
        for combo in iterator {
            let mut tot_switch_mask: u64 = 0;
            for item in combo.iter() {
                tot_switch_mask ^= row.switch_masks[*item];
            }
            if tot_switch_mask == row.target_mask {
                tot_switches += combo.len() as u64;
                break;
            }
        }
    }

    println!("Tot switches {}", tot_switches);
}
