mod datastream;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod util;

fn main() {
    day1::day1();
    println!("Day 2: {:?}", day2::day2());
    day3::day3("day3-test.txt");
    day3::day3("day3.txt");
    day4::day4("day4-test.txt");
    day4::day4("day4.txt");
    day5::day5("day5-test.txt", 3, 3);
    day5::day5("day5.txt", 9, 8);
    day6::day6("day6-test.txt");
    day6::day6("day6.txt");
    day7::day7("day7-test.txt");
    day7::day7("day7.txt");
    day8::day8("day8-test.txt");
    day8::day8("day8.txt");
}
