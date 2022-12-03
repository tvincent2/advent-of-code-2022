use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const fn letter_to_priority(letter: char) -> u32 {
    match letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => unreachable!(),
    }
}

#[derive(PartialEq)]
struct RuckSack {
    compartment1: String,
    compartment2: String,
}

impl From<String> for RuckSack {
    fn from(input: String) -> Self {
        match input.len() {
            length if length % 2 == 0 => {
                let half_length = length / 2;
                RuckSack {
                    compartment1: input[..half_length].to_string(),
                    compartment2: input[half_length..].to_string(),
                }
            }
            _ => panic!("length should be an even number"),
        }
    }
}

impl RuckSack {
    fn common_item(&self) -> char {
        for item in self.compartment1.chars() {
            if self.compartment2.contains(item) {
                return item;
            }
        }
        unreachable!()
    }

    fn priority(&self) -> u32 {
        let common_item = self.common_item();
        letter_to_priority(common_item)
    }

    fn zip_compartments(&self) -> String {
        format!("{}{}", self.compartment1, self.compartment2)
    }
}

fn common_item(rucksack1: &RuckSack, rucksack2: &RuckSack, rucksack3: &RuckSack) -> char {
    let fullsack1 = rucksack1.zip_compartments();
    let fullsack2 = rucksack2.zip_compartments();
    let fullsack3 = rucksack3.zip_compartments();
    for item in fullsack1.chars() {
        if fullsack2.contains(item) && fullsack3.contains(item) {
            return item;
        }
    }
    unreachable!()
}

fn main() {
    let file_name = "input/day-03";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let rucksacks = reader
        .lines()
        .map(|line| RuckSack::from(line.unwrap()))
        .collect::<Vec<RuckSack>>();

    let sum_of_priorities: u32 = rucksacks.iter().map(|rucksack| rucksack.priority()).sum();
    println!("Sum of priorities: {}", sum_of_priorities);

    let sum_of_priorities_by_three: u32 = rucksacks
        .chunks_exact(3)
        .map(|chunk| {
            if chunk.len() != 3 {
                unreachable!()
            } else {
                letter_to_priority(common_item(&chunk[0], &chunk[1], &chunk[2]))
            }
        })
        .sum();
    println!("Sum of priorities by 3: {}", sum_of_priorities_by_three);
}

#[cfg(test)]
mod tests {
    use crate::{common_item, RuckSack};

    #[test]
    fn rucksack_from_string() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let rucksack = RuckSack::from(input);

        assert_eq!(rucksack.compartment1, "vJrwpWtwJgWr".to_string());
        assert_eq!(rucksack.compartment2, "hcsFMMfFFhFp".to_string());
    }

    #[test]
    fn rucksack_common_item() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let rucksack = RuckSack::from(input);

        assert_eq!(rucksack.common_item(), 'p');
    }

    #[test]
    fn rucksack_priority() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let rucksack = RuckSack::from(input);

        assert_eq!(rucksack.priority(), 16);
    }

    #[test]
    fn rucksack_zip_compartment() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let rucksack = RuckSack::from(input);

        assert_eq!(
            rucksack.zip_compartments(),
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string()
        );
    }

    #[test]
    fn find_common_item_in_rucksacks() {
        let rucksack1 = RuckSack::from("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        let rucksack2 = RuckSack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string());
        let rucksack3 = RuckSack::from("PmmdzqPrVvPwwTWBwg".to_string());

        assert_eq!(common_item(&rucksack1, &rucksack2, &rucksack3), 'r');
    }
}
