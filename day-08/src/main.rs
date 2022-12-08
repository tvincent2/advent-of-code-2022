use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug,Clone)]
struct Tree {
    height: u8,
    max_height_l: u8,
    max_height_r: u8,
    max_height_t: u8,
    max_height_b: u8,
    on_border: bool,
}

impl Tree {
    fn new(height: u8) -> Self {
        Tree {
            height, max_height_b: 0, max_height_l: 0, max_height_r: 0, max_height_t: 0, on_border: false,
        }
    }

    fn is_visible(&self) -> bool {
        self.on_border ||
        self.height > self.max_height_b || self.height > self.max_height_l || self.height > self.max_height_r || self.height > self.max_height_t
    }
}

fn main() {
    let file_name = "input/day-08";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);
    
    let mut forest = reader.lines().map(|line| line.unwrap().chars().map(|c| Tree::new(c.to_digit(10).unwrap() as u8)).collect::<Vec<Tree>>()).collect::<Vec<Vec<Tree>>>();
    let size = forest.len();

    // from top left to bottom right
    for i in 0..size {
        for j in 0..size {
            if i > 0 {
                forest[i][j].max_height_t = forest[i-1][j].height.max(forest[i-1][j].max_height_t);
            } else {
                forest[i][j].on_border = true
            }
            if j > 0 {
                forest[i][j].max_height_l = forest[i][j-1].height.max(forest[i][j-1].max_height_l);
            } else {
                forest[i][j].on_border = true
            }
        }
    }
    // from bottom right to top left
    for i in (0..size).rev() {
        for j in (0..size).rev() {
            if i < size-1 {
                forest[i][j].max_height_b = forest[i+1][j].height.max(forest[i+1][j].max_height_b);
            } else {
                forest[i][j].on_border = true
            }
            if j < size-1 {
                forest[i][j].max_height_r = forest[i][j+1].height.max(forest[i][j+1].max_height_r);
            } else {
                forest[i][j].on_border = true
            }
        }
    }

    let number_of_visible_trees = forest.iter().map(|trees| trees.iter().filter(|tree| tree.is_visible()).count()).sum::<usize>();
    println!("visible trees: {}", number_of_visible_trees);

    let mut max_scenic_score = 0;
    for i in 1..(size-1) {
        for j in 1..(size-1) {
            let height = forest[i][j].height;
            let mut viewing_distance_top = 0;
            for i2 in (0..i).rev() {
                viewing_distance_top += 1;
                if forest[i2][j].height >= height {
                    break;
                }
            }
            let mut viewing_distance_bottom = 0;
            for i2 in (i+1)..size {
                viewing_distance_bottom += 1;
                if forest[i2][j].height >= height {
                    break;
                }
            }
            let mut viewing_distance_left = 0;
            for j2 in (0..j).rev() {
                viewing_distance_left += 1;
                if forest[i][j2].height >= height {
                    break;
                }
            }
            let mut viewing_distance_right = 0;
            for j2 in (j+1)..size {
                viewing_distance_right += 1;
                if forest[i][j2].height >= height {
                    break;
                }
            }
            let scenic_distance = viewing_distance_bottom * viewing_distance_left * viewing_distance_right * viewing_distance_top;
            max_scenic_score = max_scenic_score.max(scenic_distance);
        }
    }
    println!("Maximum viewing distance: {}", max_scenic_score);
}
