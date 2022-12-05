use crate::util::*;
use lazy_regex::regex_captures;

pub fn day5(filename: &str, w: usize, h: usize) -> (u32, u32) {
    let input = read_file(filename);
    println!("Day 5: {}, {}, {}", filename, w, h);

    // First parse this input row by row into a vector of vectors of chars

    //    [D]
    //[N] [C]
    //[Z] [M] [P]
    // 1   2   3
    // Should be converted to
    // [['Z', 'N'], ['M', 'C', 'D'], ['P']]

    // [[' ', 'D', ' '],[N, C, ' '],[Z, M, P]]

    let mut stacks: Vec<Vec<char>> = Vec::new();
    // put w empty stacks in the vector
    for _ in 0..w {
        stacks.push(Vec::new());
    }

    let lines_cp = input.lines().collect::<Vec<&str>>();

    for i in 0..h {
        let row = lines_cp[h - i - 1];

        for j in 0..w {
            // Map 0 -> 1, 2 -> 5, 4 -> 9, 6 -> 13, 8 -> 17
            let j2 = 4 * j + 1;
            let c = row.chars().nth(j2).unwrap();
            if c != ' ' {
                stacks[j].push(c);
            }
        }
    }

    println!("Stacks: {:?}", stacks);

    // Read the instructions. Each instruction is a (usize, usize, usize) pair
    // Parsing 'move 6 from 2 to 1' with a regex.

    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();
    for line in input.lines() {
        if let Some(captures) = regex_captures!(r"^move (\d+) from (\d+) to (\d+)$", line) {
            let from = captures.2.parse().unwrap();
            let to = captures.3.parse().unwrap();
            let count = captures.1.parse().unwrap();
            instructions.push((from, to, count));
        }
    }

    println!("Instructions: {:?}", instructions);

    // Execute the instructions

    for (from, to, count) in instructions {
        // Move the top count elements from the from_stack to the to_stack
        // Make sure to retain the ordering of the elements

        // First, pop the top count elements from the from_stack
        let mut intermediate = Vec::new();
        let mut i = 0;
        while i < count {
            let c = stacks[from - 1].pop().unwrap();
            intermediate.push(c);
            i += 1;
        }

        // Now push them onto the to_stack
        while let Some(c) = intermediate.pop() {
            stacks[to - 1].push(c);
        }

        // We deleted the part 1 solution here...
    }

    // Print stack result:

    println!("Stacks: {:?}", stacks);

    // Combine the top elements of the stacks into a string

    let mut result = String::new();
    for i in 0..w {
        let c = stacks[i].pop().unwrap();
        result.push(c);
    }

    println!("Result: {}", result);

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
