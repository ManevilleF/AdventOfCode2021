#![allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Not};

const FILE_PATH: &str = "test2.txt";

macro_rules! substr {
    ($str:expr, $range:expr) => {
        $str.get($range)
            .ok_or(format!("Can't get sub str {:?} from {}", $range, $str))?
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left = 0,
    Right = 1,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ReduceResponse {
    Explode((u32, u32)),
    MoveValue { value: u32, dir: Direction },
    Done,
}

#[derive(Clone)]
struct NumberPair([Number; 2]);

#[derive(Clone)]
enum Number {
    Regular(u32),
    Pair(Box<NumberPair>),
}

impl NumberPair {
    fn reduce(&mut self, depth: usize) -> Option<ReduceResponse> {
        let mut res = [None, None];
        for direction in [Direction::Left, Direction::Right] {
            let opposite_dir = !direction;
            let i = direction as usize;
            let opp_i = opposite_dir as usize;
            res[i] = match &mut self.0[i] {
                Number::Regular(v) => {
                    if *v >= 10 {
                        let div = *v as f32 / 2.0;
                        self.0[i] = Number::Pair(Box::new(Self([
                            Number::Regular(div.floor() as u32),
                            Number::Regular(div.ceil() as u32),
                        ])));
                        return Some(ReduceResponse::Done);
                    }
                    Some(*v)
                }
                Number::Pair(p) => {
                    if let Some(res) = p.reduce(depth + 1) {
                        return match res {
                            ReduceResponse::Explode((l, r)) => {
                                self.0[i] = Number::Regular(0);
                                self.0[opp_i].add_value([l, r][opp_i], direction);
                                Some(ReduceResponse::MoveValue {
                                    value: [l, r][i],
                                    dir: opposite_dir,
                                })
                            }
                            ReduceResponse::MoveValue { value, dir } => {
                                if dir == direction {
                                    self.0[opp_i].add_value(value, dir);
                                    Some(ReduceResponse::Done)
                                } else {
                                    Some(ReduceResponse::MoveValue { value, dir })
                                }
                            }
                            ReduceResponse::Done => Some(ReduceResponse::Done),
                        };
                    }
                    None
                }
            }
        }
        if let [Some(left), Some(right)] = res {
            if depth >= 4 {
                return Some(ReduceResponse::Explode((left, right)));
            }
        }
        None
    }
}

impl Number {
    fn parse(s: &str) -> Result<(Self, usize), String> {
        let mut index = 1;
        let res = match s.chars().next().ok_or(format!("str is empty {}", s))? {
            '[' => {
                let (left, size) = Self::parse(substr!(s, index..))?;
                index += size;
                if substr!(s, index..=index) != "," {
                    return Err(format!("Expected a `,` at {} for {}", index, s));
                }
                index += 1;
                let (right, size) = Self::parse(substr!(s, index..))?;
                index += size + 1;
                Self::Pair(Box::new(NumberPair([left, right])))
            }
            ']' => return Err(String::from("Found an unexpected `]`")),
            v => Self::Regular(v.to_digit(10).ok_or(format!("Invalid number char {}", v))?),
        };
        Ok((res, index))
    }

    fn reduce(&mut self) {
        while self.reduce_once().is_some() {
            //        println!("{:?}", self);
        }
    }

    fn reduce_once(&mut self) -> Option<ReduceResponse> {
        if let Self::Pair(p) = self {
            p.reduce(0)
        } else {
            None
        }
    }

    fn reduced(mut self) -> Self {
        self.reduce();
        self
    }

    fn add_value(&mut self, value: u32, dir: Direction) {
        match self {
            Number::Regular(v) => *v += value,
            Number::Pair(p) => p.0[dir as usize].add_value(value, dir),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Number::Regular(v) => *v,
            Number::Pair(p) => p.0[0].magnitude() * 3 + p.0[1].magnitude() * 2,
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Number::Regular(v) => v.to_string(),
                Number::Pair(b) => format!("[{:?},{:?}]", b.0[0], b.0[1]),
            }
        )
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(NumberPair([self, rhs]))).reduced()
    }
}

