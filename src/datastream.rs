//! This module collects functionality related to the data stream that is send
//! to the malfunctioning handheld device from the elves.
//!
//! The data stream is a sequence of characters. The characters are converted
//! to a number in the range 0..=25. The characters that are not a lowercase
//! letter are discarded.

/// Converts a character to a number in the range 0..=25. Or returns 42 if
/// the character is not a lowercase letter.
fn char_to_u8(c: char) -> u8 {
    if c as u8 >= b'a' && c as u8 <= b'z' {
        c as u8 - b'a'
    } else {
        42
    }
}

/// Converts a puzzle input into a datastream.
pub fn input_to_u8s(input: &str) -> Vec<u8> {
    input.chars().map(char_to_u8).filter(|c| *c != 42).collect()
}

/// Returns the start and end of the first start-of-message marker in the
/// input. The start is the index of the first character of the marker, and
/// the end is the index AFTER the last character of the marker.
pub fn find_start_of_message_marker(n: usize, input: &[u8]) -> (usize, usize) {
    // Make an array of 26 counters, one for each letter.
    let mut counters = [0; 26];
    let mut duplication_counter = 0;
    // Initialize the counters with the first n letters.
    for i in 0..n {
        counters[input[i] as usize] += 1;
        if counters[input[i] as usize] == 2 {
            duplication_counter += 1;
        }
    }
    // Now slide the window over the input, updating the counters and
    // duplication_counter as we go.
    let mut start = 0;
    let mut end = n;
    // If we have a start-of-message marker, we're done.
    while duplication_counter > 0 {
        // Update the counters and duplication_counter.
        counters[input[start] as usize] -= 1;
        if counters[input[start] as usize] == 1 {
            duplication_counter -= 1;
        }
        counters[input[end] as usize] += 1;
        if counters[input[end] as usize] == 2 {
            duplication_counter += 1;
        }

        // Move the window.
        start += 1;
        end += 1;
    }
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = input_to_u8s("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(find_start_of_message_marker(4, &input), (3, 7));
        assert_eq!(find_start_of_message_marker(14, &input), (5, 19));
    }

    #[test]
    fn test2() {
        let input = input_to_u8s("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(find_start_of_message_marker(4, &input), (1, 5));
        assert_eq!(find_start_of_message_marker(14, &input), (9, 23));
    }

    #[test]
    fn test3() {
        let input = input_to_u8s("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(find_start_of_message_marker(4, &input), (2, 6));
        assert_eq!(find_start_of_message_marker(14, &input), (9, 23));
    }

    #[test]
    fn test4() {
        let input = input_to_u8s("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(find_start_of_message_marker(4, &input), (6, 10));
        assert_eq!(find_start_of_message_marker(14, &input), (15, 29));
    }

    #[test]
    fn test5() {
        let input = input_to_u8s("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(find_start_of_message_marker(4, &input), (7, 11));
        assert_eq!(find_start_of_message_marker(14, &input), (12, 26));
    }
}
