use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

type Err = Box<::std::error::Error>;
type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub enum GuardAction {
    BeginsShift(i32),
    WakesUp,
    FallsAsleep,
}

impl FromStr for GuardAction {
    type Err = Err;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref regex: Regex =
                Regex::new(r"Guard #(?P<id>\d+).*").expect("This should be a valid regex");
        }
        match regex.captures(s) {
            // Guard #<id>..
            Some(matches) => Ok(GuardAction::BeginsShift(matches["id"].parse()?)),
            None => match s {
                "wakes up" => Ok(GuardAction::WakesUp),
                "falls asleep" => Ok(GuardAction::FallsAsleep),
                _ => Err(From::from(format!("Unknown action {}", s))),
            },
        }
    }
}

pub type Day = u32;
pub type GuardID = i32;

#[derive(Default)]
pub struct SleepingHabits(HashMap<Day, [bool; 60]>);
pub struct SleepTracker(HashMap<GuardID, SleepingHabits>);

impl SleepingHabits {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn iter(&self) -> ::std::collections::hash_map::Iter<Day, [bool; 60]> {
        self.0.iter()
    }

    pub fn log_range_asleep(&mut self, day: Day, range: Range<u32>) {
        let chart_for_day = self.0.entry(day).or_insert([false; 60]);

        for minute in range {
            chart_for_day[minute as usize] = true
        }
    }

    pub fn total_time_asleep(&self) -> i32 {
        // For each guard, calculate total sleepy minutes
        self.iter()
            .map(|(_, sleeping_table)| sleeping_table.iter().filter(|b| **b).count())
            .sum::<usize>() as i32
    }

    pub fn total_time_asleep_by_minute(&self) -> [i32; 60] {
        // Flatten into an [i32; 60] of total sleep per minute.
        let mut asleep_by_minute_total = [0; 60];

        for sleep_table in self.iter().map(|(_, values)| values) {
            for (i, v) in sleep_table.iter().enumerate() {
                if *v {
                    asleep_by_minute_total[i] += 1;
                }
            }
        }

        asleep_by_minute_total
    }

    /// Returns a tuple of the minute index, and total times slept during that minute.
    pub fn sleepiest_minute(&self) -> (i32, i32) {
        let total_asleept_by_minute = self.total_time_asleep_by_minute();
        let (sleepiest_idx, total_minutes_slept) = total_asleept_by_minute
            .iter()
            .enumerate()
            .max_by_key(|(idx, &times_asleep)| times_asleep)
            .expect("Cannot be empty");

        (sleepiest_idx as i32, *total_minutes_slept)
    }
}

impl SleepTracker {
    pub fn from_sorted_rows(sorted_rows: &[Row]) -> Result<Self> {
        let mut sleep_tracker = HashMap::new();

        let mut current_guard_id = None;
        let mut last_minute_asleep = None;

        for row in sorted_rows.iter() {
            let current_day = row.date.day();
            let action = GuardAction::from_str(row.text)?;

            match action {
                GuardAction::BeginsShift(id) => current_guard_id = Some(id),
                GuardAction::FallsAsleep => {
                    last_minute_asleep = Some(row.date.minute());
                }
                GuardAction::WakesUp => {
                    let key =
                        current_guard_id.ok_or("invalid parser state, no current_guard_id")?;

                    let start_marker = last_minute_asleep.ok_or("invalid parser state, guard was never asleep when \"wakes up\" event was triggered")?;

                    let guard_entry = sleep_tracker.entry(key).or_insert_with(SleepingHabits::new);
                    guard_entry.log_range_asleep(current_day, start_marker..row.date.minute());
                }
            }
        }

        Ok(SleepTracker(sleep_tracker))
    }

    pub fn iter(&self) -> ::std::collections::hash_map::Iter<GuardID, SleepingHabits> {
        self.0.iter()
    }
}

impl Debug for SleepTracker {
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), fmt::Error> {
        writeln!(f, "Date\tID\tMinute")?;
        writeln!(
            f,
            "    \t  \t000000000011111111112222222222333333333344444444445555555555"
        )?;
        writeln!(
            f,
            "    \t  \t012345678901234567890123456789012345678901234567890123456789"
        )?;

        for (gurad_id, habits) in self.iter() {
            for (day, minutes) in habits.iter() {
                writeln!(
                    f,
                    "{}",
                    &format!(
                        "{:4}\t{:2}\t{}",
                        day,
                        gurad_id,
                        minutes
                            .iter()
                            .map(|&b| if b { '#' } else { '.' })
                            .collect::<String>()
                    )
                )?;
            }
        }

        Ok(())
    }
}

