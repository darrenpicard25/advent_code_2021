mod days;
fn main() {
    let results_2 = days::day_2::calculate_position();
    println!("Results of Day 3: {}", results_2);

    let results_3 = days::day_3::power_consumption();

    println!("Results of Day 3 Part 1: {}", results_3);

    let results_3 = days::day_3::life_support_rate();

    println!("Results of Day 3 Part 2: {}", results_3);

    let results_4 = days::day_4::determine_winner();

    println!("Results of Day 4 Part 1: {:?}", results_4);

    let results_4 = days::day_4::determine_last_place();

    println!("Results of Day 4 Part 2: {:?}", results_4);

    let results_5 = days::day_5::determine_number_of_bad_spots();

    println!("Results of Day 5 Part 1: {:?}", results_5);

    let result_6 = days::day_6::simulate_lantern_fish();

    println!("Results of Day 6 Part 1: {}", result_6);

    let result_7 = days::day_7::determine_cheapest_horizontal_position();

    println!("Results of Day 7 Part 1: {}", result_7);

    let result_9 = days::day_9::sum_of_height_map_low_points();

    println!("Results of Day 9 Part 1: {}", result_9);

    let result_9 = days::day_9::sum_of_basins();

    println!("Results of Day 9 Part 2: {}", result_9);

    let result_10 = days::day_10::cal_total_syntax_error();

    println!("Results of Day 10 Part 1: {}", result_10);

    let result_10 = days::day_10::cal_incomplete_total();

    println!("Results of Day 10 Part 2: {}", result_10);
}
