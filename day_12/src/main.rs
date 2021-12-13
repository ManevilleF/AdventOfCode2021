use std::collections::HashMap;
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
                let (f, t) = line
                    .split_once('-')
                    .map(|(f, t)| (Cave::from(f.to_owned()), Cave::from(t.to_owned())))
                    .ok_or_else(|| format!("Invalid line: {}", line))?;
                m.entry(f.clone()).or_insert_with(Vec::new).push(t.clone());
                m.entry(t).or_insert_with(Vec::new).push(f);
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
        mut double_pass: bool,
    ) {
        match cave {
            Cave::End => return paths.push(path),
            Cave::Start if path.contains(cave) => return,
            Cave::Small(_) if path.contains(cave) && double_pass => double_pass = false,
            Cave::Small(_) if path.contains(cave) && !double_pass => return,
            _ => (),
        }
        path.push(cave.clone());
        if let Some(caves) = self.0.get(cave) {
            for ncave in caves.iter() {
                self.path_builder(ncave, path.clone(), paths, double_pass);
            }
        }
    }

    fn path_count(&self, double_path: bool) -> usize {
        let mut paths = vec![];
        self.path_builder(&Cave::Start, vec![], &mut paths, double_path);
        paths.len()
    }
}

fn main() {
    let map = CaveSystem::from_str(&std::fs::read_to_string(FILE_PATH).unwrap()).unwrap();
    println!("Part1. Path count = {}", map.path_count(false));
    println!("Part2. Path count = {}", map.path_count(true));
}
