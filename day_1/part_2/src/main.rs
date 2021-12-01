const FILE_PATH: &str = "../input.txt";

fn sum(values: &[u32], i: usize) -> u32 {
    [0, 1, 2].into_iter().filter_map(|delta| {
        values.get(i + delta)
    }).sum()
}

fn main() {
    let file_content = std::fs::read_to_string(FILE_PATH).unwrap();
    let values: Vec<u32> = file_content
        .split('\n')
        .filter_map(|str| str.parse().ok())
        .collect();
    let mut previous_value = None;
    let res = (0..values.len().saturating_sub(2)).filter(|i| {
        let v = sum(&values, *i);
        let res = previous_value.map_or(false, |pv| v > pv);
        previous_value = Some(v);
        res
    }).count();
    println!("Count: {}", res);
}
