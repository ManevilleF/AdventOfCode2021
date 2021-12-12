use std::collections::HashMap;
use std::str::FromStr;

const FILE_PATH: &str = "test.txt";

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl From<String> for Cave {
    fn from(s: String) -> Self {
        match s.as_str() {
            "start" => Self::Start,
            "end" => Self::End,
            _ if s.to_lowercase() == s => Self::Small(s),
            _ => Self::Big(s),
        }
    }
}

#[derive(Debug)]
struct CaveSystem(HashMap<Cave, Vec<Cave>>);

impl FromStr for CaveSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Result<HashMap<Cave, Vec<Cave>>, String> =
            s.lines().try_fold(HashMap::new(), |mut m, line| {
                let (from, to) = line
                    .split_once('-')
                    .ok_or_else(|| format!("Invalid line: {}", line))?;
                let entry = m
                    .entry(Cave::from(from.to_string()))
                    .or_insert_with(Vec::new);
                entry.push(Cave::from(to.to_string()));
                Ok(m)
            });
        Ok(Self(map?))
    }
}

fn main() {
    let map = CaveSystem::from_str(&std::fs::read_to_string(FILE_PATH).unwrap()).unwrap();
    println!("{:#?}", map);
}
