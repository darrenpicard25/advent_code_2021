use super::read_file;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(string: &str) -> Self {
        let mut coord = string
            .trim()
            .split(',')
            .map(|string_num| string_num.parse::<i32>().unwrap());

        Position {
            x: coord.next().unwrap(),
            y: coord.next().unwrap(),
        }
    }
}

fn get_file() -> BufReader<File> {
    read_file::read_file("./data/day_5_data.txt")
}

fn determine_line_points(p1: Position, p2: Position) -> Vec<Position> {
    let mut results = Vec::new();

    let points = (p1.x, p1.y, p2.x, p2.y);

    match points {
        (x1, y1, x2, y2) if x1 == x2 && y1 < y2 => {
            results = Vec::from_iter((y1..=y2).map(|y| Position { x: x1, y }));
        }
        (x1, y1, x2, y2) if x1 == x2 && y1 > y2 => {
            results = Vec::from_iter((y2..=y1).map(|y| Position { x: x1, y }));
        }
        (x1, y1, x2, y2) if y1 == y2 && x1 < x2 => {
            results = Vec::from_iter((x1..=x2).map(|x| Position { x, y: y1 }));
        }
        (x1, y1, x2, y2) if y1 == y2 && x1 > x2 => {
            results = Vec::from_iter((x2..=x1).map(|x| Position { x, y: y1 }));
        }
        (x1, y1, x2, y2) if (x1 - x2).abs() == (y1 - y2).abs() && x2 > x1 && y2 > y1 => {
            results = Vec::from_iter((0..=(x2 - x1)).map(|i| Position {
                x: x1 + i,
                y: y1 + i,
            }));
        }
        (x1, y1, x2, y2) if (x1 - x2).abs() == (y1 - y2).abs() && x2 > x1 && y2 < y1 => {
            results = Vec::from_iter((0..=(x2 - x1)).map(|i| Position {
                x: x1 + i,
                y: y1 - i,
            }));
        }
        (x1, y1, x2, y2) if (x1 - x2).abs() == (y1 - y2).abs() && x2 < x1 && y2 > y1 => {
            results = Vec::from_iter((0..=(x1 - x2)).map(|i| Position {
                x: x1 - i,
                y: y1 + i,
            }));
        }
        (x1, y1, x2, y2) if (x1 - x2).abs() == (y1 - y2).abs() && x2 < x1 && y2 < y1 => {
            results = Vec::from_iter((0..=(x1 - x2)).map(|i| Position {
                x: x2 + i,
                y: y2 + i,
            }));
        }
        _ => {}
    }

    results
}

fn assemble_hash_map() -> HashMap<(i32, i32), i32> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let file = get_file();

    let iter = file.lines().flat_map(|item| {
        let item = item.unwrap();
        let item = item.trim();
        let mut coordinates = item.split("->").map(|x| x.trim());

        let first = Position::new(coordinates.next().unwrap());
        let second = Position::new(coordinates.next().unwrap());

        determine_line_points(first, second)
    });

    iter.for_each(|position| {
        let value = map.entry((position.x, position.y)).or_insert(0);

        *value += 1;
    });

    map
}

pub fn determine_number_of_bad_spots() -> usize {
    let map = assemble_hash_map();

    map.iter().filter(|(_, value)| *value >= &2).count()
}

#[cfg(test)]
mod tests {
    use super::{determine_line_points, Position};
    #[test]
    fn the_strange_case() {
        let p1 = Position { x: 5, y: 5 };
        let p2 = Position { x: 8, y: 2 };

        assert_eq!(determine_line_points(p1, p2).len(), 0);
    }
}
