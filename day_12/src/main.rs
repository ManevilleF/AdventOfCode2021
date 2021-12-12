use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
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
                let (from, to) = (Cave::from(from.to_string()), Cave::from(to.to_string()));
                let entry = m.entry(from.clone()).or_insert_with(Vec::new);
                entry.push(to.clone());
                let entry = m.entry(to).or_insert_with(Vec::new);
                entry.push(from);
                Ok(m)
            });
        Ok(Self(map?))
    }
}

impl CaveSystem {
    fn path_builder(
        &self,
        cave: &Cave,
        mut path: Vec<Cave>,
        paths: &mut Vec<Vec<Cave>>,
        criteria: impl Fn(&[Cave], &Cave) -> bool + Clone,
    ) {
        match cave {
            Cave::End => paths.push(path),
            Cave::Start if path.contains(cave) => (),
            Cave::Small(_) if criteria(&path, cave) => (),
            _ => {
                path.push(cave.clone());
                if let Some(caves) = self.0.get(cave) {
                    for cave in caves.iter() {
                        self.path_builder(cave, path.clone(), paths, criteria.clone());
                    }
                }
            }
        }
    }

    fn part_1_exclusion_criteria(path: &[Cave], cave: &Cave) -> bool {
        path.contains(cave)
    }

    fn part_2_exclusion_criteria(path: &[Cave], cave: &Cave) -> bool {
        if !path.contains(cave) {
            return false;
        }
        let small_caves: Vec<Cave> = path
            .iter()
            .cloned()
            .filter(|c| matches!(c, Cave::Small(_)))
            .collect();
        let deduped: HashSet<Cave> = small_caves.iter().cloned().collect();
        small_caves.len() != deduped.len()
    }

    fn path_count(&self, criteria: impl Fn(&[Cave], &Cave) -> bool + Clone) -> usize {
        let mut paths = vec![];
        self.path_builder(&Cave::Start, vec![], &mut paths, criteria);
        paths.len()
    }
}

fn main() {
    let map = CaveSystem::from_str(&std::fs::read_to_string(FILE_PATH).unwrap()).unwrap();
    println!(
        "Part1. Path count = {}",
        map.path_count(CaveSystem::part_1_exclusion_criteria)
    );
    println!(
        "Part2. Path count = {}",
        map.path_count(CaveSystem::part_2_exclusion_criteria)
    );
}
