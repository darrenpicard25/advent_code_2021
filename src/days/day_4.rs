use super::read_file;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_draw_list(file: BufReader<File>) -> Vec<i32> {
    let mut lines = file.lines().map(|line| line.unwrap());

    let first_line = lines.nth(0).unwrap();

    Vec::from_iter(first_line.split(',').map(|word| word.parse().unwrap()))
}

pub fn get_game_boards_and_numbers() {
    let file = read_file::read_file("./day_4_test_data.txt");

    let draw_list = get_draw_list(file);

    println!("Draw List: {:?}", draw_list);
}
