mod days;

fn main() {
    let results_2 = days::day_2::calculate_position();
    println!("Results of Day 3: {}", results_2);

    let results_3 = days::day_3::power_consumption();

    println!("Results of Day 3 Part 1: {}", results_3);

    let results_3 = days::day_3::life_support_rate();

    println!("Results of Day 3 Part 2: {}", results_3);

    days::day_4::get_game_boards_and_numbers();
}
