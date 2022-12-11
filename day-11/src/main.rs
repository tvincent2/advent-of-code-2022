use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisibility: u64,
    monkey_true: usize,
    monkey_false: usize,
    processed: usize,
}

impl Monkey {
    fn process_items_with_decreasing_worryness(&mut self) -> Vec<(u64, usize)> {
        self.processed += self.items.len();
        let result = self
            .items
            .par_iter()
            .map(|item| {
                let new_item: u64 = self.apply_operation(*item) / 3;
                if new_item % self.divisibility == 0 {
                    (new_item, self.monkey_true)
                } else {
                    (new_item, self.monkey_false)
                }
            })
            .collect::<Vec<(u64, usize)>>();
        self.items = vec![];
        result
    }

    fn process_items_without_decreasing_worryness(
        &mut self,
        stress_relief: u64,
    ) -> Vec<(u64, usize)> {
        self.processed += self.items.len();
        let result = self
            .items
            .par_iter()
            .map(|item| {
                let new_item: u64 = self.apply_operation(*item) % stress_relief;
                if new_item % self.divisibility == 0 {
                    (new_item, self.monkey_true)
                } else {
                    (new_item, self.monkey_false)
                }
            })
            .collect::<Vec<(u64, usize)>>();
        self.items = vec![];
        result
    }

    fn apply_operation(&self, number: u64) -> u64 {
        match self.operation {
            Operation::Add(value) => number + value,
            Operation::Multiply(value) => number * value,
            Operation::Square => number * number,
        }
    }

    fn pass(&mut self, number: u64) {
        self.items.push(number);
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl From<String> for Operation {
    fn from(input: String) -> Self {
        let split = input.split(" ").collect::<Vec<&str>>();
        if split.len() == 8 {
            match (split[6], split[7]) {
                ("+", value) => Operation::Add(value.parse::<u64>().unwrap()),
                ("*", "old") => Operation::Square,
                ("*", value) => Operation::Multiply(value.parse::<u64>().unwrap()),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}

struct MonkeyBuilder {
    items: Vec<u64>,
    operation: Operation,
    divisibility: u64,
    monkey_true: usize,
    monkey_false: usize,
}

impl MonkeyBuilder {
    fn new() -> Self {
        MonkeyBuilder {
            items: vec![],
            operation: Operation::Add(0),
            divisibility: 1,
            monkey_true: 0,
            monkey_false: 0,
        }
    }

    fn add_item(&mut self, item: u64) -> &mut Self {
        self.items.push(item);
        self
    }

    fn set_operation(&mut self, operation: Operation) -> &mut Self {
        self.operation = operation;
        self
    }

    fn set_divisibility(&mut self, divisibility: u64) -> &mut Self {
        self.divisibility = divisibility;
        self
    }

    fn set_monkey_true(&mut self, monkey: usize) -> &mut Self {
        self.monkey_true = monkey;
        self
    }

    fn set_monkey_false(&mut self, monkey: usize) -> &mut Self {
        self.monkey_false = monkey;
        self
    }

    fn build(self) -> Monkey {
        Monkey {
            processed: 0,
            items: self.items,
            operation: self.operation,
            divisibility: self.divisibility,
            monkey_true: self.monkey_true,
            monkey_false: self.monkey_false,
        }
    }
}

fn main() {
    let file_name = "input/day-11";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let mut monkeys: Vec<Monkey> = vec![];
    let mut builder = MonkeyBuilder::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("Monkey") {
            // ignore this line
        } else if line.starts_with("  Starting items") {
            line[18..].split(",").for_each(|item| {
                builder.add_item(item.trim().parse::<u64>().unwrap());
            });
        } else if line.starts_with("  Operation") {
            builder.set_operation(Operation::from(line));
        } else if line.starts_with("  Test") {
            builder.set_divisibility(line[21..].parse::<u64>().unwrap());
        } else if line.starts_with("    If true") {
            builder.set_monkey_true(line[29..].parse::<usize>().unwrap());
        } else if line.starts_with("    If false") {
            builder.set_monkey_false(line[30..].parse::<usize>().unwrap());
        } else {
            monkeys.push(builder.build());
            builder = MonkeyBuilder::new();
        }
    }
    monkeys.push(builder.build());
    let mut monkeys_step2 = monkeys.clone();

    for _step in 0..20 {
        for m in 0..monkeys.len() {
            let mut current_monkey = monkeys[m].clone();
            let new_items = current_monkey.process_items_with_decreasing_worryness();
            for (item, monkey) in new_items {
                monkeys[monkey].pass(item);
            }
            monkeys[m] = current_monkey;
        }
    }

    let mut processed_numbers = monkeys
        .iter()
        .map(|monkey| monkey.processed)
        .collect::<Vec<usize>>();
    processed_numbers.sort();
    processed_numbers.reverse();

    println!("{:?}", processed_numbers[0] * processed_numbers[1]);

    let stress_relief = monkeys_step2
        .iter()
        .map(|monkey| monkey.divisibility.clone())
        .reduce(|accum, item| accum * item)
        .unwrap();

    for _step in 0..10_000 {
        for m in 0..monkeys_step2.len() {
            let mut current_monkey = monkeys_step2[m].clone();
            let new_items =
                current_monkey.process_items_without_decreasing_worryness(stress_relief);
            for (item, monkey) in new_items {
                monkeys_step2[monkey].pass(item);
            }
            monkeys_step2[m] = current_monkey;
        }
    }

    let mut processed_numbers = monkeys_step2
        .iter()
        .map(|monkey| monkey.processed)
        .collect::<Vec<usize>>();
    processed_numbers.sort();
    processed_numbers.reverse();

    println!("{:?}", processed_numbers[0] * processed_numbers[1]);
}

#[cfg(test)]
mod tests {
    use crate::Operation;

    #[test]
    fn parse_operation_multiply() {
        let input = "  Operation: new = old * 19".to_string();
        let operation = Operation::from(input);

        assert_eq!(operation, Operation::Multiply(19));
    }

    #[test]
    fn parse_operation_square() {
        let input = "  Operation: new = old * old".to_string();
        let operation = Operation::from(input);

        assert_eq!(operation, Operation::Square);
    }

    #[test]
    fn parse_operation_add() {
        let input = "  Operation: new = old + 25".to_string();
        let operation = Operation::from(input);

        assert_eq!(operation, Operation::Add(25));
    }
}
