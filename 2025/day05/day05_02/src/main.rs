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
struct ProblemData {
    ranges: Vec<Range>,
    ingredients: Vec<i64>,
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

fn parse_input() -> ProblemData {
    let mut data: ProblemData = ProblemData {
        ranges: vec![],
        ingredients: vec![],
    };

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        } else {
            data.ranges.push(Range::parse(trimmed_line))
        }
    }

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        } else {
            data.ingredients
                .push(trimmed_line.parse::<i64>().expect("Bad ingredient"));
        }
    }

    data.ranges.sort_by_key(|x| x.left);
    data.ingredients.sort();

    data
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
            index: range.right + 1,
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

fn main() {
    let data = parse_input();

    let events = range_to_events(data.ranges.as_slice());

    println!("Events {:?}", events);

    let mut ranges_open = 0;
    let mut last_start = 0;
    let mut fresh_extent = 0;
    for event in events {
        match event.typ {
            EventType::Start => {
                if ranges_open == 0 {
                    println!("Start range {}", event.index);
                    last_start = event.index;
                }
                ranges_open += 1;
            }
            EventType::End => {
                ranges_open -= 1;
                if ranges_open == 0 {
                    println!("End range {}", event.index);
                    fresh_extent += event.index - last_start;
                }
            }
        }
    }

    println!("Fresh extent: {}", fresh_extent);
}
