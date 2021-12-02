const FILE_PATH: &str = "input.txt";

fn get_increasing_count(values: impl Iterator<Item = u32>) -> u32 {
    let (_, count) = values.fold((None, 0), |(prev, mut count), v| {
        if v > prev.unwrap_or(v) {
            count += 1;
        }
        (Some(v), count)
    });
    count
}

fn main() {
    let file_content: Vec<u32> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split('\n')
        .filter_map(|str| str.parse().ok())
        .collect();

    println!(
        "Part 1: {}",
        get_increasing_count(file_content.iter().copied())
    );

    let triple_sums = (0..file_content.len().saturating_sub(2)).map(|i| {
        (0..=2)
            .filter_map(|delta| file_content.get(i + delta))
            .sum::<u32>()
    });
    println!("Part 2: {}", get_increasing_count(triple_sums));
}