const FILE_PATH: &str = "input.txt";

enum ChunkChar {
    OpeningChar(char),
    ClosingChar(char),
}

impl ChunkChar {
    fn new(c: char) -> Self {
        match c {
            '<' => Self::OpeningChar('>'),
            '(' => Self::OpeningChar(')'),
            '[' => Self::OpeningChar(']'),
            '{' => Self::OpeningChar('}'),
            '>' | ')' | ']' | '}' => Self::ClosingChar(c),
            _ => panic!("Unhandled char `{}`", c),
        }
    }
}

fn handle_line(line: &[char]) -> Result<Vec<char>, char> {
    let mut expected_chars = vec![];
    for c in line.iter().copied().map(ChunkChar::new) {
        match c {
            ChunkChar::OpeningChar(ec) => expected_chars.push(ec),
            ChunkChar::ClosingChar(gc) => match expected_chars.pop() {
                None => return Err(gc),
                Some(ec) => {
                    if ec != gc {
                        return Err(gc);
                    }
                }
            },
        }
    }
    Ok(expected_chars)
}

fn main() {
    let (score_a, mut score_b) = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|l| handle_line(&l))
        .fold((0, vec![]), |(mut score_a, mut score_b), r| {
            match r {
                Ok(v) => score_b.push(v.iter().rev().fold(0_u64, |res, c| {
                    res * 5
                        + match c {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        }
                })),
                Err(e) => {
                    score_a += match e {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    }
                }
            }
            (score_a, score_b)
        });
    println!("Part1. Score: {}", score_a);
    score_b.sort_unstable();
    println!("Part2. Score: {:?}", score_b.get(score_b.len() / 2));
}
