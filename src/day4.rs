use std::collections::HashMap;
use regex::Regex;
use std::str::FromStr;
use std::error::Error;
use std::result;
use std::convert::AsRef;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum RecordKind {
    Start { id: u32 },
    Asleep,
    Wake
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Record {
    kind: RecordKind,
    date: DateTime
}

pub struct RecordsByGuard {
    records: HashMap<u32, Vec<Record>>
}

impl RecordsByGuard {
    pub fn new(records: Vec<Record>) -> RecordsByGuard {
        let mut records_by_guard: HashMap<u32, Vec<Record>> = HashMap::new();
        let mut curr_guard = None;
        for record in records {
            if let RecordKind::Start{id} = record.kind {
                curr_guard = Some(id);
            }

            match curr_guard {
                None => panic!("No id set for guard yet.."),
                Some(id) => {
                    records_by_guard.entry(id).or_default().push(record)
                }
            }
        }
        RecordsByGuard { records: records_by_guard }
    }
}

#[derive(Debug)]
pub struct FreqMapByGuard {
    freqmap: HashMap<u32, HashMap<u32, u32>>
}

impl AsRef<FreqMapByGuard> for FreqMapByGuard {
    fn as_ref(&self) -> &FreqMapByGuard {
        &self
    }
}

impl FreqMapByGuard {
    pub fn new(records_by_guard: RecordsByGuard) -> FreqMapByGuard {
        let mut freqmap: HashMap<u32, HashMap<u32, u32>> = HashMap::new(); 
        for (id, guard_records) in records_by_guard.records.iter() {
            let mut sleep_start = None;
            for record in guard_records {
                match record.kind {
                    RecordKind::Asleep => sleep_start = Some(record.date.minute),
                    RecordKind::Wake => {
                        if sleep_start.is_none() {
                            panic!("Unset sleep_start for freqmap");
                        }

                        let curr_minute = record.date.minute;
                        for minute in sleep_start.unwrap()..curr_minute {
                            freqmap.entry(*id)
                                   .or_default()
                                   .entry(minute)
                                   .and_modify(|e| *e += 1)
                                   .or_insert(1);
                        }
                    }
                    RecordKind::Start { id: _ } => {},
                }
            }
        }
        FreqMapByGuard { freqmap: freqmap }
    }

    /// Return the guard id who is most asleep (part 1)
    pub fn most_asleep_guard(&self) -> u32 {
        let mut map = self.freqmap.iter()
                                  .map(|(id, minute_map)| (minute_map.values().sum::<u32>(), id))
                                  .collect::<Vec<_>>();
        map.sort();

        // (asleep_minutes, guard_id)
        *map.last().unwrap().1
    }

    /// Given a guard id, return the (number of times, minute) for the most slept minute
    pub fn minute_most_slept(&self, guard_id: u32) -> (&u32, &u32) {
        let mut minutes = self.freqmap.get(&guard_id)
                                      .unwrap()
                                      .iter()
                                      .map(|(minute, count)| (count, minute))
                                      .collect::<Vec<_>>();
        minutes.sort();
        minutes.last().unwrap().to_owned()
    }
}

type Result<T> = result::Result<T, Box<Error>>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> FreqMapByGuard {
    let mut records: Vec<Record> = input.lines()
                                        .map(|line| line.parse().unwrap_or_else(|e| {
                                            panic!(format!("Failed to parse [{:?}]: {:?}", line, e))
                                        }))
                                        .collect();

    // Sort events by date
    records.sort_by(|rec1, rec2| rec1.date.cmp(&rec2.date));
    let records_by_guard = RecordsByGuard::new(records);
    let freqmap_by_guard = FreqMapByGuard::new(records_by_guard);
    freqmap_by_guard
}

#[aoc(day4, part1)]
pub fn part1(input: &FreqMapByGuard) -> u32 {
    let most_asleep_guard = input.most_asleep_guard();
    let minute_most_asleep = input.minute_most_slept(most_asleep_guard).1;
    minute_most_asleep * most_asleep_guard
}

#[aoc(day4, part2)]
pub fn part2(input: &FreqMapByGuard) -> u32 {
    let mut minutes = Vec::new();
    for guard_id in input.freqmap.keys() {
        let minute_most_slept = input.minute_most_slept(*guard_id);
        minutes.push((minute_most_slept.0, minute_most_slept.1, guard_id))
        
    }
    minutes.sort();
    let answer = minutes.last().unwrap();
    answer.1 * answer.2
}

impl FromStr for Record {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Record> {
        lazy_static! {
            // [1518-11-01 23:58] Guard #99 begins shift
            // [1518-11-02 00:40] falls asleep
            // [1518-11-02 00:50] wakes up
            // (?x) enables insignificant whitespace mode for prettier regex
            static ref RECORD: Regex = Regex::new(r"(?x)
                \[
                    (?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) # match year-month-day
                    \s                                              # space between the date
                    (?P<hour>\d{2}):(?P<minute>\d{2})               # hour:minute
                \]
                .*
                (?:                                                 # Needed for a grouping of possible next matches
                    \#(?P<id>(\d+))|                                # #num - for id
                    (?P<asleep>asleep)|                             # asleep for asleep event
                    (?P<wake>wakes)                                 # wakes for awake event
                )
            ").expect("Failed to create Regex");
        }

        let caps = RECORD.captures(s).expect("Failed to capture line");

        let kind = 
            if let Some(m) = caps.name("id") {
                RecordKind::Start { id: m.as_str().parse()? }
            } else if let Some(_) = caps.name("asleep") {
                RecordKind::Asleep
            } else if let Some(_) = caps.name("wake") {
                RecordKind::Wake
            } else {
                panic!(format!("Could find event kind: {}", s))
            };

        let date = DateTime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?
        };

        Ok(Record { kind: kind, date: date})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator("[1518-11-01 00:00] Guard #10 begins shift
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
                                           [1518-11-05 00:55] wakes up")), 240);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator("[1518-11-01 00:00] Guard #10 begins shift
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
                                           [1518-11-05 00:55] wakes up")), 4455);
    }
}
