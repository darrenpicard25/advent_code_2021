use super::read_file;
use std::io::BufRead;

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0, aim: 0 }
    }

    pub fn multiply(&self) -> i32 {
        self.x * self.y
    }

    pub fn go_down(&mut self, n: i32) {
        self.aim += n;
    }

    pub fn go_up(&mut self, n: i32) {
        self.aim -= n;
    }

    pub fn go_forward(&mut self, n: i32) {
        self.x += n;
        self.y += self.aim * n
    }

    pub fn go_back(&mut self, n: i32) {
        self.x -= n;
        self.y += self.aim * n
    }
}

pub fn calculate_position() -> i32 {
    let file = read_file::read_file("./day_2_data.txt");

    let mut position = Position::new();

    file.lines().for_each(|line| {
        let temp = line.unwrap();
        let variables = temp.split(' ').collect::<Vec<&str>>();
        let direction = variables.get(0).unwrap();
        let amount = variables.get(1).unwrap().parse::<i32>().unwrap();

        match *direction {
            "forward" => position.go_forward(amount),
            "up" => position.go_up(amount),
            "down" => position.go_down(amount),
            "back" => position.go_back(amount),
            n => panic!("Unknown word: {}", n),
        }
    });

    position.multiply()
}
