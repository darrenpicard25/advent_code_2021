use super::read_file;
use std::collections::HashSet;
use std::io::BufRead;

fn get_map() -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let file = read_file::read_file("./data/day_9_data.txt");

    for line in file.lines() {
        let line = line.unwrap();
        map.push(
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    map
}

fn get_value(row_index: i8, col_index: i8, map: &Vec<Vec<u8>>) -> &u8 {
    if row_index >= 0 {
        map.get(row_index as usize)
            .and_then(|x| {
                if col_index >= 0 {
                    x.get((col_index) as usize)
                } else {
                    None
                }
            })
            .unwrap_or(&10)
    } else {
        &10
    }
}

fn get_low_points(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut low_points: Vec<(usize, usize)> = Vec::new();

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, position) in row.iter().enumerate() {
            let above = get_value(row_index as i8 - 1, column_index as i8, map);
            let below = get_value(row_index as i8 + 1, column_index as i8, map);
            let left = get_value(row_index as i8, column_index as i8 - 1, map);
            let right = get_value(row_index as i8, column_index as i8 + 1, map);

            if position < above && position < left && position < right && position < below {
                low_points.push((column_index, row_index));
            }
        }
    }

    low_points
}

fn get_basin_size(map: &Vec<Vec<u8>>, point: (usize, usize)) -> i32 {
    let mut basin: HashSet<(usize, usize)> = HashSet::new();

    let mut previous_size = basin.len();

    basin.insert(point);

    while previous_size != basin.len() {
        let temp = basin.clone();

        temp.iter().for_each(|(col_index, row_index)| {
            let above = get_value(*row_index as i8 - 1, *col_index as i8, map);

            if above < &9 {
                basin.insert((*col_index, *row_index - 1));
            }
            let below = get_value((row_index + 1) as i8, *col_index as i8, map);

            if below < &9 {
                basin.insert((*col_index, *row_index + 1));
            }
            let left = get_value(*row_index as i8, *col_index as i8 - 1, map);

            if left < &9 {
                basin.insert((*col_index - 1, *row_index));
            }
            let right = get_value(*row_index as i8, (*col_index + 1) as i8, map);

            if right < &9 {
                basin.insert((*col_index + 1, *row_index));
            }
        });

        previous_size = temp.len();
    }

    basin.len() as i32
}

pub fn sum_of_basins() -> i32 {
    let map = get_map();

    let low_points = get_low_points(&map);

    let mut basin_sizes = low_points
        .iter()
        .map(|point| get_basin_size(&map, *point))
        .collect::<Vec<i32>>();

    basin_sizes.sort();

    basin_sizes.get(basin_sizes.len() - 1).unwrap()
        * basin_sizes.get(basin_sizes.len() - 2).unwrap()
        * basin_sizes.get(basin_sizes.len() - 3).unwrap()
}

pub fn sum_of_height_map_low_points() -> i32 {
    let map = get_map();

    let low_points = get_low_points(&map);

    low_points
        .iter()
        .map(|(x, y)| (map.get(*y).unwrap().get(*x).unwrap() + 1) as i32)
        .sum()
}
