use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug, PartialEq)]
struct Path {
    points: Vec<(usize, usize)>,
}

impl From<String> for Path {
    fn from(input: String) -> Self {
        let split = input.split(" -> ").collect::<Vec<&str>>();
        let points = split.iter().map(|pair| {
            let pair_split = pair.split(",").collect::<Vec<&str>>();
            (pair_split[0].parse::<usize>().unwrap(), pair_split[1].parse::<usize>().unwrap())
        }).collect::<Vec<(usize, usize)>>();
        Path { points }
    }
}

impl Path {
    fn get_points(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for point_pair in self.points.windows(2) {
            let point_a = point_pair[0];
            let point_b = point_pair[1];
            if point_a.0 == point_b.0 {
                let min = point_a.1.min(point_b.1);
                let max = point_a.1.max(point_b.1);
                for y in min..=max {
                    res.push((point_a.0, y));
                }
            } else if point_a.1 == point_b.1 {
                let min = point_a.0.min(point_b.0);
                let max = point_a.0.max(point_b.0);
                for x in min..=max {
                    res.push((x, point_a.1));
                }
            } else {
                unreachable!()
            }
        }
        res
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

impl Cell {
    fn new() -> Self {
        Cell::Empty
    }
}

#[derive(Debug)]
struct Cave {
    offset: usize,
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl From<&Vec<Path>> for Cave {
    fn from(paths: &Vec<Path>) -> Self {
        let offset = paths.iter().map(|path| {
            path.points.iter().map(|point| point.0).min().unwrap()
        }).min().unwrap();
        
        let width = paths.iter().map(|path| {
            path.points.iter().map(|point| point.0).max().unwrap()
        }).max().unwrap() - offset + 1;
    
        let height = paths.iter().map(|path| {
            path.points.iter().map(|point| point.1).max().unwrap()
        }).max().unwrap() + 1;
        println!("width: {:?}, height: {:?}", width, height);

        let mut cells = vec![vec![Cell::new(); width]; height];
        for path in paths {
            for point in path.get_points() {
                cells[point.1][point.0 - offset] = Cell::Rock;
            }
        }
        Cave { offset, cells, width, height }
    }
}

struct MyError;

impl Cave {
    fn add_sand(&mut self) -> Result<(), MyError> {
        let mut cell = (0, 500 - self.offset as i32);
        loop {
            //println!("Cell {:?}", cell);
            if cell.0 + 1 == self.height {
                return Err(MyError);
            } else if self.cells[cell.0 + 1][cell.1 as usize] == Cell::Empty {
                cell = (cell.0 + 1, cell.1);
            } else {
                if cell.1 == 0 {
                    return Err(MyError);
                } else if self.cells[cell.0 + 1][(cell.1 - 1) as usize] == Cell::Empty {
                    cell = (cell.0 + 1, cell.1 - 1);
                } else if cell.1 as usize == self.width - 1 {
                    return Err(MyError);
                } else if self.cells[cell.0 + 1][(cell.1 + 1) as usize] == Cell::Empty {
                    cell = (cell.0 + 1, cell.1 + 1);
                } else {
                    self.cells[cell.0][cell.1 as usize] = Cell::Sand;
                    return Ok(());
                }
            }
        }
    }
}

#[derive(Debug)]
struct BigCave {
    offset: usize,
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl From<&Vec<Path>> for BigCave {
    fn from(paths: &Vec<Path>) -> Self {
        let path_offset = paths.iter().map(|path| {
            path.points.iter().map(|point| point.0).min().unwrap()
        }).min().unwrap();
        
        let path_width = paths.iter().map(|path| {
            path.points.iter().map(|point| point.0).max().unwrap()
        }).max().unwrap() - path_offset + 1;
    
        let height = paths.iter().map(|path| {
            path.points.iter().map(|point| point.1).max().unwrap()
        }).max().unwrap() + 1 + 2;
        let (width, offset) = if path_width > 2 * height + 1 {
            (path_width, path_offset)
        } else {
            (2 * height + 1, 500 - height)
        };
        println!("width: {:?}, height: {:?}", width, height);

        let mut cells = vec![vec![Cell::new(); width]; height];
        for path in paths {
            for point in path.get_points() {
                cells[point.1][point.0 - offset] = Cell::Rock;
            }
        }
        for i in 0..width {
            cells[height - 1][i] = Cell::Rock;
        }
        BigCave { offset, cells, width, height }
    }
}

impl BigCave {
    fn add_sand(&mut self) -> Result<(), MyError> {
        let mut cell = (0, 500 - self.offset as i32);
        loop {
            //println!("Cell {:?}", cell);
            if self.cells[cell.0][cell.1 as usize] == Cell::Sand {
                return Err(MyError);
            } else if cell.0 + 1 == self.height {
                return Err(MyError);
            } else if self.cells[cell.0 + 1][cell.1 as usize] == Cell::Empty {
                cell = (cell.0 + 1, cell.1);
            } else {
                if cell.1 == 0 {
                    return Err(MyError);
                } else if self.cells[cell.0 + 1][(cell.1 - 1) as usize] == Cell::Empty {
                    cell = (cell.0 + 1, cell.1 - 1);
                } else if cell.1 as usize == self.width - 1 {
                    return Err(MyError);
                } else if self.cells[cell.0 + 1][(cell.1 + 1) as usize] == Cell::Empty {
                    cell = (cell.0 + 1, cell.1 + 1);
                } else {
                    self.cells[cell.0][cell.1 as usize] = Cell::Sand;
                    return Ok(());
                }
            }
        }
    }
}

fn main() {
    let file_name = "input/day-14";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let paths = reader.lines().map(|line| Path::from(line.unwrap())).collect::<Vec<Path>>();
    // println!("{:?}", paths);
    // for path in &paths {
    //     println!("{:?}", path.get_points());
    // }
    
    let mut cave = Cave::from(&paths);
    //println!("{:?}", cave);

    let mut step = 0;
    while cave.add_sand().is_ok() {
        step += 1;
    }
    println!("{} units of sand", step);
    
    let mut big_cave = BigCave::from(&paths);
    
    let mut step = 0;
    while big_cave.add_sand().is_ok() {
        step += 1;
    }
    println!("{} units of sand for the big cave", step);
}
