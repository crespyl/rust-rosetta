// Implements http://rosettacode.org/wiki/Flipping_bits_game

use std::io::{self, BufRead};
use std::rand::{self, Rng};
use std::collections::BitVec;

#[derive(PartialEq)]
struct Board {
    width: usize,
    height: usize,
    bits: BitVec,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            width: width,
            height: height,
            bits: BitVec::from_elem(width*height, false),
        }
    }
    
    fn get_bit(&self, col: usize, row: usize) -> bool {
        self.bits.get(row * self.width + col).unwrap_or(false)
    }

    fn set_bit(&mut self, col: usize, row: usize, bit: bool) {
        self.bits.set(row * self.width + col, bit)
    }

    fn flip_row(&mut self, row: usize) {
        for i in 0..self.width {
            let bit = self.bits.get(self.width*row + i).unwrap_or(false);
            self.bits.set(self.width*row + i, !bit);
        }
    }

    fn flip_col(&mut self, col: usize) {
        for i in 0..self.height {
            let bit = self.bits.get(self.width*i + col).unwrap_or(false);
            self.bits.set(self.width*i + col, !bit);
        }
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let (w, h) = (self.width, self.height);
        for _ in 0..w {
            self.flip_col(rng.gen() % w);
        }
        for _ in 0..h {
            self.flip_row(rng.gen() % h);
        }
    }

    fn print(&self) {
        // print column headers
        print!("  ");
        for col in 0..self.width {
            print!("{} ", (('a' as u8) + col as u8) as char );
        }
        println!("");
        for row in 0..self.height {
            print!("{} ", row+1);
            for col in 0..self.width {
                print!("{} ", if self.get_bit(col, row) { "1" } else { "0" });
            }
            println!("");
        }
    }
}

fn main() {
    let mut board = Board::new(3, 3);
    let mut target = Board::new(3, 3);

    while board == target {
        board.randomize();
        target.randomize();
    }

    let stdin = io::stdin();
    let mut line = String::new();
    let mut moves = 0;

    while !line.starts_with("q") {
        println!("Target:");
        target.print();
        println!("");

        println!("Board:");
        board.print();
        println!("");

        if target == board {
            println!("You won in {} moves!", moves);
            break;
        }

        moves += 1;
        println!("Move #{}", moves);
        println!("Flip row 1..3 or column 'a'..'c': ");
        line.clear();
        stdin.lock().read_line(&mut line);

        let c = line.char_at(0);
        if c.is_digit(10) {
            if let Ok(row) = line.trim().parse() {
                if row-1 < board.height {
                    board.flip_row(row-1);
                } else { println!("Invalid row!"); }
            }
        } else if c.is_alphabetic() {
            let col = ((c.to_lowercase().next().unwrap_or('a') as u8) - ('a' as u8)) as usize;
            if col < board.width {
                board.flip_col(col);
            } else { println!("Invalid column!"); }
        }
    }
}
