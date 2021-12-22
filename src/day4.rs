use core::ops::Range;
use indexmap::IndexMap;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;

#[cfg(test)]
#[test]
fn bingo_test() {
    let (_, last_number, _, score) = bingo(sample());

    assert_eq!(last_number, 24);
    assert_eq!(score, 4512);
}

pub fn real() -> io::Lines<io::BufReader<File>> {
    read_lines("./day4/input.txt")
}
pub fn sample() -> io::Lines<io::BufReader<File>> {
    read_lines("./day4/input_sample.txt")
}

pub fn bingo(file_input: io::Lines<io::BufReader<File>>) -> (usize, i32, usize, i32) {
    let mut lines_iter = file_input.map(|l| l.unwrap());

    let move_order: Vec<i32> = lines_iter
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut results: HashSet<_, _> = HashSet::new();
    let mut board_id = 1;

    let (values, _): (Vec<String>, Vec<String>) = lines_iter.partition(|p| p != ""); //.collect::<Vec<String>>().chunks(5);

    for chunk in values.chunks(5) {
        let mut last_index = 0;

        let mut board_numbers: IndexMap<i32, bool> = chunk
            .into_iter()
            .flat_map(|l| {
                l.trim()
                    .split_ascii_whitespace()
                    .map(|s| (s.parse::<i32>().unwrap(), false))
            })
            .into_iter()
            .collect();

        for (i, m) in move_order.iter().enumerate() {
            last_index = i;

            // the number exists on the board.
            if let Some(x) = board_numbers.get_mut(m) {
                *x = true;
            }

            fn check(hashmap: &IndexMap<i32, bool>, range: Range<i32>) -> bool {
                hashmap
                    .iter()
                    .skip(range.start.try_into().unwrap())
                    .take(5)
                    .all(|(_, &v)| v == true)
            }

            let bingo = check(&board_numbers, 0..5)
                || check(&board_numbers, 5..10)
                || check(&board_numbers, 10..15)
                || check(&board_numbers, 15..20)
                || check(&board_numbers, 20..25);

            if bingo {
                break;
            }
        }

        let sum_uncalled = board_numbers
            .into_iter()
            .filter(|(_, v)| *v == false)
            .map(|(k, _)| k)
            .sum::<i32>();

        results.insert((
            board_id,
            move_order[last_index],
            last_index,
            move_order[last_index] * sum_uncalled,
        ));
        board_id = board_id + 1;
    }

    let winner = results
        .iter()
        .min_by(|(_, _, a, _), (_, _, b, _)| a.partial_cmp(b).unwrap())
        .unwrap();

    let (a, b, c, d) = *winner;

    println!(
        "board {:?} wins with number {:?} on move {:?}. final score for puzzle {:?}",
        a, b, c, d
    );

    return *winner;
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
