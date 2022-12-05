use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug, PartialEq)]
struct Stacks {
    stacks: Vec<String>,
}

impl Stacks {
    fn move_items_with_crane_9000(&mut self, move_order: &MoveOrder) {
        for _step in 0..move_order.number {
            self.move_one_item(move_order.from, move_order.to);
        }
    }

    fn move_one_item(&mut self, from: usize, to: usize) {
        let item = self.stacks[from].pop().unwrap();
        self.stacks[to].push(item);
    }

    fn move_items_with_crane_9001(&mut self, move_order: &MoveOrder) {
        let mut moved = String::with_capacity(move_order.number);
        for _step in 0..move_order.number {
            moved.push(self.stacks[move_order.from].pop().unwrap());
        }
        for _step in 0..move_order.number {
            self.stacks[move_order.to].push(moved.pop().unwrap());
        }
    }

    fn top_of_stacks(&self) -> String {
        self.stacks.iter().map(|stack| stack.chars().last().unwrap()).collect::<String>()
    }
}

impl From<&[String]> for Stacks {
    fn from(input: &[String]) -> Self {
        let width = input[0].len();
        let number_of_stacks = (width + 1) / 4;
        let mut stacks = vec!["".to_string(); number_of_stacks];
        for line in input.iter().rev() {
            let vec_line = line.chars().collect::<Vec<char>>();
            let chunks = vec_line.chunks(4);
            for (index, chunk) in chunks.enumerate() {
                if chunk[0] == '[' {
                    stacks[index].push(chunk[1]);
                }
            }
        }
        Stacks { stacks }
    }
}

#[derive(Debug, PartialEq)]
struct MoveOrder {
    number: usize,
    from: usize,
    to: usize,
}

impl From<String> for MoveOrder {
    fn from(input: String) -> Self {
        MoveOrder::from(&input)
    }
}

impl From<&String> for MoveOrder {
    fn from(input: &String) -> Self {
        let split_input = input.split(" ").collect::<Vec<&str>>();
        MoveOrder { number: split_input[1].parse::<usize>().unwrap(), from: split_input[3].parse::<usize>().unwrap() -1, to: split_input[5].parse::<usize>().unwrap() -1 }
    }
}

fn get_split_index(lines: &Vec<String>) -> usize {
    for (index, line) in lines.iter().enumerate() {
        if line.starts_with(" 1") {
            return index;
        }
    }
    unreachable!()
}

fn main() {
    let file_name = "input/day-05";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);
    
    let lines = reader.lines().map(|line_result| line_result.unwrap()).collect::<Vec<String>>();
    let index = get_split_index(&lines);

    let (stacks_input, move_orders_input) = lines.split_at(index);
    let mut stacks = Stacks::from(stacks_input);
    // we ignore the first line as it contains ' 1  2  3 …'
    // and the second one as it's empty
    let move_orders: Vec<MoveOrder> = move_orders_input[2..].iter().map(|input| MoveOrder::from(input)).collect();

    for move_order in move_orders.iter() {
        stacks.move_items_with_crane_9000(move_order);
    }
    println!("Top of stacks after moves with crane 9000: {}", stacks.top_of_stacks());

    let mut stacks = Stacks::from(stacks_input);
    for move_order in move_orders.iter() {
        stacks.move_items_with_crane_9001(move_order);
    }
    println!("Top of stacks after moves with crane 9001: {}", stacks.top_of_stacks());
}

//     [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 

#[cfg(test)]
mod tests {
    use crate::{Stacks, MoveOrder};

    #[test]
    fn read_drawing() {
        let drawing = vec!["    [D]    ".to_string(), "[N] [C]    ".to_string(), "[Z] [M] [P]".to_string()];
        let stacks = Stacks::from(drawing.as_slice());

        assert_eq!(stacks, Stacks { stacks: vec!["ZN".to_string(), "MCD".to_string(), "P".to_string()] });
    }

    #[test]
    fn read_move_order() {
        let input = "move 1 from 2 to 3".to_string();
        let move_order = MoveOrder::from(input);

        // from and to indices start at 0, so we expect 1 and 2 instead of 2 and 3
        assert_eq!(move_order, MoveOrder { number: 1, from: 1, to: 2 });
    }

    #[test]
    fn apply_move_order_with_crane_9000() {
        let drawing = vec!["    [D]    ".to_string(), "[N] [C]    ".to_string(), "[Z] [M] [P]".to_string()];
        let mut stacks = Stacks::from(drawing.as_slice());

        let input = "move 1 from 2 to 3".to_string();
        let move_order = MoveOrder::from(input);

        stacks.move_items_with_crane_9000(&move_order);
        assert_eq!(stacks, Stacks { stacks: vec!["ZN".to_string(), "MC".to_string(), "PD".to_string()] });
    }

    #[test]
    fn apply_move_order_with_crane_9000_several_items() {
        let drawing = vec!["    [D]    ".to_string(), "[N] [C]    ".to_string(), "[Z] [M] [P]".to_string()];
        let mut stacks = Stacks::from(drawing.as_slice());

        let input = "move 2 from 2 to 3".to_string();
        let move_order = MoveOrder::from(input);

        stacks.move_items_with_crane_9000(&move_order);
        assert_eq!(stacks, Stacks { stacks: vec!["ZN".to_string(), "M".to_string(), "PDC".to_string()] });
    }

    #[test]
    fn apply_move_order_with_crane_9001() {
        let drawing = vec!["    [D]    ".to_string(), "[N] [C]    ".to_string(), "[Z] [M] [P]".to_string()];
        let mut stacks = Stacks::from(drawing.as_slice());

        let input = "move 1 from 2 to 3".to_string();
        let move_order = MoveOrder::from(input);

        stacks.move_items_with_crane_9001(&move_order);
        assert_eq!(stacks, Stacks { stacks: vec!["ZN".to_string(), "MC".to_string(), "PD".to_string()] });
    }

    #[test]
    fn apply_move_order_with_crane_9001_several_items() {
        let drawing = vec!["    [D]    ".to_string(), "[N] [C]    ".to_string(), "[Z] [M] [P]".to_string()];
        let mut stacks = Stacks::from(drawing.as_slice());

        let input = "move 2 from 2 to 3".to_string();
        let move_order = MoveOrder::from(input);

        stacks.move_items_with_crane_9001(&move_order);
        assert_eq!(stacks, Stacks { stacks: vec!["ZN".to_string(), "M".to_string(), "PCD".to_string()] });
    }

    #[test]
    fn top_of_stacks() {
        let drawing = vec!["    [D]    ".to_string(), "[N] [C]    ".to_string(), "[Z] [M] [P]".to_string()];
        let stacks = Stacks::from(drawing.as_slice());

        assert_eq!(stacks.top_of_stacks(), "NDP".to_string());
    }
}