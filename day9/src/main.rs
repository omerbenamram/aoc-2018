#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt::Display;
use std::iter::FromIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Marble = u32;
type PlayerId = u32;
type Score = u32;

struct MarblesGame {
    board: LinkedList<Marble>,
    scores: HashMap<PlayerId, Score>,
    n_players: u32,
    current_player: PlayerId,
    current_marble: usize,
}

impl MarblesGame {
    pub fn new(n_players: u32) -> Self {
        let mut initial_board = LinkedList::new();
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

    /// Insert marble into board without copying all data.
    fn insert_at(&mut self, idx: usize, marble: Marble) {
        let right = self.board.split_off(idx);
        self.board.push_back(marble);
        self.board.extend(right);
    }

    fn remove_at(&mut self, idx: usize) -> Marble {
        let right = self.board.split_off(idx + 1);
        let marble = self
            .board
            .pop_back()
            .unwrap_or_else(|| panic!("A marble should be present at idx {}", idx));

        self.board.extend(right);
        marble
    }

    fn add_regular_marble(&mut self, marble: Marble) {
        let last_marble_idx = self.board.len() - 1;

        // If current marble is last, we start from front of list
        let marble_position: usize = if self.current_marble == last_marble_idx {
            1
        } else {
            self.current_marble + 2
        };

        self.insert_at(marble_position, marble);
        self.current_marble = marble_position;
    }

    fn add_special_marble(&mut self, marble: Marble) {
        // First, the current player keeps the marble they would have placed,
        // adding it to their score.
        self.scores
            .entry(self.current_player)
            .and_modify(|score| *score += marble);

        // In addition, the marble 7 marbles counter-clockwise from the current marble
        // is removed from the circle and also added to the current player's score.
        let next_marble_to_remove = if self.current_marble.saturating_sub(7) == 0 {
            let offset_from_front = 7 - self.current_marble;
            self.board.len() - offset_from_front
        } else {
            self.current_marble - 7
        };

        let additional_marble = self.remove_at(next_marble_to_remove);
        self.scores
            .entry(self.current_player)
            .and_modify(|score| *score += additional_marble);

        self.current_marble = next_marble_to_remove;
    }

    pub fn add_marble(&mut self, marble: Marble) {
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
    let mut game = MarblesGame::new(num_players);

    for marble in 1..=n_marbles {
        game.add_marble(marble);
        // println!("{}", game);
    }

    game.high_score()
}

impl Display for MarblesGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "[{}]", self.current_player)?;
        for (i, marble) in self.board.iter().enumerate() {
            if i == self.current_marble {
                write!(f, " ({}) ", marble)?;
            } else {
                write!(f, " {} ", marble)?;
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
