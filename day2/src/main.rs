use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;
use std::{fs::File, io::BufReader, io::Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn frequencies(s: &str) -> HashMap<char, i32> {
    let mut letters = HashMap::with_capacity(26);

    for c in s.chars() {
        let e = letters.entry(c).or_insert(0);
        *e += 1;
    }

    letters
}

fn letters_distance(a: &str, b: &str) -> i32 {
    a.chars()
        .zip(b.chars())
        .fold(0, |acc, t| if t.0 != t.1 { acc + 1 } else { acc })
}

fn common_letters(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|t| t.0 == t.1)
        .map(|t| t.0)
        .collect()
}

fn part2(input: &str) -> Option<(String)> {
    let lines: Vec<&str> = input.lines().into_iter().collect();

    for line in lines.iter() {
        for candidate in lines.iter() {
            if letters_distance(line, candidate) == 1 {
                return Some(common_letters(line, candidate));
            }
        }
    }

    None
}

fn part1(input: &str) -> Result<i32> {
    let mut contains_two_same_letters = 0;
    let mut contains_three_same_letters = 0;

    for line in input.lines().into_iter() {
        let freqs = frequencies(&line);
        if freqs.iter().any(|(k, v)| *v == 2) {
            contains_two_same_letters += 1;
        }
        if freqs.iter().any(|(k, v)| *v == 3) {
            contains_three_same_letters += 1;
        }
    }

    Ok(contains_two_same_letters * contains_three_same_letters)
}

#[test]
fn test_frequencies_1() {
    let freqs = frequencies("bababc");
    assert_eq!(freqs[&'a'], 2, "should have 2*a");
    assert_eq!(freqs[&'b'], 3, "should have 3*b");
    assert_eq!(freqs[&'c'], 1, "should have 1*c");
}

#[test]
fn test_letter_distance() {
    assert_eq!(letters_distance("abcde", "axcye"), 2);
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day2/input/ids");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;

    println!("{}", part1(&input)?);
    println!(
        "{:?}",
        part2(&input).expect(From::from("Failed to found the matching strings"))
    );

    Ok(())
}
