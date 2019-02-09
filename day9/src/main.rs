#![allow(dead_code)]
#![feature(vecdeque_rotate)]
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

type PlayerId = u32;
type Score = u32;

type MarbleValue = u32;
struct MarblesGame {
    board: VecDeque<MarbleValue>,
    scores: HashMap<PlayerId, Score>,
    n_players: u32,
    current_player: PlayerId,
    // Position in the vector of the current marble
    current_marble: usize,
}

impl MarblesGame {
    pub fn new(n_players: u32, max_marble_value: usize) -> Self {
        let mut initial_board = VecDeque::with_capacity(max_marble_value);
        initial_board.push_back(0);

        let mut initial_scores = HashMap::new();

        for player in 1..=n_players {
            initial_scores.insert(player as PlayerId, 0);
        }

        MarblesGame {
            board: initial_board,
            scores: initial_scores,
            n_players,
            current_player: 0,
            current_marble: 0,
        }
    }

    fn add_regular_marble(&mut self, marble: MarbleValue) {
        self.board.rotate_left(1);
        self.board.push_back(marble);
    }

    fn add_special_marble(&mut self, marble: MarbleValue) {
        // First, the current player keeps the marble they would have placed,
        // adding it to their score.
        self.scores
            .entry(self.current_player)
            .and_modify(|score| *score += marble);

        // In addition, the marble 7 marbles counter-clockwise from the current marble
        // is removed from the circle and also added to the current player's score.
        self.board.rotate_right(7);
        let next = self.board.pop_back().unwrap();
        self.board.rotate_left(1);
        self.scores
            .entry(self.current_player)
            .and_modify(|score| *score += next);
    }

    pub fn add_marble(&mut self, marble: u32) {
        let marble_is_special = marble % 23 == 0;

        if marble_is_special {
            self.add_special_marble(marble)
        } else {
            self.add_regular_marble(marble)
        }

        if self.current_player == self.n_players {
            self.current_player = 1;
        } else {
            self.current_player += 1;
        }
    }

    pub fn high_score(&self) -> Score {
        *self
            .scores
            .values()
            .max()
            .expect("There is always some score present")
    }
}

fn marble_game(num_players: u32, n_marbles: u32) -> Score {
    let mut game = MarblesGame::new(num_players, n_marbles as usize);

    for marble in 1..=n_marbles {
        game.add_marble(marble);
    }

    game.high_score()
}

#[test]
fn test_part1() {
    assert_eq!(marble_game(9, 25), 32);
    assert_eq!(marble_game(17, 1104), 2764);
    assert_eq!(marble_game(10, 1618), 8317);
}

fn main() {
    println!("Part 1: {}", marble_game(426, 72058));
    println!("Part 2: {}", marble_game(426, 7205800));
}
