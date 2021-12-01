#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    nonstandard_style
)]

const FILE_PATH: &str = "../input.txt";

fn get_sum(sums: &mut [u32], delta: usize, i: usize) -> Option<&mut u32> {
    let pos = i.checked_sub(delta)?;
    sums.get_mut(pos)
}

fn main() {
    let file_content = std::fs::read_to_string(FILE_PATH).unwrap();
    let values: Vec<u32> = file_content
        .split('\n')
        .filter_map(|str| str.parse().ok())
        .collect();
    let mut sums: Vec<u32> = values
        .clone()
        .drain(..values.len().saturating_sub(2))
        .collect();
    for (i, v) in values.into_iter().enumerate() {
        for delta in [1, 2] {
            if let Some(sum) = get_sum(&mut sums, delta, i) {
                *sum += v;
            }
        }
    }
    let mut previous_value = None;
    let res = sums
        .into_iter()
        .filter_map(|v| {
            let res = previous_value.and_then(|pv| if v > pv { Some(()) } else { None });
            previous_value = Some(v);
            res
        })
        .count();
    println!("Count: {}", res);
}
