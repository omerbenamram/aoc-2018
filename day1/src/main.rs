use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn part1(input: &str) -> Result<i32> {
    let mut freq = 0_i32;
    for line in input.lines() {
        freq += line.parse::<i32>()?
    }
    Ok(freq)
}

fn part2(input: &str) -> Result<i32> {
    let mut seen = HashSet::new();

    let mut freq = 0_i32;
    seen.insert(0);

    loop {
        for line in input.lines().into_iter() {
            let change = line.parse::<i32>()?;
            freq += change;

            if seen.contains(&freq) {
                return Ok(freq);
            } else {
                seen.insert(freq);
            }
        }
    }
}

#[test]
fn test_part2_1() {
    let input = ["+1", "-1"].join("\r\n");
    assert_eq!(part2(&input).unwrap(), 0);
}

#[test]
fn test_part2_2() {
    let input = ["-6", "+3", "+8", "+5", "-6"].join("\r\n");
    assert_eq!(part2(&input).unwrap(), 5);
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day1/input/frequencies");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);

    Ok(())
}
