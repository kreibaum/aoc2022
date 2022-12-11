use std::collections::VecDeque;

use crate::util::*;

pub fn day11(filename: &str) {
    let input = read_file(filename);

    let mut monkeys = parse_monkeys(&input);

    for i in 0..20 {
        simulate_all_monkeys(&mut monkeys);
    }
    // Print the monkeys one by one
    for monkey in monkeys {
        println!("{:?}", monkey);
    }
    println!();

    // 55388 is too low for part 1.
    // That is what you get for manually multiplying the starting items by 19...

    
}

fn simulate_all_monkeys(monkeys: &mut [Monkey]) {
    for monkey_id in 0..monkeys.len() {
        simulate_one_monkey(monkeys, monkey_id);
    }
}

fn simulate_one_monkey(monkeys: &mut [Monkey], monkey_id: usize) {
    while monkeys[monkey_id].items.len() > 0 {
        simulate_one_item(monkeys, monkey_id);
    }
}

fn simulate_one_item(monkeys: &mut [Monkey], monkey_id: usize) {
    let monkey = &mut monkeys[monkey_id];
    monkey.inspection_counter += 1;
    let item = monkey.items.pop_front().unwrap();
    let new_item = match monkey.operation {
        Operation::Add(x) => item + x,
        Operation::Multiply(x) => item * x,
        Operation::Square => item * item,
    };
    // Divide item value by 3 and round down
    let new_item = new_item / 3;
    if new_item % monkey.test == 0 {
        monkeys[monkey.true_monkey].items.push_back(new_item);
    } else {
        monkeys[monkey.false_monkey].items.push_back(new_item);
    }
}

// Monkey definition:
// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
// Monkey can be parsed woth the regex
// (\d+):\s+Starting items: ([\d, ]+)\s+Operation: new = old ([\+\*]) (\d+|old)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)
#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u128>,
    operation: Operation,
    test: u128,
    true_monkey: usize,
    false_monkey: usize,
    inspection_counter: usize,
}
#[derive(Debug)]
enum Operation {
    Add(u128),
    Multiply(u128),
    Square,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    // Use a regex to parse the input into a vector of monkeys
    let regex = regex::Regex::new(
        r"(?s)Monkey (\d+):\s+Starting items: ([\d, ]+)\s+Operation: new = old ([\+\*]) (\d+|old)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)",
    ).unwrap();

    let mut monkeys = Vec::new();

    // Iternate over all matches of this regex
    for r_match in regex.find_iter(&input) {
        let capture = regex.captures(r_match.as_str()).unwrap();
        // If the operation number at index 4 is "old", then we use the Square operation.
        // Otherwise, parse the number and use the Add or Multiply operation.
        let operation = match &capture[4] {
            "old" => Operation::Square,
            _ => match &capture[3] {
                "+" => Operation::Add(capture[4].parse().unwrap()),
                "*" => Operation::Multiply(capture[4].parse().unwrap()),
                _ => panic!("Unknown operation"),
            },
        };

        let monkey = Monkey {
            id: capture[1].parse().unwrap(),
            items: capture[2]
                .trim()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            operation,
            test: capture[5].parse().unwrap(),
            true_monkey: capture[6].parse().unwrap(),
            false_monkey: capture[7].parse().unwrap(),
            inspection_counter: 0,
        };
        monkeys.push(monkey);
    }

    monkeys
}
