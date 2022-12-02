mod day1;
mod day2;
mod day3;
mod util;

use util::*;

fn main() {
    day1::day1();
    println!("Day 2: {:?}", day2::day2());
    day3::day3("day3-test.txt");
    day3::day3("day3.txt");
}
