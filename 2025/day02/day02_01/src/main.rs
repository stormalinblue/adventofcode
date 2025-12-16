use std::cmp;
use std::io;

#[derive(Debug)]
struct Range {
    left: i64,
    right: i64,
}

impl Range {
    fn contains(&self, obj: i64) -> bool {
        self.left <= obj && obj <= self.right
    }

    fn parse(line: &str) -> Self {
        let (left, right) = line.split_once("-").expect("Bad range");
        Self {
            left: left.parse::<i64>().expect("Bad left"),
            right: right.parse::<i64>().expect("Bad right"),
        }
    }
}

#[derive(Debug)]
enum EventType {
    Start,
    End,
}

#[derive(Debug)]
struct Event {
    typ: EventType,
    index: i64,
}

fn parse_input() -> Vec<Range> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Could not read line");

    let result = line.trim().split(",").map(|x| Range::parse(x)).collect();
    result
}

fn range_to_events(ranges: &[Range]) -> Vec<Event> {
    let mut events = Vec::new();

    for range in ranges {
        events.push(Event {
            typ: EventType::Start,
            index: range.left,
        });

        events.push(Event {
            typ: EventType::End,
            index: range.right,
        });
    }

    events.sort_by_key(|x| {
        (
            x.index,
            match x.typ {
                EventType::End => 1,
                EventType::Start => 0,
            },
        )
    });

    events
}

fn invalid_in_range(range: &Range) -> i64 {
    const TEN: i64 = 10i64;

    // println!("Range {:?}", &range);

    let mut power: u32 = 1;
    let mut invalid_tot: i64 = 0;
    loop {
        let divisor = TEN.pow(power) + 1;
        let min_quot = TEN.pow(power - 1);
        let max_quot = min_quot * 10 - 1;

        let min_mul = divisor * min_quot;
        let max_mul = divisor * max_quot;

        // println!("range {:#?}", range);
        // println!("power {}", power);

        if range.left > max_mul {
            power += 1;
            continue;
        }

        if range.right < min_mul {
            break;
        }

        let lb = cmp::max(min_mul, range.left);
        let num_invalid = cmp::min(range.right / divisor, max_quot) - (lb / divisor)
            + (if lb % divisor == 0 { 1 } else { 0 });

        if num_invalid == 0 {
            power += 1;
            continue;
        } else {
            let first_invalid = if lb % divisor == 0 {
                lb / divisor
            } else {
                lb / divisor + 1
            };

            let last_invalid = first_invalid + num_invalid - 1;

            // println!("mul {} {}", min_mul, max_mul);
            // println!("First invalid {} {}", first_invalid, last_invalid);

            let invalid_sum =
                ((first_invalid + last_invalid) * (last_invalid - first_invalid + 1)) / 2;
            // println!("Invalid sum {}", invalid_sum);
            invalid_tot += invalid_sum * divisor;
        }

        power += 1;
    }

    invalid_tot
}

fn fix_ranges(ranges: &[Range]) -> Vec<Range> {
    let events = range_to_events(ranges);

    let mut new_ranges = Vec::<Range>::new();
    let mut last_start = -1;
    let mut active_ranges: i32 = 0;
    for event in events {
        match event.typ {
            EventType::Start => {
                last_start = event.index;
                active_ranges += 1;
            }
            EventType::End => {
                active_ranges -= 1;
                if active_ranges == 0 {
                    new_ranges.push(Range {
                        left: last_start,
                        right: event.index,
                    })
                }
            }
        }
    }

    new_ranges
}

fn main() {
    let mut ranges = parse_input();
    ranges.sort_by_key(|x| x.left);
    let fixed_ranges = &fix_ranges(ranges.as_slice());
    // println!("Before {} after {}", ranges.len(), fixed_ranges.len());

    // println!("Before {:?}", ranges);
    // println!("After {:?}", fixed_ranges);
    // println!("{:#?}", ranges);

    // for range in fixed_ranges {
    //     println!(
    //         "pair {} {} {}",
    //         range.left,
    //         range.right,
    //         invalid_in_range(&range)
    //     );
    //     // println!("");
    // }

    println!(
        "Total invalid {}",
        fixed_ranges
            .iter()
            .map(|x| invalid_in_range(x))
            .sum::<i64>()
    );
}
