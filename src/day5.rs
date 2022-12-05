use crate::util::*;
use lazy_regex::regex_captures;

/// --- Day 5: Supply Stacks ---
///
/// The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
///
/// The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
///
/// The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
///
/// They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input).
pub fn day5(filename: &str, w: usize, h: usize) -> (String, String) {
    let input = read_file(filename);

    let mut stacks = parse_stacks(&input, w, h);
    let instructions = parse_instructions(&input);

    execute_ungrouped(instructions, &mut stacks);

    let part1 = read_solution(&stacks);
    println!("Day 5, Part 1: {}", part1);

    // Part 2: Group the stacks
    let mut stacks = parse_stacks(&input, w, h);
    let instructions = parse_instructions(&input);

    execute_grouped(instructions, &mut stacks);

    let part2 = read_solution(&stacks);
    println!("Day 5, Part 2: {}", part2);

    (part1, part2)
}

/// Combine the top elements of the stacks into a string
fn read_solution(stacks: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for i in 0..stacks.len() {
        let c = *stacks[i].last().unwrap();
        result.push(c);
    }
    result
}

/// Execute the instructions, part 2
fn execute_grouped(instructions: Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<char>>) {
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
    }
}

/// Execute the instructions, part 1
fn execute_ungrouped(instructions: Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<char>>) {
    for (from, to, count) in instructions {
        // Move the top count elements from the from_stack to the to_stack
        // This is done one by one and reversing the order of the elements
        let mut i = 0;
        while i < count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
            i += 1;
        }
    }
}

fn parse_stacks(input: &str, w: usize, h: usize) -> Vec<Vec<char>> {
    // First parse this input row by row into a vector of vectors of chars
    //    [D]
    //[N] [C]
    //[Z] [M] [P]
    // 1   2   3
    // Should be converted to
    // [['Z', 'N'], ['M', 'C', 'D'], ['P']]
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
    stacks
}

fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
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
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5() {
        assert_eq!(
            day5("day5-test.txt", 3, 3),
            ("CMZ".to_string(), "MCD".to_string())
        );
        assert_eq!(
            day5("day5.txt", 9, 8),
            ("SVFDLGLWV".to_string(), "DCVTCVPCL".to_string())
        );
    }
}