fn main() {
    let numbers: Vec<Number> = std::fs::read_to_string(FILE_PATH)
        .unwrap()
        .lines()
        .map(|l| Number::parse(l).unwrap().0)
        .collect();
    let sum = numbers
        .iter()
        .cloned()
        .reduce(|acc, v| {
            println!("{:?}\n+ {:?}", acc, v);
            let res = acc + v;
            println!("= {:?}\n", res);
            res
        })
        .unwrap();
    println!("Sum = {}", sum.magnitude());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_1() {
        let str = "[[[[[9,8],1],2],3],4]";
        let (mut number, _) = Number::parse(str).unwrap();
        assert_eq!(format!("{:?}", number), "[[[[[9,8],1],2],3],4]".to_string());
        number.reduce();
        assert_eq!(format!("{:?}", number), "[[[[0,9],2],3],4]".to_string());
    }

    #[test]
    fn test_reduce_2() {
        let str = "[7,[6,[5,[4,[3,2]]]]]";
        let (mut number, _) = Number::parse(str).unwrap();
        assert_eq!(format!("{:?}", number), "[7,[6,[5,[4,[3,2]]]]]".to_string());
        number.reduce();
        assert_eq!(format!("{:?}", number), "[7,[6,[5,[7,0]]]]".to_string());
    }

    #[test]
    fn test_reduce_3() {
        let str = "[[6,[5,[4,[3,2]]]],1]";
        let (mut number, _) = Number::parse(str).unwrap();
        assert_eq!(format!("{:?}", number), "[[6,[5,[4,[3,2]]]],1]".to_string());
        number.reduce();
        assert_eq!(format!("{:?}", number), "[[6,[5,[7,0]]],3]".to_string());
    }

    #[test]
    fn test_reduce_4() {
        let str = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let (mut number, _) = Number::parse(str).unwrap();
        assert_eq!(
            format!("{:?}", number),
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string()
        );
        number.reduce();
        assert_eq!(
            format!("{:?}", number),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string()
        );
    }

    #[test]
    fn test_reduce_single() {
        let str = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let (mut number, _) = Number::parse(str).unwrap();
        assert_eq!(
            format!("{:?}", number),
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string()
        );
        number.reduce_once();
        assert_eq!(
            format!("{:?}", number),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string()
        );
    }

    #[test]
    fn test_small_sum() {
        let (a, _) = Number::parse("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let (b, _) = Number::parse("[1,1]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string()
        );
    }

    #[test]
    fn test_sum_1() {
        let list = "\
         [1,1]\n\
         [2,2]\n\
         [3,3]\n\
         [4,4]\n\
         ";
        let numbers: Vec<Number> = list.lines().map(|l| Number::parse(l).unwrap().0).collect();
        let sum = numbers.iter().cloned().reduce(|acc, v| acc + v).unwrap();
        assert_eq!(
            format!("{:?}", sum),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string()
        );
    }

    #[test]
    fn test_sum_2() {
        let list = "\
         [1,1]\n\
         [2,2]\n\
         [3,3]\n\
         [4,4]\n\
         [5,5]\n\
         ";
        let numbers: Vec<Number> = list.lines().map(|l| Number::parse(l).unwrap().0).collect();
        let sum = numbers.iter().cloned().reduce(|acc, v| acc + v).unwrap();
        assert_eq!(
            format!("{:?}", sum),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_string()
        );
    }

    #[test]
    fn test_sum_3() {
        let list = "\
         [1,1]\n\
         [2,2]\n\
         [3,3]\n\
         [4,4]\n\
         [5,5]\n\
         [6,6]\n\
         ";
        let numbers: Vec<Number> = list.lines().map(|l| Number::parse(l).unwrap().0).collect();
        let sum = numbers.iter().cloned().reduce(|acc, v| acc + v).unwrap();
        assert_eq!(
            format!("{:?}", sum),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_1() {
        let (a, _) = Number::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").unwrap();
        let (b, _) = Number::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_2() {
        let (a, _) =
            Number::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").unwrap();
        let (b, _) = Number::parse("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_3() {
        let (a, _) =
            Number::parse("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]").unwrap();
        let (b, _) =
            Number::parse("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_4() {
        let (a, _) =
            Number::parse("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]").unwrap();
        let (b, _) = Number::parse("[7,[5,[[3,8],[1,4]]]]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_5() {
        let (a, _) =
            Number::parse("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]").unwrap();
        let (b, _) = Number::parse("[[2,[2,2]],[8,[8,1]]]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_6() {
        let (a, _) =
            Number::parse("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]").unwrap();
        let (b, _) = Number::parse("[2,9]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_7() {
        let (a, _) = Number::parse("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]").unwrap();
        let (b, _) = Number::parse("[1,[[[9,3],9],[[9,0],[0,7]]]]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_8() {
        let (a, _) =
            Number::parse("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]").unwrap();
        let (b, _) = Number::parse("[[[5,[7,4]],7],1]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]".to_string()
        );
    }

    #[test]
    fn test_sum_large_9() {
        let (a, _) = Number::parse("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]").unwrap();
        let (b, _) = Number::parse("[[[[4,2],2],6],[8,7]]").unwrap();
        assert_eq!(
            format!("{:?}", a + b),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string()
        );
    }
}
