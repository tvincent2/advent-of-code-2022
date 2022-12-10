use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Instruction {
    NoOp,
    AddX(i32),
}

impl From<String> for Instruction {
    fn from(input: String) -> Self {
        if input == "noop" {
            Instruction::NoOp
        } else if input.starts_with("addx") {
            let x = input.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            Instruction::AddX(x)
        } else {
            unreachable!()
        }
    }
}

fn main() {
    let file_name = "input/day-10";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let instructions = reader
        .lines()
        .map(|line| Instruction::from(line.unwrap()))
        .collect::<Vec<Instruction>>();

    let mut x = 1;
    let mut x_history = vec![x];

    for instruction in instructions {
        match instruction {
            Instruction::NoOp => x_history.push(x),
            Instruction::AddX(value) => {
                x_history.push(x);
                x += value;
                x_history.push(x);
            }
        }
    }

    let mut sum = 0;
    for cycle in [20, 60, 100, 140, 180, 220] {
        sum += cycle as i32 * x_history[cycle - 1];
    }

    println!("Sum: {}", sum);

    for line_index in 0..6 {
        let mut screen_line = String::new();
        for crt_index in 0..40 {
            let history_index = line_index * 40 + crt_index;
            let sprite_index = x_history[history_index] - 1;
            if (sprite_index..sprite_index + 3).contains(&(crt_index as i32)) {
                screen_line.push('#');
            } else {
                screen_line.push('.');
            }
        }
        println!("{}", screen_line);
    }
}
