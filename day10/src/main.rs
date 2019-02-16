use lazy_static::lazy_static;
use log::{debug, log};
use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Coordinate {
    pub fn advance(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
struct CoordinateGrid {
    coordinates: Vec<Coordinate>,
}

impl CoordinateGrid {
    pub fn from_coordiantes(coordinates: Vec<Coordinate>) -> Self {
        CoordinateGrid { coordinates }
    }

    pub fn advance(&mut self) {
        self.coordinates.iter_mut().for_each(|c| c.advance())
    }

    pub fn bounds(&self) -> (i32, i32) {
        let x_max = self.coordinates.iter().max_by_key(|c| c.x).unwrap().x;
        let y_max = self.coordinates.iter().max_by_key(|c| c.y).unwrap().y;
        let x_min = self.coordinates.iter().min_by_key(|c| c.x).unwrap().x;
        let y_min = self.coordinates.iter().min_by_key(|c| c.y).unwrap().y;

        let x_range = x_max.abs() - x_min.abs() + 1;
        let y_range = y_max.abs() - y_min.abs() + 1;

        (x_range, y_range)
    }
}

impl Display for CoordinateGrid {
    //noinspection ALL
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        let x_max = self.coordinates.iter().max_by_key(|c| c.x).unwrap().x;
        let y_max = self.coordinates.iter().max_by_key(|c| c.y).unwrap().y;
        let x_min = self.coordinates.iter().min_by_key(|c| c.x).unwrap().x;
        let y_min = self.coordinates.iter().min_by_key(|c| c.y).unwrap().y;

        let x_normalized = x_max.abs() - x_min.abs() + 1;
        let y_normalized = y_max.abs() - y_min.abs() + 1;

        let mut grid = vec![vec![0; y_normalized as usize]; x_normalized as usize];

        for c in self.coordinates.iter() {
            let normalized_x = if c.x <= 0 {
                (c.x + x_min.abs()) as usize
            } else {
                (c.x - x_min.abs()) as usize
            };

            let normalized_y = if c.y <= 0 {
                (c.y + y_min.abs()) as usize
            } else {
                (c.y - y_min.abs()) as usize
            };

            grid[normalized_x][normalized_y] = 1;
        }

        for row in 0..y_normalized as usize {
            for value in 0..x_normalized as usize {
                if grid[value][row] == 1 {
                    write!(f, "# ")?
                } else {
                    write!(f, ". ")?
                }
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

impl FromStr for Coordinate {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref COORDINATES: Regex = Regex::new(
                r#"position=<\s*([-\d]+),\s*([-\d]+)> velocity=<\s*([-\d]+),\s*([-\d]+)>"#
            )
            .expect("This is a valid regex");
        }

        if let Some(m) = COORDINATES.captures(s) {
            Ok(Coordinate {
                x: m[1].parse()?,
                y: m[2].parse()?,
                vx: m[3].parse()?,
                vy: m[4].parse()?,
            })
        } else {
            err!("Invalid input {}", s)
        }
    }
}

fn part1(input: &str, n_seconds: u32, box_size: u32) -> Result<()> {
    let mut coordinates = vec![];

    for line in input.lines() {
        coordinates.push(Coordinate::from_str(line)?);
    }

    let mut board = CoordinateGrid::from_coordiantes(coordinates);

    let mut attempts = 0;
    let mut seconds = 0;

    while attempts <= 5 && seconds <= n_seconds {
        let (x, y) = board.bounds();

        if x as u32 <= box_size && y as u32 <= box_size {
            attempts += 1;
            println!("{}", seconds);
            println!("{}", board);
        }

        seconds += 1;
        board.advance();
    }

    if attempts == 0 {
        return err!("Didn't find any message");
    }

    Ok(())
}

#[test]
fn test_part1() {
    let test_input =
        read_input_from_file("/Users/omerba/Workspace/AOC2018/day10/input/test").unwrap();

    part1(&test_input, 3, 10).unwrap();
}

#[test]
fn test_draw() {
    let test_input =
        read_input_from_file("/Users/omerba/Workspace/AOC2018/day10/input/coords").unwrap();

    let mut coordinates = vec![];

    for line in test_input.lines() {
        coordinates.push(Coordinate::from_str(line).unwrap());
    }

    let mut board = CoordinateGrid::from_coordiantes(coordinates);

    for i in 0..10333 {
        board.advance();
    }

    println!("{}", board);
}

#[test]
fn test_coordinate_parsing() {
    let input = "position=<-3,  6> velocity=< 2, -1>";
    assert_eq!(
        Coordinate::from_str(input).unwrap(),
        Coordinate {
            x: -3,
            y: 6,
            vx: 2,
            vy: -1
        }
    )
}

#[test]
fn test_coordinate_parsing_spaces() {
    env_logger::init();
    let input = "position=< -3,  6> velocity=< -2, -1>";
    assert_eq!(
        Coordinate::from_str(input).unwrap(),
        Coordinate {
            x: -3,
            y: 6,
            vx: -2,
            vy: -1
        }
    )
}

fn read_input_from_file(path: impl AsRef<Path>) -> Result<String> {
    let f = File::open(path)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;

    Ok(input)
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day10/input/coords");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;
    part1(&input, 10334, 100)?;
    Ok(())
}
