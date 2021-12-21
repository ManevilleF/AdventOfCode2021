#![allow(clippy::cast_possible_truncation)]
const BOARD_SIZE: u32 = 10;

#[derive(Debug, Clone)]
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
    let mut p1 = Player::new(7);
    let mut p2 = Player::new(1);
    // Part 1
    {
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
}
