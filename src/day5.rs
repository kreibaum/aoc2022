use crate::util::*;

pub fn day5(filename: &str) -> (u32, u32) {
    let input = read_file(filename);
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5() {
        assert_eq!(day5("day4-test.txt"), (2, 1));
        assert_eq!(day5("day4.txt"), (2, 1));
    }
}
