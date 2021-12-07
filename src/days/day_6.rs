use super::read_file;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_original_fishes(file: BufReader<File>) -> Vec<usize> {
    let iter = file
        .lines()
        .map(|line| line.unwrap())
        .take(1)
        .collect::<String>();

    iter.split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn calculate_fish_after_x_days(days: i32, original_fish: Vec<usize>) -> u64 {
    let mut map: HashMap<usize, u64> = HashMap::new();

    original_fish.iter().for_each(|age| {
        let num = map.get_mut(age);
        match num {
            Some(x) => {
                *x += 1;
            }
            None => {
                map.insert(*age, 1);
            }
        };
    });

    let mut day = 1;

    while day <= days {
        let age_0 = *map.get(&0).unwrap_or(&0);
        let age_1 = *map.get(&1).unwrap_or(&0);
        let age_2 = *map.get(&2).unwrap_or(&0);
        let age_3 = *map.get(&3).unwrap_or(&0);
        let age_4 = *map.get(&4).unwrap_or(&0);
        let age_5 = *map.get(&5).unwrap_or(&0);
        let age_6 = *map.get(&6).unwrap_or(&0);
        let age_7 = *map.get(&7).unwrap_or(&0);
        let age_8 = *map.get(&8).unwrap_or(&0);

        map.insert(0, age_1);
        map.insert(1, age_2);
        map.insert(2, age_3);
        map.insert(3, age_4);
        map.insert(4, age_5);
        map.insert(5, age_6);
        map.insert(6, age_7 + age_0);
        map.insert(7, age_8);
        map.insert(8, age_0);

        day += 1;
    }

    map.iter().map(|(_, num)| num).sum()
}

pub fn simulate_lantern_fish() -> u64 {
    let number_days = 256;

    let file = read_file::read_file("./data/day_6_data.txt");
    let original_fish = get_original_fishes(file);

    calculate_fish_after_x_days(number_days, original_fish)
}
