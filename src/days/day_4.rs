use super::read_file;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Board {
    board: Vec<Vec<(i32, bool)>>,
    is_winner: bool,
}

impl Board {
    pub fn new(board: Vec<Vec<i32>>) -> Self {
        board.iter().for_each(|row| assert!(row.len() == 5));
        assert!(board.len() == 5);
        let board = board
            .into_iter()
            .map(|row| row.into_iter().map(|num| (num, false)).collect())
            .collect();
        Board {
            board,
            is_winner: false,
        }
    }

    fn is_winner(&self) -> bool {
        let has_complete_row = self.board.iter().any(|row| row.iter().all(|(_, n)| *n));

        let has_complete_column = self
            .board
            .iter()
            .take(1)
            .enumerate()
            .any(move |(index, _)| {
                self.board.iter().all(|row| {
                    let (_, is_found) = row.get(index).unwrap();
                    *is_found
                })
            });

        has_complete_row || has_complete_column
    }

    pub fn check_off_number(&mut self, num: i32) -> bool {
        self.board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|(value, is_found)| {
                if *value == num {
                    *is_found = true;
                }
            })
        });

        let is_winner = self.is_winner();

        self.is_winner = is_winner;

        is_winner
    }

    fn sum_unmarked(&self) -> i32 {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|(_, is_found)| !is_found)
                    .map(|(value, _)| value)
                    .sum::<i32>()
            })
            .sum()
    }
}

fn get_file() -> BufReader<File> {
    read_file::read_file("./data/day_4_test_data.txt")
}

fn get_draw_list() -> Vec<i32> {
    let file = get_file();
    let mut lines = file.lines().map(|line| line.unwrap());

    let first_line = lines.nth(0).unwrap();

    Vec::from_iter(first_line.split(',').map(|word| word.parse().unwrap()))
}

fn get_boards() -> Vec<Board> {
    let file = get_file();

    let lines = file.lines().map(|line| line.unwrap());

    let mut boards: Vec<Board> = Vec::new();
    let mut table: Vec<Vec<i32>> = Vec::new();

    lines.into_iter().skip(2).for_each(|s| match s {
        e if e.is_empty() => {
            let board = Board::new(table.clone());
            table.clear();
            boards.push(board)
        }
        line => {
            let row: Vec<i32> = line
                .trim()
                .split_ascii_whitespace()
                .into_iter()
                .map(|element| element.parse::<i32>().unwrap())
                .collect();
            table.push(row);
        }
    });

    if !table.is_empty() {
        let board = Board::new(table.clone());
        table.clear();
        boards.push(board)
    }

    boards
}

pub fn determine_winner() -> Option<i32> {
    let draw_list = get_draw_list();
    let mut boards = get_boards();

    let mut winning_number = 0;

    'main: for num in draw_list {
        for board in &mut boards {
            let winner = board.check_off_number(num);

            if winner {
                winning_number = num;
                break 'main;
            }
        }
    }

    let winning_board = boards.iter().find(|board| board.is_winner);

    match winning_board {
        Some(b) => Some(b.sum_unmarked() * winning_number),
        _ => None,
    }
}

pub fn determine_last_place() -> Option<i32> {
    let draw_list = get_draw_list();
    let mut boards = get_boards();

    for num in &draw_list {
        for board in &mut boards {
            board.check_off_number(*num);
        }

        let losers: Vec<&Board> = boards.iter().filter(|board| !board.is_winner).collect();

        if losers.len() == 1 {
            break;
        }
    }

    let mut last_place: Vec<Board> = boards
        .into_iter()
        .filter(|board| !board.is_winner)
        .collect();
    let last_place = last_place.get_mut(0).unwrap();

    for num in &draw_list {
        let is_complete = last_place.check_off_number(*num);

        if is_complete {
            return Some(num * last_place.sum_unmarked());
        }
    }

    None
}
