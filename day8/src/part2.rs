use aoclib::reader::lines;
use grid::{grid, Grid};

fn up_score(g: &Grid<u32>, row: usize, col: usize) -> u32 {
    let my_height = g[row][col];

    let mut visible_trees = 0;
    for r in (0..row).rev() {
        if g[r][col] < my_height {
            visible_trees += 1;
        } else if g[r][col] >= my_height {
            visible_trees += 1;
            break
        }
    }

    // println!("U {} {} = {}", row, col, visible_trees);
    visible_trees
}

fn down_score(g: &Grid<u32>, row: usize, col: usize) -> u32 {
    let my_height = g[row][col];

    let mut visible_trees = 0;
    for r in row+1..g.rows() {
        if g[r][col] < my_height {
            visible_trees += 1;
        } else if g[r][col] >= my_height {
            visible_trees += 1;
            break
        }
    }

    // println!("D {} {} = {}", row, col, visible_trees);
    visible_trees
}

fn left_score(g: &Grid<u32>, row: usize, col: usize) -> u32 {
    let my_height = g[row][col];

    let mut visible_trees = 0;
    for c in (0..col).rev() {
        if g[row][c] < my_height {
            visible_trees += 1;
        } else if g[row][c] >= my_height {
            visible_trees += 1;
            break
        }
    }

    // println!("L {} {} = {}", row, col, visible_trees);
    visible_trees
}

fn right_score(g: &Grid<u32>, row: usize, col: usize) -> u32 {
    let my_height = g[row][col];

    let mut visible_trees = 0;
    for c in col+1..g.cols() {
        if g[row][c] < my_height {
            visible_trees += 1;
        } else if g[row][c] >= my_height {
            visible_trees += 1;
            break
        }
    }

    // println!("R {} {} = {}", row, col, visible_trees);
    visible_trees
}

fn compute_score(g: &Grid<u32>, row: usize, col: usize) -> u32 {
    let score = up_score(g, row, col) * down_score(g, row, col) * left_score(g, row, col) * right_score(g, row, col);

    // println!("S {} {} = {}", row, col, score);

    score
}

pub fn part2(input: &str) -> u32 {
    let mut g: Grid<u32> = grid![];
    for row in lines(input) {
        g.push_row(row.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut highest_score = 0;
    for row in 0..g.rows() {
        for col in 0..g.cols() {
            let score = compute_score(&g, row, col);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}