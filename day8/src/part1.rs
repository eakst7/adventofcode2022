use aoclib::reader::lines;
use grid::{grid, Grid};

#[allow(dead_code)]
fn print_grid(g: &Grid<u32>, visible: &Grid<bool>) {
    for r in 0..g.rows() {
        for c in 0..g.cols() {
            print!("{}{}", g[r][c], if visible[r][c] { "!" } else { " " } )
        }
        println!("");
    }
}

pub fn part1(input: &str) -> u32 {
    let mut g: Grid<u32> = grid![];
    for row in lines(input) {
        g.push_row(row.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut visible: Grid<bool> = Grid::new(g.rows(), g.cols());

    // Right to left
    for row in 0..g.rows() {
        let mut row_highest: u32 = g[row][0];
        for col in 0..g.cols() {
            if (row == 0) || (row == g.rows()-1) {
                visible[row][col] = true;
            }

            if (col == 0) || (col == g.cols()-1) {
                visible[row][col] = true;
            }

            if g[row][col] > row_highest {
                visible[row][col] = true;
                row_highest = g[row][col];
            }
        }
    }

    // Left to right
    for row in 0..g.rows() {
        let mut row_highest = g[row][g.cols()-1];
        for col in (0..g.cols()).rev() {
            if (row == 0) || (row == g.rows()-1) {
                visible[row][col] = true;
            }

            if (col == 0) || (col == g.cols()-1) {
                visible[row][col] = true;
            }

            if g[row][col] > row_highest {
                //println!("Checking {} {} {} visible", row, col, g[row][col]);
                visible[row][col] = true;
                row_highest = g[row][col];
            } else {
                //println!("Checking {} {} {}", row, col, g[row][col]);
            }
        }
    }

    // Top to bottom
    for col in 0..g.cols() {
        let mut col_highest = g[0][col];
        for row in 0..g.rows() {
            if (row == 0) || (row == g.rows()-1) {
                visible[row][col] = true;
            }

            if (col == 0) || (col == g.cols()-1) {
                visible[row][col] = true;
            }

            if g[row][col] > col_highest {
                //println!("Checking {} {} {} visible", row, col, g[row][col]);
                visible[row][col] = true;
                col_highest = g[row][col];
            } else {
                //println!("Checking {} {} {}", row, col, g[row][col]);

            }
        }
    }

    // Bottom to top
    for col in 0..g.cols() {
        let mut col_highest = g[g.rows()-1][col];
        for row in (0..g.rows()).rev() {
            if (row == 0) || (row == g.rows()-1) {
                visible[row][col] = true;
            }

            if (col == 0) || (col == g.cols()-1) {
                visible[row][col] = true;
            }

            if g[row][col] > col_highest {
                visible[row][col] = true;
                col_highest = g[row][col];
            }
        }
    }

    //print_grid(&g, &visible);

    visible.iter().filter(|b| **b).count() as u32
}