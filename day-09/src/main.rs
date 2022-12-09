use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        match input {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    steps: i32,
}

impl From<String> for Move {
    fn from(input: String) -> Self {
        let split = input.split(" ").collect::<Vec<&str>>();
        if split.len() == 2 {
            Move {
                direction: Direction::from(split[0]),
                steps: split[1].parse::<i32>().unwrap(),
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new() -> Self {
        Knot { x: 0, y: 0 }
    }

    fn follow_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    fn follow(&mut self, other: &Knot) {
        if (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 {
            // no need to move
        } else {
            // need to move
            if self.x == other.x {
                // vertical move
                if self.y - other.y > 0 {
                    self.follow_direction(&Direction::Down);
                } else {
                    self.follow_direction(&Direction::Up);
                }
            } else if self.y == other.y {
                // horizontal move
                if self.x - other.x > 0 {
                    self.follow_direction(&Direction::Left);
                } else {
                    self.follow_direction(&Direction::Right);
                }
            } else {
                // diagonal move
                if self.x - other.x > 0 {
                    self.follow_direction(&Direction::Left);
                } else if self.x - other.x < 0 {
                    self.follow_direction(&Direction::Right);
                }
                if self.y - other.y > 0 {
                    self.follow_direction(&Direction::Down);
                } else if self.y - other.y < 0 {
                    self.follow_direction(&Direction::Up);
                }
            }
        }
    }
}

#[derive(Debug)]
struct HeadTail<const COUNT: usize> {
    head: Knot,
    tail: [Knot; COUNT],
    trail: HashSet<Knot>,
}

impl<const COUNT: usize> HeadTail<COUNT> {
    fn new() -> Self {
        let tail = [Knot::new(); COUNT];
        let mut trail = HashSet::new();
        trail.insert(Knot::new());
        HeadTail { head: Knot::new(), tail, trail }
    }

    fn follow_direction(&mut self, direction: &Direction) {
        self.head.follow_direction(direction);
        self.tail[0].follow(&self.head);
        for index in 1..COUNT {
            let previous_knot = self.tail[index-1].clone();
            self.tail[index].follow(&previous_knot);
        }
        self.trail.insert(self.tail[COUNT-1]);
    }

    fn follow_move(&mut self, move_instruction: &Move) {
        //println!("Following {:?}", &move_instruction);
        for _step in 0..move_instruction.steps {
            self.follow_direction(&move_instruction.direction);
            //println!("Step: {:?}", self);
        }
    }
}

fn main() {
    let file_name = "input/day-09";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let moves = reader
        .lines()
        .map(|line| Move::from(line.unwrap()))
        .collect::<Vec<Move>>();
    //println!("{:?}", moves);

    let mut head_tail: HeadTail<1> = HeadTail::new();
    for mov in &moves {
        head_tail.follow_move(mov);
    }
    println!("{}", head_tail.trail.len());

    let mut head_tail: HeadTail<9> = HeadTail::new();
    for mov in &moves {
        head_tail.follow_move(mov);
    }
    println!("{}", head_tail.trail.len());
}
