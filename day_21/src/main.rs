#![allow(clippy::cast_possible_truncation)]
use std::collections::HashMap;

const BOARD_SIZE: u32 = 10;

#[derive(Debug, Copy, Clone)]
struct Player {
    score: u32,
    position: u8,
}

impl Player {
    const fn new(position: u8) -> Self {
        Self { score: 0, position }
    }

    fn apply_dice_delta(&mut self, delta: u32) {
        let mut pos = (u32::from(self.position) + delta) % BOARD_SIZE;
        if pos == 0 {
            pos = 10;
        }
        self.score += pos;
        self.position = pos as u8;
    }
}

#[derive(Debug, Default)]
struct TestDice {
    value: u32,
    count: u32,
}

impl TestDice {
    fn throw(&mut self) -> u32 {
        self.count += 3;
        (0..3)
            .map(|_| {
                self.value = self.value % 100 + 1;
                self.value
            })
            .sum()
    }
}

fn main() {
    let p1 = Player::new(7);
    let p2 = Player::new(1);
    // Part 1
    {
        let mut p1 = p1;
        let mut p2 = p2;
        let mut dice = TestDice::default();

        let mut i = 0;
        while p1.score < 1000 && p2.score < 1000 {
            let throw = dice.throw();
            if i % 2 == 0 {
                p1.apply_dice_delta(throw);
            } else {
                p2.apply_dice_delta(throw);
            }
            i += 1;
        }
        let looser = if p1.score < 1000 { &p1 } else { &p2 };
        println!("Part 1: {}", dice.count * looser.score);
    }
    // Part 2
    {
        let possible_throws: HashMap<u32, u64> =
            vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
                .into_iter()
                .collect();
        let (mut p1_wins, mut p2_wins) = (0_u64, 0_u64);
        let mut stack = vec![(0, (p1, p2), 1)];
        while let Some((i, (p1, p2), u)) = stack.pop() {
            if p1.score >= 21 {
                p1_wins += u;
            } else if p2.score >= 21 {
                p2_wins += u;
            } else {
                for (throw, count) in &possible_throws {
                    if i % 2 == 0 {
                        let mut player = p1;
                        player.apply_dice_delta(*throw);
                        stack.push((i + 1, (player, p2), u * count));
                    } else {
                        let mut player = p2;
                        player.apply_dice_delta(*throw);
                        stack.push((i + 1, (p1, player), u * count));
                    }
                }
            }
        }
        println!(
            "Part 2: P1 wins {} times and P2 {} times. Max = {}",
            p1_wins,
            p2_wins,
            p1_wins.max(p2_wins)
        );
    }
}
