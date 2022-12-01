mod day1;
mod util;

use util::*;

fn main() {
    day1::day1();

    // day2();
}

fn day2() -> (usize, usize) {
    let input = read_file("day2.txt");

    // TODO: Implement day 2

    (0, 0)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        assert_eq!(day2(), (0, 0));
    }
}
