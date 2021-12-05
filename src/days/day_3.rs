use super::read_file;
use std::io::BufRead;

fn get_binary_data() -> Vec<String> {
    let file = read_file::read_file("./day_3_data.txt");

    let mut binary_data: Vec<String> = Vec::new();

    file.lines()
        .map(|line| line.unwrap())
        .for_each(|word| binary_data.push(word));

    binary_data
}

fn calculate_index_frequency(binary_data: &Vec<String>) -> Vec<i32> {
    let mut frequency_tracker: Vec<i32> = Vec::new();

    binary_data.iter().for_each({
        |bit| {
            bit.trim().char_indices().for_each(|(index, char)| {
                let value = frequency_tracker.get_mut(index);

                match value {
                    Some(x) if char == '0' => {
                        *x -= 1;
                    }
                    Some(x) => *x += 1,
                    None if char == '0' => frequency_tracker.insert(index, -1),
                    None => frequency_tracker.insert(index, 1),
                }
            })
        }
    });

    frequency_tracker
}

fn calculate_gamma_rate(index_frequency: &Vec<i32>) -> i32 {
    let gamma_rate = index_frequency
        .iter()
        .map(|num| match num {
            num if num < &0 => '1',
            _ => '0',
        })
        .collect::<String>();

    isize::from_str_radix(&gamma_rate, 2).unwrap() as i32
}

fn calculate_epsilon_rate(index_frequency: &Vec<i32>) -> i32 {
    let gamma_rate = index_frequency
        .iter()
        .map(|num| match num {
            num if num > &0 => '1',
            _ => '0',
        })
        .collect::<String>();

    isize::from_str_radix(&gamma_rate, 2).unwrap() as i32
}

pub fn power_consumption() -> i32 {
    let binary_data = get_binary_data();
    let index_frequency = calculate_index_frequency(&binary_data);

    calculate_epsilon_rate(&index_frequency) * calculate_gamma_rate(&index_frequency)
}

fn calculate_oxygen_generator_rate(mut binary_data: Vec<String>) -> i32 {
    let mut index = 0;
    while binary_data.len() != 1 {
        let frequency = calculate_index_frequency(&binary_data);
        let index_frequency = frequency.get(index).unwrap();

        let filter_char = if index_frequency >= &0 { '1' } else { '0' };

        binary_data = binary_data
            .into_iter()
            .filter(|binary| binary.chars().nth(index).unwrap() == filter_char)
            .collect::<Vec<String>>();

        index += 1
    }

    isize::from_str_radix(binary_data.get(0).unwrap(), 2).unwrap() as i32
}

fn calculate_co2_scrubber_rate(mut binary_data: Vec<String>) -> i32 {
    let mut index = 0;
    while binary_data.len() != 1 {
        let frequency = calculate_index_frequency(&binary_data);
        let index_frequency = frequency.get(index).unwrap();

        let filter_char = if index_frequency >= &0 { '0' } else { '1' };

        binary_data = binary_data
            .into_iter()
            .filter(|binary| binary.chars().nth(index).unwrap() == filter_char)
            .collect::<Vec<String>>();

        index += 1
    }

    isize::from_str_radix(binary_data.get(0).unwrap(), 2).unwrap() as i32
}

pub fn life_support_rate() -> i32 {
    let binary_data = get_binary_data();

    calculate_oxygen_generator_rate(binary_data.clone()) * calculate_co2_scrubber_rate(binary_data)
}
