#![allow(dead_code)]
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

pub type Result<T> = result::Result<T, Box<Error>>;

#[derive(Debug)]
struct Node {
    children: Option<Vec<Node>>,
    metadata: Vec<i32>,
}

impl Node {
    fn from_input_stream(stream: &mut impl Iterator<Item = i32>) -> Result<Node> {
        let number_of_children = match stream.next() {
            Some(value) => value,
            None => return err!("Unexpected EOF"),
        };
        let number_of_metadata_entries = match stream.next() {
            Some(value) => value,
            None => return err!("Unexpected EOF"),
        };

        let mut children = vec![];
        let mut metadata = vec![];

        for _ in 0..number_of_children {
            children.push(Node::from_input_stream(stream)?);
        }

        for _ in 0..number_of_metadata_entries {
            if let Some(number) = stream.next() {
                metadata.push(number);
            }
        }

        if !children.is_empty() {
            Ok(Node {
                children: Some(children),
                metadata,
            })
        } else {
            Ok(Node {
                children: None,
                metadata,
            })
        }
    }

    pub fn metadata_sum(&self) -> i32 {
        let mut sum = self.metadata.iter().sum();

        if let Some(ref children) = self.children {
            for child in children.iter() {
                sum += child.metadata_sum()
            }
        }

        sum
    }

    pub fn value(&self) -> i32 {
        if let Some(ref children) = self.children {
            let mut children_sum = 0;

            for position in self.metadata.iter() {
                // indices are 1-based
                if let Some(child) = children.get((*position - 1) as usize) {
                    children_sum += child.value();
                }
            }
            children_sum
        } else {
            self.metadata.iter().sum()
        }
    }
}

fn parse_input(input: &str) -> Result<Node> {
    let mut numbers: Vec<i32> = vec![];

    for number in input.split_ascii_whitespace() {
        numbers.push(number.parse()?);
    }

    let mut numbers = numbers.into_iter();
    Ok(Node::from_input_stream(&mut numbers)?)
}

fn part1(tree: &Node) -> i32 {
    tree.metadata_sum()
}

fn part2(tree: &Node) -> i32 {
    tree.value()
}

#[test]
fn test_part1() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    let tree = parse_input(input).unwrap();

    assert_eq!(part1(&tree), 138);
}

#[test]
fn test_part2() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    let tree = parse_input(input).unwrap();

    assert_eq!(part1(&tree), 138);
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day8/input/tree");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;
    let tree = parse_input(&input)?;

    println!("Part 1: {}", part1(&tree));
    println!("Part 2: {}", part2(&tree));
    Ok(())
}
