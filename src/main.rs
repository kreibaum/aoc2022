mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

fn main() {
    day1::day1();
    println!("Day 2: {:?}", day2::day2());
    day3::day3("day3-test.txt");
    day3::day3("day3.txt");
    day4::day4("day4-test.txt");
    day4::day4("day4.txt");
    day5::day5("day5-test.txt");
    day5::day5("day5.txt");
}
