fn run_length_encode<E, Q>(iter: E) -> impl Iterator<Item = (usize, Q)>
where
    E: Iterator<Item = Q>,
    Q: Eq,
{
    let mut peekable = iter.peekable();

    std::iter::from_fn(move || {
        let first = peekable.next()?;
        let mut count = 1;

        while let Some(next) = peekable.peek() {
            if next == &first {
                peekable.next();
                count += 1;
            } else {
                break;
            }
        }

        Some((count, first))
    })
}

fn recursive_rle_len(iterations: usize) -> usize {
    // let mut epoch = vec![1usize];
    let mut epoch: Vec<usize> = "3113322113"
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    for _ in 0..iterations {
        epoch = run_length_encode(epoch.iter())
            .map(|(count, val)| [count, *val])
            .flatten()
            .collect();
    }
    epoch.len()
}

fn part1() -> usize {
    recursive_rle_len(40)
}

fn part2() -> usize {
    recursive_rle_len(50)
}

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
