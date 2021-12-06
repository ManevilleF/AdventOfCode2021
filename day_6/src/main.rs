use std::collections::HashMap;

const FILE_PATH: &str = "input.txt";

fn simulate_one_day(timers: &mut HashMap<u8, usize>) {
    *timers = timers
        .iter()
        .fold(HashMap::default(), |mut map, (key, value)| {
            let new_key = key.checked_sub(1).unwrap_or_else(|| {
                *map.entry(8).or_insert(0) += value;
                6
            });
            *map.entry(new_key).or_insert(0) += value;
            map
        });
}

fn main() {
    let mut timers = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<u8>().ok())
        .fold(HashMap::default(), |mut map, timer| {
            *map.entry(timer).or_insert(0) += 1;
            map
        });
    for day in 1..=256 {
        simulate_one_day(&mut timers);

        if day == 80 {
            println!("Day {} = {}", day, timers.values().sum::<usize>());
        }
    }
    println!("Day 256 = {}", timers.values().sum::<usize>());
}
