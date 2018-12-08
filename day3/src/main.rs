use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::iter;
use std::path::PathBuf;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug, PartialOrd, PartialEq)]
struct Rectangle {
    id: i32,
    offset_h: i32,
    offset_w: i32,
    height: i32,
    width: i32,
}

impl Rectangle {
    pub fn from_descriptor(descriptor: &str) -> Result<Self> {
        lazy_static! {
            static ref RECTANGLES_PARSER: Regex = Regex::new(r"\s*#(?P<id>\d+)\s+@\s+(?P<offset_w>\d+),(?P<offset_h>\d+):\s+(?P<width>\d+)x(?P<height>\d+)").expect("It should be a valid regex");
        }

        let matches = match RECTANGLES_PARSER.captures(descriptor) {
            Some(matches) => matches,
            None => return Err(From::from(format!("{} failed to match regex", descriptor))),
        };

        Ok(Rectangle {
            id: matches["id"].parse()?,
            offset_h: matches["offset_h"].parse()?,
            offset_w: matches["offset_w"].parse()?,
            height: matches["height"].parse()?,
            width: matches["width"].parse()?,
        })
    }
}

#[test]
fn test_rectangle_from_descriptor() {
    assert_eq!(
        Rectangle::from_descriptor("#1 @ 1,3: 4x4").unwrap(),
        Rectangle {
            id: 1,
            offset_h: 3,
            offset_w: 1,
            height: 4,
            width: 4
        }
    )
}
fn zeroed_matrix() -> Vec<Vec<i32>> {
    let mut matrix = Vec::with_capacity(1000);

    for row in 0..1000 {
        matrix.push(Vec::with_capacity(1000))
    }

    for i in 0..1000 {
        for j in 0..1000 {
            matrix[i].push(0);
        }
    }

    matrix
}

fn part1(input: &str) -> Result<i32> {
    let mut matrix = zeroed_matrix();

    for line in input.lines() {
        let rectangle = Rectangle::from_descriptor(&line)?;
        for i in rectangle.offset_h..rectangle.offset_h + rectangle.height {
            for j in rectangle.offset_w..rectangle.offset_w + rectangle.width {
                matrix[i as usize][j as usize] += 1;
            }
        }
    }

    return Ok(matrix
        .iter()
        .map(|row| row.iter().filter(|cell| **cell > 1).count() as i32)
        .sum());
}

fn part2(input: &str) -> Result<i32> {
    let mut matrix = zeroed_matrix();
    let mut rectangles = Vec::new();

    for line in input.lines() {
        rectangles.push(Rectangle::from_descriptor(&line)?);
    }

    for rectangle in rectangles.iter() {
        for i in rectangle.offset_h..rectangle.offset_h + rectangle.height {
            for j in rectangle.offset_w..rectangle.offset_w + rectangle.width {
                matrix[i as usize][j as usize] += 1;
            }
        }
    }

    // Now check what part two is asking for by doing another
    // iteration and finding which rectangles are overlapping.
    let mut overlapping_rectangles: HashSet<i32> = rectangles.iter().map(|r| r.id).collect();

    'outer: for rectangle in rectangles.iter() {
        for i in rectangle.offset_h..rectangle.offset_h + rectangle.height {
            for j in rectangle.offset_w..rectangle.offset_w + rectangle.width {
                if matrix[i as usize][j as usize] > 1 {
                    overlapping_rectangles.remove(&rectangle.id);
                    continue 'outer;
                }
            }
        }
    }

    // There should only be one id left!
    if overlapping_rectangles.len() == 1 {
        Ok(overlapping_rectangles.drain().next().unwrap())
    } else {
        Err(From::from("Failed to locate the needed ID"))
    }
}

#[test]
fn test_part1() {
    let test_input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    assert_eq!(part1(test_input).unwrap(), 4);
}

#[test]
fn test_part2() {
    let test_input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    assert_eq!(part2(test_input).unwrap(), 3);
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day3/input/rectangles");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);

    Ok(())
}