fn sanitize_input(input: &str) -> Result<Vec<Row>> {
    let mut rows = Vec::new();

    for line in input.lines() {
        rows.push(Row::from_str(line)?);
    }

    rows.sort_unstable_by_key(|row| row.date);
    Ok(rows)
}

pub fn part1(input: &str) -> Result<i32> {
    let rows = sanitize_input(input)?;

    let sleep_tracker = SleepTracker::from_sorted_rows(&rows)?;

    let (sleepiest_guard_id, total_time_slept, sleepiest_minute) = sleep_tracker
        .iter()
        .map(|(&id, sleeping_habits)| {
            let total_asleep = sleeping_habits.total_time_asleep();
            let sleepiest_minute = sleeping_habits.sleepiest_minute();

            (id, total_asleep, sleepiest_minute.0)
        })
        .max_by_key(|tup| tup.1)
        .ok_or("The list cannot be empty")?;

    println!(
        "Sleepiest guard is #{}, slept for total of {} minutes, sleepiest minute is {}",
        sleepiest_guard_id, total_time_slept, sleepiest_minute
    );
    Ok(sleepiest_guard_id * sleepiest_minute as i32)
}

pub fn part2(input: &str) -> Result<i32> {
    let rows = sanitize_input(input)?;

    let sleep_tracker = SleepTracker::from_sorted_rows(&rows)?;

    let (sleepiest_guard_id, sleepiest_minute, sleepiest_minute_frequency) = sleep_tracker
        .iter()
        .map(|(&id, sleeping_habits)| {
            // Flatten into an [i32; 60] of total sleep per minute.
            let sleepiest_minute = sleeping_habits.sleepiest_minute();
            (id, sleepiest_minute.0, sleepiest_minute.1)
        })
        .max_by_key(|tup| tup.2)
        .expect("Max");

    println!(
        "Sleepiest guard is #{}, slept during minute {} for {} times!",
        sleepiest_guard_id, sleepiest_minute, sleepiest_minute_frequency
    );

    Ok(sleepiest_guard_id * sleepiest_minute as i32)
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Row<'a> {
    date: NaiveDateTime,
    text: &'a str,
}

impl<'a> Row<'a> {
    pub fn from_str(s: &'a str) -> Result<Self> {
        lazy_static! {
            static ref regex: Regex = Regex::new(r"\[(?P<date>[^\[]+)]\s(?P<text>[\s\w#]+)")
                .expect("This should be a valid regex");
        }

        let captures = match regex.captures(s) {
            Some(matches) => matches,
            None => return Err(From::from(format!("Row {} failed to match regex", s))),
        };

        let date = NaiveDateTime::parse_from_str(
            captures.name("date").unwrap().as_str(),
            "%Y-%m-%d %H:%M",
        )
        .unwrap();

        Ok(Row {
            date,
            text: captures.name("text").unwrap().as_str(),
        })
    }
}
#[test]
fn test_part1() {
    let test_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    let input = sanitize_input(test_input).unwrap();
    let tracker = SleepTracker::from_sorted_rows(&input).unwrap();
    println!("{:?}", &tracker);
    assert_eq!(part1(test_input).unwrap(), 240);
}

#[test]
fn test_part2() {
    let test_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    let input = sanitize_input(test_input).unwrap();
    let tracker = SleepTracker::from_sorted_rows(&input).unwrap();
    println!("{:?}", &tracker);
    assert_eq!(part2(&test_input).unwrap(), 4455);
}

#[test]
fn test_row_from_input() {
    assert_eq!(
        Row::from_str("[1518-11-22 23:58] Guard #3463 begins shift").unwrap(),
        Row {
            date: NaiveDateTime::new(
                NaiveDate::from_ymd(1518, 11, 22),
                NaiveTime::from_hms(23, 58, 0)
            ),
            text: "Guard #3463 begins shift"
        }
    )
}

fn main() -> Result<()> {
    let input = PathBuf::from("/Users/omerba/Workspace/AOC2018/day4/input/sleep_times");
    let f = File::open(input)?;
    let mut f = BufReader::new(f);

    let mut input = String::new();

    f.read_to_string(&mut input)?;

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);

    Ok(())
}
