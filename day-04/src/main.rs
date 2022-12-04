use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Debug)]
struct Assignment {
    startSection: u32,
    endSection: u32,
}

impl From<String> for Assignment {
    fn from(input: String) -> Self {
        let split: Vec<&str> = input.split("-").collect();
        if split.len() == 2 {
            Assignment {
                startSection: split.first().unwrap().parse::<u32>().unwrap(),
                endSection: split.last().unwrap().parse::<u32>().unwrap(),
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(PartialEq, Debug)]
struct AssignmentPair {
    assignment1: Assignment,
    assignment2: Assignment,
}

impl From<String> for AssignmentPair {
    fn from(input: String) -> Self {
        let split: Vec<&str> = input.split(",").collect();
        if split.len() == 2 {
            AssignmentPair {
                assignment1: Assignment::from(split.first().unwrap().to_string()),
                assignment2: Assignment::from(split.last().unwrap().to_string()),
            }
        } else {
            unreachable!()
        }
    }
}

impl AssignmentPair {
    fn has_a_full_overlap(&self) -> bool {
        (self.assignment1.startSection >= self.assignment2.startSection
            && self.assignment1.endSection <= self.assignment2.endSection)
            || (self.assignment2.startSection >= self.assignment1.startSection
                && self.assignment2.endSection <= self.assignment1.endSection)
    }

    fn has_a_partial_overlap(&self) -> bool {
        (self.assignment1.startSection <= self.assignment2.startSection
            && self.assignment2.startSection <= self.assignment1.endSection)
            || (self.assignment2.startSection <= self.assignment1.startSection
                && self.assignment1.startSection <= self.assignment2.endSection)
    }
}

fn main() {
    let file_name = "input/day-04";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let assignment_pairs = reader
        .lines()
        .map(|line| AssignmentPair::from(line.unwrap()))
        .collect::<Vec<AssignmentPair>>();

    let number_of_full_overlaps = assignment_pairs
        .iter()
        .filter(|pair| pair.has_a_full_overlap())
        .count();
    println!("{} pairs fully overlap", number_of_full_overlaps);

    let number_of_partial_overlaps = assignment_pairs
        .iter()
        .filter(|pair| pair.has_a_partial_overlap())
        .count();
    println!("{} pairs partialy overlap", number_of_partial_overlaps);
}

#[cfg(test)]
mod tests {
    use crate::{Assignment, AssignmentPair};

    #[test]
    fn assignment_from_string() {
        let input = "2-4".to_string();
        let assignment = Assignment::from(input);

        assert_eq!(assignment.startSection, 2);
        assert_eq!(assignment.endSection, 4);
    }

    #[test]
    fn assignment_pair_from_string() {
        let input = "2-4,6-8".to_string();
        let assignment_pair = AssignmentPair::from(input);

        assert_eq!(
            assignment_pair,
            AssignmentPair {
                assignment1: Assignment {
                    startSection: 2,
                    endSection: 4
                },
                assignment2: Assignment {
                    startSection: 6,
                    endSection: 8
                }
            }
        );
    }

    #[test]
    fn no_full_overlap() {
        let input = "2-4,6-8".to_string();
        let assignment_pair = AssignmentPair::from(input);

        assert!(!assignment_pair.has_a_full_overlap());
    }

    #[test]
    fn full_overlap() {
        let input = "2-8,3-7".to_string();
        let assignment_pair = AssignmentPair::from(input);

        assert!(assignment_pair.has_a_full_overlap());
    }

    #[test]
    fn no_partial_overlap() {
        let input = "2-4,6-8".to_string();
        let assignment_pair = AssignmentPair::from(input);

        assert!(!assignment_pair.has_a_partial_overlap());
    }

    #[test]
    fn full_overlap_is_partial_overlap() {
        let input = "2-8,3-7".to_string();
        let assignment_pair = AssignmentPair::from(input);

        assert!(assignment_pair.has_a_partial_overlap());
    }

    #[test]
    fn partial_overlap() {
        let input = "5-7,7-9".to_string();
        let assignment_pair = AssignmentPair::from(input);

        assert!(assignment_pair.has_a_partial_overlap());
    }
}
