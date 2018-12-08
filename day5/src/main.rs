use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn have_different_case(a: char, b: char) -> bool {
    a.is_ascii_uppercase() ^ b.is_ascii_uppercase()
}

fn are_same_letter(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase()
}
pub fn part1(polymer: &mut Iterator<Item = char>) -> String {
    let mut stack = vec![];

    // We create a stack that will represent the final string.
    // For each value we want to insert:
    //  1. We compare it to the top of the stack - if they match, we collapse them both.
    //  2. Repeat until no need to collapse.
    //  3. Push the safe char into the stack.
    fn _polymer_reaction(stack: &mut Vec<char>, feed: &mut Iterator<Item = char>) {
        if let Some(c) = feed.next() {
            if let Some(peek) = stack.last() {
                if have_different_case(c, *peek) & are_same_letter(c, *peek) {
                    stack.pop().unwrap();
                    return _polymer_reaction(stack, feed);
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
            return _polymer_reaction(stack, feed);
        }
    }

    _polymer_reaction(&mut stack, polymer);

    stack.into_iter().collect()
}

const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn part2(polymer: &str) -> usize {
    LETTERS
        .iter()
        .map(|letter| {
            part1(
                &mut polymer
                    .chars()
                    .filter(|c| c.to_ascii_lowercase() != *letter),
            )
            .len()
        })
        .min()
        .expect("It should not be empty")
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&mut "aa".chars()),
        "aa".to_string(),
        "It should not collapse letters of same case."
    );
    assert_eq!(
        part1(&mut "aaddddd".chars()),
        "aaddddd".to_string(),
        "It should not collapse letters of same case."
    );
    assert_eq!(
        part1(&mut "aabAAB".chars()),
        "aabAAB".to_string(),
        "It should not collapse letters of same case."
    );
    assert_eq!(part1(&mut "cCcc".chars()), "cc".to_string());
    assert_eq!(
        part1(&mut "abBA".chars()),
        "".to_string(),
        "It should collapse."
    );
    assert_eq!(
        part1(&mut "dabAcCaCBAcCcaDA".chars()),
        "dabCBAcaDA".to_string()
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part1(
            &mut "dabAcCaCBAcCcaDA"
                .chars()
                .filter(|c| c.to_ascii_lowercase() != 'b')
        )
        .len(),
        8
    );
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day5/input/polymer");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;

    println!("{}", part1(&mut input.chars()).len());
    println!("{}", part2(&input));

    Ok(())
}
