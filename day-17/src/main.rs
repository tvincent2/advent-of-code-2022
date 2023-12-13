use std::{collections::VecDeque, fs};

#[derive(Debug, PartialEq)]
enum Move {
    Left,
    Right,
}

impl From<char> for Move {
    fn from(input: char) -> Self {
        match input {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Full,
}

#[derive(Debug, PartialEq)]
struct Rock {
    cells: Vec<(usize, usize)>,
}

impl Rock {
    fn right(&self) -> Vec<(usize, usize)> {
        let right_cells = self.cells.iter().map(|(y,x)| (*y, *x + 1)).filter(|point| !self.cells.contains(point)).collect::<Vec<(usize, usize)>>();
        right_cells
    }

    fn left(&self) -> Vec<(usize, usize)> {
        let left_cells = self.cells.iter().map(|(y,x)| (*y, *x - 1)).filter(|point| !self.cells.contains(point)).collect::<Vec<(usize, usize)>>();
        left_cells
    }

    fn bottom(&self) -> Vec<(usize, usize)> {
        let bottom_cells = self.cells.iter().map(|(y,x)| (*y - 1, *x)).filter(|point| !self.cells.contains(point)).collect::<Vec<(usize, usize)>>();
        bottom_cells
    }

    fn go_left(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = (cell.0, cell.1 - 1);
        }
    }

    fn go_down(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = (cell.0 - 1, cell.1);
        }
    }

    fn go_right(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = (cell.0, cell.1 + 1);
        }
    }
}

struct RockLauncher {
    current: usize,
}

impl RockLauncher {
    fn new() -> Self {
        RockLauncher { current: 0 }
    }

    fn launch(&mut self, height: usize) -> Rock {
        let starting_height = height + 4;
        let rock = match self.current % 5 {
            0 => Rock { cells: vec![(starting_height, 3), (starting_height, 4), (starting_height, 5), (starting_height, 6)] },
            1 => Rock { cells: vec![(starting_height, 4), (starting_height + 1, 3), (starting_height + 1, 4), (starting_height + 1, 5), (starting_height + 2, 4)] },
            2 => Rock { cells: vec![(starting_height, 3), (starting_height, 4), (starting_height, 5), (starting_height + 1, 5), (starting_height + 2, 5)] },
            3 => Rock { cells: vec![(starting_height, 3), (starting_height + 1, 3), (starting_height + 2, 3), (starting_height + 3, 3)] },
            4 => Rock { cells: vec![(starting_height, 3), (starting_height, 4), (starting_height + 1, 3), (starting_height + 1, 4)] },
            _ => unreachable!(),
        };
        self.current += 1;
        rock
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    cells: VecDeque<[Cell; 9]>,
    lowest: [usize; 9],
    offset: usize,
}

impl Grid {
    fn new() -> Self {
        Grid { cells: VecDeque::from([[Cell::Full; 9]]), lowest: [usize::MAX, 0, 0, 0, 0, 0, 0, 0, usize::MAX], offset: 0 }
    }
    
    fn add_line(&mut self) {
        self.cells.push_back([Cell::Full, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Full]);
    }

    fn add_three_lines(&mut self) {
        self.add_line();
        self.add_line();
        self.add_line();
    }

    fn add_rock(&mut self, rock: &Rock) {
        for point in &rock.cells {
            self.cells[point.0 - self.offset][point.1] = Cell::Full;
            if point.0 > self.lowest[point.1] {
                self.lowest[point.1] = point.0;
            }
        }
        // let min = self.lowest[..].iter().min().unwrap();
        // if *min > self.offset {
        //     let difference = *min - self.offset - 1;
        //     for _i in 0..difference {
        //         self.cells.pop_front();
        //     }
        //     self.offset = *min;
        // }
    }

    fn intersect(&self, cells: Vec<(usize, usize)>) -> bool {
        // println!("Testing {:?}", cells);
        // println!("Grid height {}, offset {}", self.cells.len(), self.offset);
        cells
        .iter()
        .filter(|(y, _)| y - self.offset < self.cells.len())
        .any(|(y, x)| self.cells[y - self.offset][*x] == Cell::Full)
    }

    fn max_height(&self) -> usize {
        self.lowest[1..8].iter().max().unwrap().clone()
    }
}

fn main() {
    let file_name = "input/day-17-test";
    let content = fs::read_to_string(file_name).expect("oops");

    let moves = content.chars().map(|char| Move::from(char)).collect::<Vec<Move>>();
    // println!("Moves: {:?}", moves);
    let mut move_iter = moves.iter().cycle();

    let mut grid = Grid::new();
    let mut rock_launcher = RockLauncher::new();

    run_tetris(rock_launcher, &mut grid, move_iter, moves.len(), 10000000);
    // println!("{:?}", grid);
    println!("height: {}", grid.max_height());
    println!("moves: {}", moves.len());
}

fn run_tetris(mut rock_launcher: RockLauncher, grid: &mut Grid, mut move_iter: std::iter::Cycle<std::slice::Iter<Move>>, nb_moves: usize, iterations: usize) {
    let mut heights = vec![];
    let mut counter = 0;
    for _ in 0..iterations {
        let mut rock = rock_launcher.launch(grid.max_height());
        // println!("rock: {:?}", rock);
        grid.add_three_lines();

        let mut can_go_down = true;
        while can_go_down {
            match move_iter.next().unwrap() {
                Move::Left => {
                    if !grid.intersect(rock.left()) {
                        rock.go_left();
                    }
                },
                Move::Right => {
                    if !(grid.intersect(rock.right())) {
                        rock.go_right();
                    }
                }
            }
            counter += 1;
            can_go_down = !grid.intersect(rock.bottom());
            if can_go_down {
                rock.go_down();
            } else {
                grid.add_rock(&rock);
            }
        }
        // println!("{}", counter % nb_moves);
        if counter % nb_moves == 0 {
            let diff = grid.max_height() - heights.last().unwrap_or(&0);
            heights.push(diff);
        }
    }
    println!("{}", counter);
    println!("{:?}", heights);
}
