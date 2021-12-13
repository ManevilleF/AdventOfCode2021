const FILE_PATH: &str = "input.txt";

fn simulate_one_day(timers: &mut [usize; 9]) {
    timers.rotate_left(1);
    timers[6] += timers[8];
}

fn main() {
    let mut timers = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .fold([0_usize; 9], |mut map, timer| {
            if let Some(v) = map.get_mut(timer) {
                *v += 1;
            }
            map
        });
    for day in 1..=256 {
        simulate_one_day(&mut timers);

        if day == 80 {
            println!("Day {} = {}", day, timers.iter().sum::<usize>());
        }
    }
    println!("Day 256 = {}", timers.iter().sum::<usize>());
}
