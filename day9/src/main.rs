#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt::Display;
use std::iter::FromIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type PlayerId = u32;
type Score = u32;

type MarbleValue = u32;
type MarbleId = usize;

struct MarblesGame {
    board: Vec<Marble>,
    scores: HashMap<PlayerId, Score>,
    n_players: u32,
    current_player: PlayerId,
    // Position in the vector of the current marble
    current_marble: MarbleId,
}

#[derive(Clone)]
struct Marble {
    value: MarbleValue,
    previous_id: MarbleId,
    next_id: MarbleId,
}

impl Marble {
    pub fn with_value(value: u32) -> Self {
        Marble {
            value,
            previous_id: 0,
            next_id: 0,
        }
    }
}

impl MarblesGame {
    pub fn new(n_players: u32, max_marble_value: usize) -> Self {
        let mut initial_board = Vec::with_capacity(max_marble_value);
        initial_board.push(Marble::with_value(0));

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

    fn add_marble_to_board(&mut self, marble: MarbleValue) -> MarbleId {
        let marble = Marble::with_value(marble);
        self.board.push(marble);
        self.board.len() - 1
    }

    /// Insert marble into board without copying all data.
    fn insert_after(&mut self, idx: MarbleId, marble: MarbleValue) -> MarbleId {
        let old_next = self.board[idx].next_id;

        let my_idx = self.add_marble_to_board(marble);
        self.board[my_idx].previous_id = idx;
        self.board[my_idx].next_id = old_next;

        self.board[idx].next_id = my_idx;
        self.board[old_next].previous_id = my_idx;
        my_idx
    }

    fn clockwise(&self, amount: usize) -> MarbleId {
        let mut current = self.current_marble;

        for i in 1..=amount {
            current = self.board[current].next_id
        }

        current
    }

    fn counter_clockwise(&self, amount: usize) -> MarbleId {
        let mut current = self.current_marble;

        for i in 1..=amount {
            current = self.board[current].previous_id
        }

        current
    }

    fn remove_at(&mut self, idx: MarbleId) -> MarbleValue {
        let prev = self.board[idx].previous_id;
        let next = self.board[idx].next_id;

        self.board[prev].next_id = next;
        self.board[next].previous_id = prev;

        self.board[idx].value
    }

    fn add_regular_marble(&mut self, marble: MarbleValue) {
        let current_marble = self.insert_after(self.clockwise(1), marble);
        self.current_marble = current_marble;
    }

    fn add_special_marble(&mut self, marble: MarbleValue) {
        // First, the current player keeps the marble they would have placed,
        // adding it to their score.
        self.scores
            .entry(self.current_player)
            .and_modify(|score| *score += marble);

        // In addition, the marble 7 marbles counter-clockwise from the current marble
        // is removed from the circle and also added to the current player's score.
        let next_marble_to_remove = self.counter_clockwise(7);
        self.current_marble = self.board[next_marble_to_remove].next_id;

        let additional_marble = self.remove_at(next_marble_to_remove);
        self.scores
            .entry(self.current_player)
            .and_modify(|score| *score += additional_marble);
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

impl Display for Marble {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

impl Display for MarblesGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{}]", self.current_player)?;
        let mut marble = self.board[0].clone();
        write!(f, " {} ", marble.value)?;

        loop {
            let next = marble.next_id;
            marble = self.board[next].clone();

            if next == 0 {
                break;
            }

            if next == self.current_marble {
                write!(f, " ({}) ", marble.value)?;
            } else {
                write!(f, " {} ", marble.value)?;
            }
        }

        Ok(())
    }
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
