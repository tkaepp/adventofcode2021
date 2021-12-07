use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::slice::Chunks;
use std::vec;

use num::Num;

#[cfg(test)]
#[test]
fn bingo_test() {
    assert_eq!(bingo(day4_sample()), 4512);
}

struct Number {
    value: u8,
    marked: bool,
}

struct Board {
    lines: [[Number; 5]; 5],
}

struct BingoBoard2 {
    boards: Vec<Board>,
}

struct BingoBoard {
    boardlines: Vec<Vec<u8>>,
    move_order: Vec<u8>,
}

fn bingo(input: BingoBoard) -> i32 {
    let sum_of_unmarked = sum_of_unmarked();
    let last_number = 66;

    for value in input.move_order {
        let boards = input.boardlines.chunks(5);

        for board in boards {
            for line in board.chunks(5) {}
        }
    }

    let result = sum_of_unmarked * last_number;
    result
}

fn sum_of_unmarked() -> i32 {
    2
}

fn day4_sample() -> BingoBoard {
    let move_order = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
    let lines = read_lines("./day4/input1.txt");
    let (boardlines, _emptylines): (Vec<[u8; 5]>, Vec<[u8; 5]>) = lines
        .map(|line| -> [u8; 5] {
            let boardline = match line.unwrap().trim() {
                "" => vec![], // empty line for new board
                x => x
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect(),
            };
            boardline
                .try_into()
                .expect("there should be 5 numbers on one line")
        })
        .into_iter()
        .partition(|line| line.len() > 1);

    /*
    22 13 17 11  0 // vec<u8> || [u8; 5]
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

    */

    let b1: Vec<Board> = boardlines
    .chunks(5)
    .map(|board_chunk| -> Board {


        let numberIter = board_chunk
        .into_iter()
        .map(|row| {
            let array: [Number; 5] = row.map(|val| Number {
                value: val,
                marked: false,
            });
            array
        });
        let collect = numberIter        .collect();
        
        //.try_into().expect("");
 let x = lines.into_iter().collect::Vec<[Number; 5]>();


            Board { lines: todo!() }
    })
    .collect::<Vec<Board>>();

    let b2: Vec<Board> = boardlines
        .chunks(5)
        .map(|board_chunk| {
            let board: = board_chunk
                .into_iter()
                .map(|line| {
                    let array: [Number; 5] = line.map(|val| Number {
                        value: val,
                        marked: false,
                    });
                    array
                })
                .map(|lines| Board { lines: lines });

            board
        })
        .collect::<Vec<Board>>();

    let asdf: Vec<Board> = b2;

    BingoBoard {
        boardlines: boardlines,
        move_order: move_order
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect(),
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
}
