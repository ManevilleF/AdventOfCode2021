const FILE_PATH: &str = "input.txt";

fn simulate_one_day(timers: &mut Vec<u8>) {
    let new_timers = timers.iter_mut().fold(vec![], |mut acc, v| {
        match v.checked_sub(1) {
            None => {
                acc.push(8);
                *v = 6;
            }
            Some(s) => *v = s,
        }
        acc
    });
    timers.extend(new_timers.iter());
}

fn main() {
    let mut initial_timers: Vec<u8> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Initial state: {:?}", initial_timers);
    for day in 1..=80 {
        simulate_one_day(&mut initial_timers);
        println!("Day {} = {}", day, initial_timers.len());
    }
}
