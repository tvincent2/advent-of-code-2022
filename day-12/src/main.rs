use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Debug)]
struct Cell {
    is_start: bool,
    is_end: bool,
    altitude: u8,
    visited: bool,
    previous: Option<(usize, usize)>,
}

const fn letter_to_altitude(letter: char) -> u8 {
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
        _ => unreachable!(),
    }
}

impl From<char> for Cell {
    fn from(input: char) -> Self {
        match input {
            'S' => Cell {
                is_start: true,
                is_end: false,
                altitude: 1,
                visited: false,
                previous: None
            },
            'E' => Cell {
                is_start: false,
                is_end: true,
                altitude: 26,
                visited: false,
                previous: None
            },
            c => Cell {
                is_start: false,
                is_end: false,
                altitude: letter_to_altitude(c),
                visited: false,
                previous: None,
            },
        }
    }
}

fn get_neighbors(
    (line, column): (usize, usize),
    number_of_lines: usize,
    number_of_columns: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if line > 0 {
        neighbors.push((line - 1, column));
    }
    if column > 0 {
        neighbors.push((line, column - 1));
    }
    if line < number_of_lines - 1 {
        neighbors.push((line + 1, column));
    }
    if column < number_of_columns - 1 {
        neighbors.push((line, column + 1));
    }
    neighbors
}

fn main() {
    let file_name = "input/day-12";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);

    let mut grid = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| Cell::from(c))
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    // We start from the end and go back to the start so we can use the same algorithm on both problems
    let mut end_cell = (0, 0);
    for (line_index, line) in grid.iter().enumerate() {
        for (column_index, cell) in line.iter().enumerate() {
            if cell.is_end {
                end_cell = (line_index, column_index);
                break;
            }
        }
    }
    let lines = grid.len();
    let columns = grid[0].len();

    let mut cells_to_check = vec![end_cell];
    let mut path_length_to_start = 0;
    let mut path_length_to_low_cell = 0;
    'outer: for step in 0.. {
        let mut next_cells_to_check = vec![];
        for cell in &cells_to_check {
            if grid[cell.0][cell.1].is_start {
                path_length_to_start = step;
                break 'outer;
            }
            if grid[cell.0][cell.1].altitude == 1 && path_length_to_low_cell == 0 {
                path_length_to_low_cell = step;
            }
            grid[cell.0][cell.1].visited = true;
            let mut neighbors_to_visit = get_neighbors(*cell, lines, columns)
                .iter()
                .filter(|(line, column)| !grid[*line][*column].visited)
                .filter(|(line, column)| {
                    let current_altitude = grid[cell.0][cell.1].altitude;
                    let neighbor_altitude = grid[*line][*column].altitude;
                    neighbor_altitude + 1 >= current_altitude
                })
                .map(|cell| *cell)
                .collect::<Vec<(usize, usize)>>();
            for neighbor in &neighbors_to_visit {
                grid[neighbor.0][neighbor.1].previous = Some((cell.0, cell.1));
            }
            next_cells_to_check.append(&mut neighbors_to_visit);
        }
        if next_cells_to_check.is_empty() {
            panic!("Should not be empty");
        }
        next_cells_to_check.sort();
        next_cells_to_check.dedup();
        cells_to_check = next_cells_to_check;
    };

    println!("{} steps to reach start", path_length_to_start);
    println!("{} steps to reach low cell", path_length_to_low_cell);
}
