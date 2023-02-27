use rand::prelude::*;
use std::time::Duration;
use std::thread::sleep;

const BOARD_SIZE: usize = 50;

struct Game {
    board: [[bool; BOARD_SIZE]; BOARD_SIZE],
}
impl Game {
    fn new() -> Self {
        Self {
            board: [[false; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                if *cell {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
    fn generate(&mut self) {
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = random(); // random bool
            }
        }
    }
    fn num_neighbors(&self, row:usize, col:usize) -> usize {

        let mut neighbors = 0;

        let above = (col as i32) - 1;
        let below = (col as i32) + 1;
        let left = (row as i32) - 1;
        let right = (row as i32) + 1;
        if above >= 0 {
            if self.board[row][col-1] {
                neighbors += 1;
            }
        }
        if below < BOARD_SIZE as i32 {
            if self.board[row][col+1] {
                neighbors += 1;
            }
        }
        if left >= 0 {
            if self.board[row-1][col] {
                neighbors += 1;
            }
        }
        if right < BOARD_SIZE as i32 {
            if self.board[row+1][col] {
                neighbors += 1;
            }
        }
        neighbors
    }

    fn iterate(&mut self) {
        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Any live cell with two or three live neighbours lives on to the next generation.
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        for (i, row) in self.board.clone().iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let neighbors = self.num_neighbors(i, j);
                if *cell {
                    // live cell
                    if neighbors < 2 {
                        // underpopulation
                        self.board[i][j] = false;
                    }
                    else if neighbors > 3 {
                        // overpopulation
                        self.board[i][j] = false;
                    }
                    // else, lives on
                } else {
                    // dead cell
                    if neighbors == 3 {
                        // reproduction
                        self.board[i][j] = true;
                    }
                }
            }
        }
    }
}

fn clear() {
    print!("\x1B[2J");
}

fn main() {
    let mut game = Game::new();

    game.generate();
    for _ in 0..100 { // run 100 iterations (years)
        game.iterate();
        game.print_board();

        sleep(Duration::from_millis(100)); // sleep 1 second
        clear();
    }
    game.print_board();
    println!("Done!")
}
