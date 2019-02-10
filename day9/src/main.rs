#![allow(dead_code)]
#![feature(vecdeque_rotate)]
use std::collections::VecDeque;

type Score = usize;

fn marble_game(num_players: usize, n_marbles: usize) -> Score {
    let mut board = VecDeque::with_capacity(n_marbles + 1);
    board.push_back(0);

    let mut scores = vec![0; num_players];

    for marble in 1..=n_marbles {
        if marble % 23 == 0 {
            // Pop the marble 7 positions counter-clockwise
            board.rotate_right(7);

            scores[marble % num_players] +=
                marble + board.pop_back().expect("There is always a marble");

            // Mark adjacent one as current
            board.rotate_left(1);
        } else {
            board.rotate_left(1);
            board.push_back(marble);
        }
    }

    *scores
        .iter()
        .max()
        .expect("There is always some score present")
}

#[test]
fn test_part1() {
    assert_eq!(marble_game(9, 25), 32);
    assert_eq!(marble_game(17, 1104), 2764);
    assert_eq!(marble_game(10, 1618), 8317);
}

fn main() {
    println!("Part 1: {}", marble_game(426, 72058));
    println!("Part 2: {}", marble_game(426, 7_205_800));
}
