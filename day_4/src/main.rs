use std::collections::HashMap;
use std::str::FromStr;

const FILE_PATH: &str = "input.txt";

#[derive(Debug, Clone)]
struct BingoResults(Vec<u32>);

impl FromStr for BingoResults {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(str::parse)
                .collect::<Result<Vec<u32>, Self::Err>>()?,
        ))
    }
}

#[derive(Debug, Clone)]
struct BoardNumber {
    number: u32,
    marked: bool,
}

#[derive(Debug, Clone)]
struct BoardLine([BoardNumber; 5]);

impl FromStr for BoardLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .split_ascii_whitespace()
            .map(|str| {
                str.parse()
                    .map(|number| BoardNumber {
                        number,
                        marked: false,
                    })
                    .map_err(|e| format!("Invalid line {}: {}", str, e))
            })
            .collect::<Result<Vec<BoardNumber>, Self::Err>>()?;
        let line = line
            .try_into()
            .map_err(|_| format!("{} doesn't have 5 valid elements", s))?;
        Ok(Self(line))
    }
}

#[derive(Debug, Clone)]
struct Board([BoardLine; 5]);

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(BoardLine::from_str)
            .collect::<Result<Vec<BoardLine>, Self::Err>>()?;
        let lines = lines
            .try_into()
            .map_err(|_| format!("{} doesn't have 5 valid elements", s))?;
        Ok(Self(lines))
    }
}

impl Board {
    fn unmarked_sum(&self) -> u32 {
        self.0
            .iter()
            .map(|line| {
                line.0
                    .iter()
                    .filter(|n| !n.marked)
                    .map(|n| n.number)
                    .sum::<u32>()
            })
            .sum()
    }

    fn marked_coords(&self) -> Vec<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .fold(vec![], |mut res, (y, line)| {
                let values: Vec<(usize, usize)> = line
                    .0
                    .iter()
                    .enumerate()
                    .filter(|(_x, n)| n.marked)
                    .map(|(x, _number)| (x, y))
                    .collect();
                res.extend(values);
                res
            })
    }

    fn is_completed(&self) -> bool {
        let marked_coords = self.marked_coords();
        for (x_ref, y_ref) in &marked_coords {
            let x_line = marked_coords.iter().filter(|(x, _y)| x == x_ref).count();
            let y_line = marked_coords.iter().filter(|(_x, y)| y == y_ref).count();
            if x_line == 5 || y_line == 5 {
                return true;
            }
        }
        false
    }

    fn mark_number(&mut self, marked_number: u32) {
        for line in &mut self.0 {
            for number in &mut line.0 {
                if number.number == marked_number {
                    number.marked = true;
                }
            }
        }
    }

    fn handle_marked_number(&mut self, number: u32) -> Option<u32> {
        self.mark_number(number);
        self.is_completed().then(|| self.unmarked_sum())
    }
}

fn find_winning_board(results: &BingoResults, mut boards: Vec<Board>) -> Option<(usize, u32)> {
    for result in &results.0 {
        for (board_id, board) in boards.iter_mut().enumerate() {
            if let Some(sum) = board.handle_marked_number(*result) {
                return Some((board_id, *result * sum));
            }
        }
    }
    None
}

fn find_loosing_board(results: &BingoResults, boards: Vec<Board>) -> Option<(usize, u32)> {
    let mut current_boards: HashMap<usize, Board> = boards.into_iter().enumerate().collect();
    for result in &results.0 {
        let winning_boards = current_boards
            .iter_mut()
            .fold(vec![], |mut res, (id, board)| {
                if let Some(sum) = board.handle_marked_number(*result) {
                    res.push((*id, *result * sum));
                }
                res
            });
        for (id, sum) in winning_boards {
            if current_boards.len() == 1 {
                return Some((id, sum));
            }
            current_boards.remove(&id);
        }
    }
    None
}

fn main() {
    let mut contents: Vec<String> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .split("\n\n")
        .map(ToString::to_string)
        .collect();
    let bingo_results = BingoResults::from_str(&contents.remove(0)).unwrap();
    let boards: Vec<Board> = contents
        .iter()
        .cloned()
        .map(|str| Board::from_str(&str).unwrap())
        .collect();
    let (winner_id, winner_result) = find_winning_board(&bingo_results, boards.clone()).unwrap();
    println!("Part 1: id = {} result = {}", winner_id, winner_result);
    let (looser_id, looser_result) = find_loosing_board(&bingo_results, boards).unwrap();
    println!("Part 2: id = {} result = {}", looser_id, looser_result);
}
