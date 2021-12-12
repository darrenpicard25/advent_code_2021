use super::read_file;
use std::io::BufRead;

fn calc_array_score(remaining: Vec<char>) -> u64 {
    remaining.iter().rev().fold(0, |acc, char| {
        let score = match char {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unknown bracket"),
        };
        acc * 5 + score
    })
}

fn get_incomplete_line_data(line: String) -> Option<u64> {
    let mut tracker: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '<' | '[' | '{' | '(' => tracker.push(c),
            ')' => {
                let opening_bracket = tracker.pop();
                if Some('(') != opening_bracket {
                    return None;
                }
            }
            ']' => {
                let opening_bracket = tracker.pop();
                if Some('[') != opening_bracket {
                    return None;
                }
            }
            '}' => {
                let opening_bracket = tracker.pop();
                if Some('{') != opening_bracket {
                    return None;
                }
            }
            '>' => {
                let opening_bracket = tracker.pop();
                if Some('<') != opening_bracket {
                    return None;
                }
            }
            _ => panic!("Unknown bracket"),
        }
    }

    let score = calc_array_score(tracker);
    Some(score)
}

fn calculate_line_syntax_error_score(line: String) -> i32 {
    let mut tracker: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '<' | '[' | '{' | '(' => tracker.push(c),
            ')' => {
                let opening_bracket = tracker.pop();
                if Some('(') != opening_bracket {
                    return 3;
                }
            }
            ']' => {
                let opening_bracket = tracker.pop();
                if Some('[') != opening_bracket {
                    return 57;
                }
            }
            '}' => {
                let opening_bracket = tracker.pop();
                if Some('{') != opening_bracket {
                    return 1197;
                }
            }
            '>' => {
                let opening_bracket = tracker.pop();
                if Some('<') != opening_bracket {
                    return 25137;
                }
            }
            _ => panic!("Unknown bracket"),
        }
    }

    0
}

pub fn cal_total_syntax_error() -> i32 {
    let file = read_file::read_file("./data/day_10_data.txt");

    let mut score = 0;

    for line in file.lines() {
        let line = line.unwrap();

        score += calculate_line_syntax_error_score(line);
    }

    score
}

pub fn cal_incomplete_total() -> u64 {
    let file = read_file::read_file("./data/day_10_data.txt");

    let mut scores: Vec<u64> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();

        let score = get_incomplete_line_data(line);

        if let Some(s) = score {
            scores.push(s)
        }
    }

    scores.sort();

    *scores.get(scores.len() / 2).unwrap()
}
