use std::{fs::File, io::{BufReader, BufRead}};

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn distance_to_beacon(&self) -> i64 {
        (self.position.x - self.closest_beacon.x).abs() + (self.position.y - self.closest_beacon.y).abs()
    }

    fn checked_cells_on_line(&self, y: i64) -> Option<(i64, i64)> {
        // println!("{:?}", self);
        let distance_to_beacon = self.distance_to_beacon();
        // println!("distance: {}", distance_to_beacon);
        let distance_to_y = (self.position.y - y).abs();
        if distance_to_y > distance_to_beacon {
            None
        } else {
            let x_left = self.position.x - distance_to_beacon + distance_to_y;
            let x_right = self.position.x + distance_to_beacon - distance_to_y;
            if self.closest_beacon.y == y {
                if self.closest_beacon.x == x_left {
                    Some((x_left + 1, x_right))
                } else {
                    Some((x_left, x_right -1))
                }
            } else {
                Some((x_left, x_right))
            }
        }
    }



    fn checked_cells_on_line_with_restricted_x(&self, y: i64, min_x: i64, max_x: i64) -> Option<(i64, i64)> {
        // println!("{:?}", self);
        let distance_to_beacon = self.distance_to_beacon();
        // println!("distance: {}", distance_to_beacon);
        let distance_to_y = (self.position.y - y).abs();
        if distance_to_y > distance_to_beacon {
            None
        } else {
            let x_left = self.position.x - distance_to_beacon + distance_to_y;
            let x_right = self.position.x + distance_to_beacon - distance_to_y;
            // if self.closest_beacon.y == y {
            //     if self.closest_beacon.x == x_left {
            //         Some((min_x.max(x_left + 1), max_x.min(x_right)))
            //     } else {
            //         Some((min_x.max(x_left), max_x.min(x_right -1)))
            //     }
            // } else {
                Some((min_x.max(x_left), max_x.min(x_right)))
            // }
        }
    }
}

fn can_be_merged((a1, b1): (i64, i64), (a2, b2): (i64, i64)) -> bool {
    if b1 + 1 < a2 {
        false
    } else if b2 + 1 < a1 {
        false
    } else {
        true
    }
}

fn merge_intervals(intervals: Vec<(i64, i64)>) -> (i64, i64) {
    let begin = intervals.iter().map(|itv| itv.0).min().unwrap();
    let end = intervals.iter().map(|itv| itv.1).max().unwrap();
    (begin, end)
}

fn flatten_interval_vec(cells: Vec<Option<(i64, i64)>>) -> Vec<(i64, i64)> {
    let non_empty_intervals = cells.iter().filter_map(|interval| *interval).collect::<Vec<(i64, i64)>>();
    let mut merged_intervals: Vec<(i64, i64)> = vec![];
    for interval in non_empty_intervals {
        let mut indexes_to_remove = merged_intervals.iter().enumerate().filter(|(_, itv)| can_be_merged(interval, **itv)).map(|(index, _)| index).collect::<Vec<usize>>();
        indexes_to_remove.reverse();
        let mut to_merge = vec![interval];
        for index in indexes_to_remove {
            to_merge.push(merged_intervals.remove(index));
        }
        merged_intervals.push(merge_intervals(to_merge));
        
    }
    merged_intervals
}

fn count_cells(cells: Vec<Option<(i64, i64)>>) -> usize {
    let merged_intervals = flatten_interval_vec(cells);
    println!("Merged interval {:?}", merged_intervals);
    merged_intervals.iter().map(|interval| (interval.1 - interval.0 + 1) as usize).sum()
}

fn spot_hole_in_intervals(intervals: Vec<(i64, i64)>, min_x: i64, max_x: i64) -> Option<i64> {
    if intervals.is_empty() || intervals.len() > 2 {
        unreachable!()
    } else if intervals.len() == 1 {
        let interval = intervals[0];
        if interval.0 > min_x {
            Some(min_x)
        } else if interval.1 < max_x {
            Some(max_x)
        } else {
            None
        }
    } else {
        let first_interval = intervals[0];
        if first_interval.0 == min_x {
            Some(first_interval.1 + 1)
        } else {
            Some(first_interval.0 - 1)
        }
    }
}

fn main() {
    let file_name = "input/day-15";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    // x=2, y=18
    let re = Regex::new(r"Sensor at x=(?P<x1>[-]?\d+), y=(?P<y1>[-]?\d+): closest beacon is at x=(?P<x2>[-]?\d+), y=(?P<y2>[-]?\d+)").unwrap();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_distance_to_beacon = 0;

    let sensors: Vec<Sensor> = reader.lines().map(|line| {
        let l = line.unwrap();
        let capture = re.captures(l.as_str()).unwrap();
        let position = Point { x: capture["x1"].parse().unwrap(), y: capture["y1"].parse().unwrap() };
        let closest_beacon = Point { x: capture["x2"].parse().unwrap(), y: capture["y2"].parse().unwrap() };
        min_x = min_x.min(position.x).min(closest_beacon.x);
        max_x = max_x.max(position.x).max(closest_beacon.x);
        min_y = min_y.min(position.y).min(closest_beacon.y);
        max_y = max_y.max(position.y).max(closest_beacon.y);
        let sensor = Sensor{ position, closest_beacon };
        max_distance_to_beacon = max_distance_to_beacon.max(sensor.distance_to_beacon());
        sensor
    }).collect::<Vec<Sensor>>();

    // let line_2m = sensors.iter().map(|sensor| sensor.checked_cells_on_line(2_000_000)).collect::<Vec<Option<(i64, i64)>>>();
    // println!("line 10 {:?}", line_10);
    // let count = count_cells(line_2m);
    // println!("foo {}", count);
    let min_x = 0;
    let max_x = 4000000;
    let min_y = 0;
    let max_y = 4000000;
    for y in min_y..=max_y {
        let line = sensors.iter().map(|sensor| sensor.checked_cells_on_line_with_restricted_x(y, min_x, max_x)).collect::<Vec<Option<(i64, i64)>>>();
        let flattened_intervals = flatten_interval_vec(line);
        // println!("y {} -> {:?}", y, line);
        if let Some(x) = spot_hole_in_intervals(flattened_intervals, min_x, max_x) {
            println!("x{} y {} -> {}", x, y, 4000000 * x + y);
            break;
        }
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{Sensor, Point};
}