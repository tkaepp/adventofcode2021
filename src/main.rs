mod day1;
mod day2;
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
}

fn print_dashes() {
    println!("---------------------------------------");
}
