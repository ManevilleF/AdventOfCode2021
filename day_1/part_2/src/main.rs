const FILE_PATH: &str = "../input.txt";

fn main() {
    let file_content = std::fs::read_to_string(FILE_PATH).unwrap();
    let values: Vec<u32> = file_content
        .split('\n')
        .filter_map(|str| str.parse().ok())
        .collect();
    let (_, count) = (0..values.len().saturating_sub(2)).fold((None, 0), |(prev, count), i| {
        let v: u32 = [0, 1, 2]
            .into_iter()
            .filter_map(|delta| values.get(i + delta))
            .sum();
        let count = if v > prev.unwrap_or(v) {
            count + 1
        } else {
            count
        };
        (Some(v), count)
    });
    println!("Count: {}", count);
}
