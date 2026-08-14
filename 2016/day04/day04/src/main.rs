use std::{cmp::Reverse, collections::BTreeMap, io::BufRead};

#[derive(Debug)]
struct EncryptedName {
    name: String,
}

impl EncryptedName {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn checksum(&self) -> String {
        let mut counts = BTreeMap::new();

        for c in self.name.chars() {
            if c.is_ascii_lowercase() {
                *counts.entry(c).or_insert(0) += 1
            }
        }

        let mut count_chars: Vec<_> = counts.iter().map(|(k, v)| (v, k)).collect();
        count_chars.sort_by_key(|(v, k)| (Reverse(*v), *k));
        count_chars.iter().map(|(_, c)| *c).take(5).collect()
    }
}

#[cfg(test)]
pub mod test {
    use crate::EncryptedName;

    #[test]
    fn test_checksum() {
        let ename = EncryptedName::new("aaaaa-bbb-z-y-x");
        assert_eq!(ename.checksum(), "abxyz")
    }
}

#[derive(Debug)]
struct Room {
    ename: EncryptedName,
    sector_id: i64,
    checksum: String,
}

fn parse_input() -> Vec<Room> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.expect("Could not read line");
            let trimmed_line = line.trim();

            let (ename_str, rest) = trimmed_line
                .rsplit_once('-')
                .expect("Expected at least one dash");

            let (sector_id_str, rest) = rest
                .rsplit_once('[')
                .expect("Expected at least one open bracket");

            let checksum_str = rest.trim_end_matches(']');

            Room {
                ename: EncryptedName::new(ename_str),
                sector_id: sector_id_str.parse().expect("Expected sector id number"),
                checksum: checksum_str.to_string(),
            }
        })
        .collect()
}

fn part1(input: &Vec<Room>) -> i64 {
    input
        .iter()
        .map(|room| {
            if room.ename.checksum() == room.checksum {
                room.sector_id
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &Vec<Room>) -> Option<i64> {
    let non_decoys = input
        .iter()
        .filter(|room| room.ename.checksum() == room.checksum);

    let characters: [char; 26] = (0..26)
        .map(|x| ((x + 'a' as u8) as char))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    for room in non_decoys {
        let out_str: String = room
            .ename
            .name
            .chars()
            .map(|x| match x {
                '-' => ' ',
                _ => characters[((x as usize) - ('a' as usize) + (room.sector_id as usize)) % 26],
            })
            .collect();
        if out_str.contains("north") {
            return Some(room.sector_id);
        }
    }

    None
}

fn main() {
    let input = parse_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
