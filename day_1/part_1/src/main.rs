const FILE_PATH: &str = "../input.txt";

// Short Version

fn main() {
    let file_content = std::fs::read_to_string(FILE_PATH).unwrap();
    let (_, count) = file_content
        .split('\n')
        .filter_map(|str| str.parse::<u32>().ok())
        .fold((None, 0u32), |(prev, count), v| {
            let count = if v > prev.unwrap_or(v) {
                count + 1
            } else {
                count
            };
            (Some(v), count)
        });
    println!("Count: {}", count);
}

// Complete Version

// use std::fs;
//
// fn get_int_vec(str: String) -> Vec<u32> {
//     let split_str: Vec<&str> = str.split('\n').collect();
//     let mut res = Vec::with_capacity(split_str.len());
//     for str in split_str {
//         let v: u32 = match str.parse() {
//             Ok(u) => u,
//             Err(e) => {
//                 log::error!("Failed to parse u32: {}", e);
//                 continue;
//             }
//         };
//         res.push(v);
//     }
//     res
// }
//
// pub enum MeasurementDelta {
//     None,
//     Increased,
//     Decreased,
// }
//
// impl MeasurementDelta {
//     pub fn display(&self) -> String {
//         format!(
//             "({})",
//             match self {
//                 Self::None => "N/A - no previous measurement",
//                 Self::Increased => "increased",
//                 Self::Decreased => "decreased",
//             }
//         )
//     }
//
//     pub fn get(previous: u32, new: u32) -> Self {
//         if new > previous {
//             Self::Increased
//         } else {
//             Self::Decreased
//         }
//     }
// }
//
// fn main() {
//     env_logger::init();
//
//     let result = fs::read_to_string(FILE_PATH);
//     let file_content = match result {
//         Ok(v) => v,
//         Err(e) => panic!("ERROR: {}", e),
//     };
//     let vec = get_int_vec(file_content);
//
//     let mut previous = None;
//     let mut count = 0;
//     for value in vec {
//         let delta = match previous {
//             None => MeasurementDelta::None,
//             Some(pv) => MeasurementDelta::get(pv, value),
//         };
//         if let MeasurementDelta::Increased = delta {
//             count += 1;
//         };
//         log::info!("{} {}", value, delta.display());
//         previous = Some(value);
//     }
//     println!("Increased Count: {}", count);
// }
