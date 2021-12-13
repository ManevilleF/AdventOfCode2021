const FILE_PATH: &str = "input.txt";

fn handle_line(mut line: impl Iterator<Item = char>) -> Result<Vec<char>, char> {
    line.try_fold(vec![], |mut expected_chars, c| {
        match c {
            '<' => expected_chars.push('>'),
            '(' => expected_chars.push(')'),
            '[' => expected_chars.push(']'),
            '{' => expected_chars.push('}'),
            '>' | ')' | ']' | '}' => {
                if !expected_chars.pop().map_or(false, |ec| ec == c) {
                    return Err(c);
                }
            }
            _ => panic!("Unhandled char `{}`", c),
        }
        Ok(expected_chars)
    })
}

const fn part1_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1_197,
        '>' => 25_137,
        _ => 0,
    }
}

const fn part2_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn main() {
    let (score_a, mut score_b) = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|l| handle_line(l.chars()))
        .fold((0_u32, vec![]), |(mut score_a, mut score_b), r| {
            match r {
                Ok(v) => score_b.push(
                    v.iter()
                        .rev()
                        .fold(0_u64, |res, c| res * 5 + part2_score(*c)),
                ),
                Err(e) => score_a += part1_score(e),
            }
            (score_a, score_b)
        });
    println!("Part1. Score: {}", score_a);
    score_b.sort_unstable();
    println!("Part2. Score: {}", score_b.get(score_b.len() / 2).unwrap());
}
