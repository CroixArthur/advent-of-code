use std::{fs, ops::Range};

#[derive(Debug, Clone)]
pub struct RangeSet {
    ranges: Vec<Range<i64>>,
}

impl RangeSet {
    pub fn new(ranges: Vec<Range<i64>>) -> Self {
        RangeSet { ranges }
    }

    pub fn union(&mut self, other: Self) {
        match (self.ranges.is_empty(), other.ranges.is_empty()) {
            (true, true) => self.ranges.clear(),
            (true, false) => self.ranges = other.ranges,
            (false, true) => (),
            _ => {
                self.ranges.extend(other.ranges);
                self.ranges.sort_by(|a, b| a.start.cmp(&b.start));

                let mut result = Vec::new();
                let mut current = self.ranges[0].clone();

                for range in self.ranges[1..].iter() {
                    if current.end < range.start {
                        result.push(current);
                        current = range.clone();
                    } else {
                        current.end = current.end.max(range.end);
                    }
                }
                result.push(current);

                self.ranges = result;
            }
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for range1 in self.ranges.iter() {
            for range2 in other.ranges.iter() {
                if range1.start < range2.end && range1.end > range2.start {
                    let start = range1.start.max(range2.start);
                    let end = range1.end.min(range2.end);
                    result.push(start..end);
                }
            }
        }
        RangeSet { ranges: result }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = self.ranges.clone();
        for other_range in other.ranges.iter() {
            let mut new_result = Vec::new();
            for range in result.iter() {
                if range.start >= other_range.end || range.end <= other_range.start {
                    new_result.push(range.clone());
                } else {
                    if range.start < other_range.start {
                        new_result.push(range.start..other_range.start);
                    }
                    if range.end > other_range.end {
                        new_result.push(other_range.end..range.end);
                    }
                }
            }
            result = new_result;
        }
        RangeSet { ranges: result }
    }

    pub fn get_first(&self) -> Option<i64> {
        self.ranges.first().map(|r| r.start)
    }

    pub fn iter(&self) -> std::vec::IntoIter<Range<i64>> {
        self.ranges.clone().into_iter()
    }
}

fn part_two(input: &str) -> Option<i64> {
    let mut location_ranges: RangeSet = RangeSet { ranges: Vec::new() };
    let mut ranges_to_add: RangeSet = RangeSet { ranges: Vec::new() };
    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if i == 0 {
            let parsed_seeds = line
                .split(": ")
                .filter(|s| !s.is_empty())
                .nth(1)
                .unwrap()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .chunks(2)
                .map(|s| (s[0], s[1]))
                .collect::<Vec<(i64, i64)>>();

            for (start, range) in parsed_seeds {
                location_ranges.union(RangeSet {
                    ranges: vec![start..start + range],
                });
            }
            continue;
        }
        if line.chars().next().unwrap().is_numeric() {
            let mapping_line = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let destination_start = mapping_line[0];
            let source_start = mapping_line[1];
            let range = mapping_line[2];
            let mapping_difference = destination_start - source_start;
            let mapping_range = RangeSet {
                ranges: vec![source_start..source_start + range],
            };

            let range_diff = mapping_range.intersection(&location_ranges);
            for diff in range_diff.iter() {
                ranges_to_add.union(RangeSet {
                    ranges: vec![diff.start + mapping_difference..diff.end + mapping_difference],
                });
            }
            location_ranges = location_ranges.difference(&range_diff);
            continue;
        }
        location_ranges.union(ranges_to_add);
        ranges_to_add = RangeSet { ranges: Vec::new() };
    }
    location_ranges.union(ranges_to_add);

    location_ranges.get_first()
}

pub fn run() {
  let filepath = "src/inputs/5input.txt";
  
  match fs::read_to_string(filepath) {
    Ok(lines) => match part_two(lines.as_str()) {
      Some(res) => println!("{res}"),
      None => ()
    },
    Err(e) => println!("{e}")
  }
}