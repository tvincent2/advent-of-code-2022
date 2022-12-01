use std::{fs::{self, File}, io::{BufReader, BufRead}};

#[derive(Debug)]
struct Elf {
    pub energies: Vec<u32>,
}

fn main() {
    let file_name = "input/day-1";
    // let data = fs::read_to_string(file_name).unwrap();
    // println!("{}", data);
    // let elves: Vec<&str> = data.split("\n\n").collect();
    // println!("{}", elves.len());
    // let elves2 = elves.iter().map(|&elf_string| elf_string.split("\n").collect::Vec<&str>()).collect();
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let mut elves = vec![];
    let mut energies = vec![];

    for line in reader.lines() {
        if let Ok(number) = line.unwrap().parse::<u32>() {
            energies.push(number);
        } else {
            if !energies.is_empty() {
                let elf = Elf{energies};
                elves.push(elf);
                energies = vec![];
            }
        }
    }
    if !energies.is_empty() {
        let elf = Elf{energies};
        elves.push(elf);
    }
    println!("{:?}", elves);

    let max = elves.iter().map(|elf| elf.energies.iter().sum::<u32>()).max();
    println!("{:?}", max);
}
