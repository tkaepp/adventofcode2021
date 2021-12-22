mod day1;
mod day2;
mod day3;
mod day4;
mod inputs;

fn main() {
    print_dashes();
    println!("Day 1 with the sample data");
    print_dashes();
    day1::solve_part_one(inputs::day1_sample().to_vec());
    day1::solve_part_two(inputs::day1_sample().to_vec());
    print_dashes();
    println!("Day 1 with the real data");
    print_dashes();
    day1::solve_part_one(inputs::day1().to_vec());
    day1::solve_part_two(inputs::day1().to_vec());

    print_dashes();

    println!("Day 2 with the sample data");
    print_dashes();
    day2::solve_part_one(day2::day2_sample());
    day2::solve_part_two(day2::day2_sample());
    print_dashes();
    println!("Day 2 with the real data");
    print_dashes();
    day2::solve_part_one(day2::day2());
    day2::solve_part_two(day2::day2());

    print_dashes();

    println!("Day 3-1 with the sample data");
    print_dashes();
    day3::powerconsumption(day3::day3_sample(), 5);
    day3::powerconsumption(day3::day3(), "111101110001".len().try_into().unwrap());

    println!("Day 3-2 with the sample data");
    print_dashes();
    day3::life_support_rating(day3::day3_sample(), 5);
    day3::life_support_rating(day3::day3(), "111101110001".len().try_into().unwrap());

    println!("Day 4-1");
    print_dashes();
    day4::bingo(day4::sample());
    day4::bingo(day4::real());

    // println!("Day 4-1");
    // print_dashes();
    // day4::bingo(day4::sample());
    // day4::bingo(day4::real());
}

fn print_dashes() {
    println!("---------------------------------------");
}
