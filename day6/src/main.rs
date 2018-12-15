use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::mem;
use std::str::FromStr;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use colored::*;
use core::borrow::Borrow;
#[cfg(test)]
use env_logger::try_init;
use log::debug;
use std::collections::HashSet;
use std::fmt::Write;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

struct Grid {
    pub inner: Vec<Vec<char>>,
    pub origins: HashSet<Point>,
}

struct IterPoints<'a> {
    grid: &'a Grid,
    x: i32,
    y: i32,
}

impl Iterator for IterPoints<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let dim = self.grid.size() as i32;
        if (self.x >= dim) & (self.y >= dim) {
            return None;
        }

        let p = Point {
            x: self.x,
            y: self.y,
        };

        if self.x <= dim {
            self.x += 1;
        } else {
            self.x = 0;
            self.y += 1;
        }

        Some(p)
    }
}

impl Grid {
    pub fn new(sz: usize, origins: &[Point]) -> Self {
        let mut matrix = Vec::with_capacity(sz);

        for _ in 0..sz {
            matrix.push(Vec::with_capacity(sz))
        }

        for row in matrix.iter_mut().take(sz) {
            for _ in 0..sz {
                row.push('?');
            }
        }

        let mut hs = HashSet::new();
        for p in origins.iter().cloned() {
            hs.insert(p);
        }

        Grid {
            inner: matrix,
            origins: hs,
        }
    }

    pub fn points(&self) -> IterPoints {
        IterPoints {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
    
    pub fn set_value(&mut self, p: &Point, v: char) {
        self.inner[p.y as usize][p.x as usize] = v;
    }

    pub fn count(&self, c: char) -> i32 {
        self.inner
            .iter()
            .map(|col| col.iter().map(|some_c| *some_c == c).filter(|x| *x).count() as i32)
            .sum()
    }

    pub fn print(&self) -> Result<String> {
        let mat = &self.inner;
        let mut f = String::new();

        write!(f, "    ")?;
        for n in 0..mat.len() {
            write!(f, "{:3}", n)?;
        }
        writeln!(f)?;
        for row in 0..mat.len() {
            write!(f, "{:4}: ", row)?;
            for col in 0..mat.len() {
                match mat[row][col] {
                    '?' => write!(f, "{:2}", '?')?,
                    _ => {
                        if self.origins.contains(
                            Point {
                                x: col as i32,
                                y: row as i32,
                            }
                            .borrow(),
                        ) {
                            write!(f, "{:2}", format!("{}", mat[row][col]).red())?
                        } else {
                            write!(f, "{:2}", format!("{}", mat[row][col]).green())?
                        }
                    }
                }

                if col != mat.len() - 1 {
                    write!(f, ",")?;
                }
            }
            writeln!(f)?;
        }

        Ok(f)
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RECTANGLES_PARSER: Regex =
                Regex::new(r"(?P<x>\d+),\s+(?P<y>\d+)").expect("It should be a valid regex");
        }

        let matches = match RECTANGLES_PARSER.captures(s) {
            Some(matches) => matches,
            None => return Err(From::from(format!("{} failed to match regex", s))),
        };

        Ok(Point {
            x: matches["x"].parse()?,
            y: matches["y"].parse()?,
        })
    }
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        return vec![
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
        ];
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn round(
    matrix: &mut Grid,
    points: &[(Point, char)],
    diverging: &mut HashSet<char>,
) -> Vec<(Point, char)> {
    let mut temp = vec![];
    let mut visited: HashSet<Point> = HashSet::new();

    for (p, origin) in points.iter() {
        if ((p.x < 0) | (p.y < 0)) | ((p.x >= matrix.size() as i32) | (p.y >= matrix.size() as i32))
        {
            // divergence
            diverging.insert(*origin);
            continue;
        }

        match matrix.inner[p.y as usize][p.x as usize] {
            // Free spot
            '?' => {
                matrix.set_value(p, *origin);
                visited.insert(p.clone());
            }
            '.' => continue,
            c if c == *origin => continue,
            _ if visited.contains(p) => {
                matrix.set_value(p, '.');
                continue;
            }
            _ => {
                continue;
            }
        }

        for n in p.neighbors() {
            temp.push((n, *origin))
        }
    }

    temp
}

fn part2(input: &str, matrix_size: usize, cap: i32) -> Result<i32> {
    let mut points = vec![];

    for line in input.lines() {
        points.push(Point::from_str(line)?);
    }

    let mut matrix = Grid::new(matrix_size, &points);
    let mut found = Vec::new();

    for p in matrix.points() {
        let mut total_distance = 0;
        for origin in points.iter() {
            total_distance += p.manhattan_distance(origin);
            if total_distance > cap {
                break;
            }
        }

        if total_distance < cap {
            debug!("{:?} -> {}", p, total_distance);
            found.push(p)
        }
    }

    for point in found.iter() {
        matrix.set_value(point, '#');
    }
    debug!("\n{}", matrix.print()?);

    Ok(found.len() as i32)
}

fn part1(input: &str, matrix_size: usize) -> Result<i32> {
    let mut points = vec![];

    for line in input.lines() {
        points.push(Point::from_str(line)?);
    }

    let mut matrix = Grid::new(matrix_size, &points);
    let mut diverging = HashSet::new();
    let mut q = Vec::new();

    let mut letters_used = Vec::new();

    for (point, letter) in points.iter().cloned().zip((b'a'..b'z').chain(b'A'..b'Z')) {
        letters_used.push(char::from(letter));
        q.push((point, char::from(letter)))
    }

    let mut points = round(&mut matrix, &q, &mut diverging);

    loop {
        debug!("{} enqueued", points.len());
        let new = round(&mut matrix, &points, &mut diverging);
        if points.is_empty() {
            break;
        }

        mem::replace(&mut points, new);
    }

    debug!("Diverged: {:?}", diverging);
    debug!("\n{}", matrix.print()?);
    let letter_to_count: HashMap<char, i32> = letters_used
        .iter()
        .filter(|c| !diverging.contains(c))
        .map(|c| (*c, matrix.count(*c)))
        .collect();
    debug!("Counts: {:#?}", letter_to_count);

    Ok(*letter_to_count.values().max().expect("Matrix is not empty"))
}

#[test]
fn test_part1() {
    try_init().ok();
    let test_input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

    assert_eq!(part1(test_input, 20).unwrap(), 17);
}

#[test]
fn test_part2() {
    try_init().ok();
    let test_input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

    assert_eq!(part2(test_input, 20, 32).unwrap(), 16);
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day6/input/rectangles");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;

    println!("{}", part1(&input, 1000)?);
    println!("{}", part2(&input, 1000, 10000)?);

    Ok(())
}
