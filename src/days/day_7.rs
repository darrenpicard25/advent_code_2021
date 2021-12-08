use super::read_file;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_crab_horizontal_positions(file: BufReader<File>) -> Vec<i32> {
    let iter = file
        .lines()
        .map(|line| line.unwrap())
        .take(1)
        .collect::<String>();

    iter.split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

fn determine_cheap_position(mut positions: Vec<i32>) -> i32 {
    positions.sort();

    let mut map = HashMap::new();

    let first = positions.first().unwrap();
    let last = positions.last().unwrap();

    let positions = &positions;

    for pos in *first..*last {
        if !map.contains_key(&pos) {
            let fuel = positions
                .into_iter()
                .map(|val| {
                    let n = (val - pos).abs();
                    (n.pow(2) + n) / 2
                })
                .sum::<i32>();

            map.insert(pos, fuel);
        }
    }

    let mut min_value = i32::MAX;
    let mut min_pos = 0;

    map.iter().for_each(|(key, value)| {
        if value < &min_value {
            min_value = *value;
            min_pos = *key;
        }
    });

    println!("Key: {}; Value: {}", min_pos, min_value);
    min_pos
}

pub fn determine_cheapest_horizontal_position() -> i32 {
    let file = read_file::read_file("./data/day_7_data.txt");

    let original_positions = get_crab_horizontal_positions(file);

    determine_cheap_position(original_positions)
}
