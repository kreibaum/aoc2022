use std::collections::HashMap;

use crate::util::*;
use lazy_regex::regex_captures;

// --- Day 10: Cathode-Ray Tube ---

// You avoid the ropes, plunge into the river, and swim to shore.

// The Elves yell something about meeting back up with them upriver, but the river is too loud to tell exactly what they're saying. They finish crossing the bridge and disappear from view.

// Situations like this must be why the Elves prioritized getting the communication system on your handheld device working. You pull it out of your pack, but the amount of water slowly draining from a big crack in its screen tells you it probably won't be of much immediate use.

// Unless, that is, you can design a replacement for the device's video system! It seems to be some kind of cathode-ray tube screen and simple CPU that are both driven by a precise clock circuit. The clock circuit ticks at a constant rate; each tick is called a cycle.

// Start by figuring out the signal being sent by the CPU. The CPU has a single register, X, which starts with the value 1. It supports only two instructions:

//     addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
//     noop takes one cycle to complete. It has no other effect.

// The CPU uses these instructions in a program (your puzzle input) to, somehow, tell the screen what to draw.

// Consider the following small program:

// noop
// addx 3
// addx -5

// Execution of this program proceeds as follows:

//     At the start of the first cycle, the noop instruction begins execution. During the first cycle, X is 1. After the first cycle, the noop instruction finishes execution, doing nothing.
//     At the start of the second cycle, the addx 3 instruction begins execution. During the second cycle, X is still 1.
//     During the third cycle, X is still 1. After the third cycle, the addx 3 instruction finishes execution, setting X to 4.
//     At the start of the fourth cycle, the addx -5 instruction begins execution. During the fourth cycle, X is still 4.
//     During the fifth cycle, X is still 4. After the fifth cycle, the addx -5 instruction finishes execution, setting X to -1.

// Maybe you can learn something by looking at the value of the X register throughout execution. For now, consider the signal strength (the cycle number multiplied by the value of the X register) during the 20th cycle and every 40 cycles after that (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles).

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}
#[derive(Debug)]
struct CPU {
    x: i32,
    cycle: i32,
    instr_index: usize,
    instructions: Vec<Instruction>,
}

pub fn day10(filename: &str) {
    let input = read_file(filename);

    let mut instructions = Vec::new();
    for line in input.lines() {
        if let Some((_, value)) = regex_captures!(r"^addx (-?\d+)$", line) {
            let instruction = Instruction::Addx(value.parse().unwrap());
            instructions.push(instruction);
        } else if let Some(_) = regex_captures!(r"^noop$", line) {
            let instruction = Instruction::Noop;
            instructions.push(instruction);
        }
    }

    let mut cpu = CPU {
        x: 1,
        cycle: 1,
        instr_index: 0,
        instructions,
    };

    let mut strength_map: HashMap<usize, i32> = HashMap::new();
    // Simulate the first 20 instructions
    for _ in 0..222 {
        one_step(&mut cpu);
        // println!("Cycle: {}, X: {}", cpu.cycle, cpu.x);

        let cycle_even = if (cpu.cycle % 2) == 0 {
            cpu.cycle
        } else {
            cpu.cycle + 1
        };

        if cycle_even == 20 || cycle_even % 40 == 20 {
            // println!(
            //     "Cycle: {}, X: {}, Signal strength: {}",
            //     cycle_even,
            //     cpu.x,
            //     cycle_even * cpu.x
            // );
            strength_map.insert(cycle_even as usize, cycle_even * cpu.x);
        }
    }
    // Sum everything in the map
    let mut sum = 0;
    for (_, value) in strength_map {
        sum += value;
    }
    println!("Sum: {}", sum);

    println!();

    // Part 2, drawing the sprite
    // Reset the CPU
    let mut instructions = Vec::new();
    for line in input.lines() {
        if let Some((_, value)) = regex_captures!(r"^addx (-?\d+)$", line) {
            let instruction = Instruction::Addx(value.parse().unwrap());
            instructions.push(instruction);
        } else if let Some(_) = regex_captures!(r"^noop$", line) {
            let instruction = Instruction::Noop;
            instructions.push(instruction);
        }
    }

    let mut cpu = CPU {
        x: 1,
        cycle: 0,
        instr_index: 0,
        instructions,
    };

    // Execute the program and put all the x values into a vector.
    let mut x_values = Vec::new();
    let mut cycle = 0;
    for _ in 0..242 {
        let old_x = cpu.x;
        one_step(&mut cpu);
        while cycle < cpu.cycle {
            x_values.push(old_x);
            cycle += 1;
        }
    }
    println!("x_values: {:?}", x_values);

    // Print out a 40x6 grid
    // If the x index is within +/- 1 of the x_values entry, print a #
    // Otherwise print a .
    for y in 0..6 {
        for x in 0..40 {
            let x_value = x_values[y * 40 + x];
            let found = x_value >= (x as i32 - 1) && x_value <= (x as i32 + 1);
            if found {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn one_step(cpu: &mut CPU) {
    if cpu.instr_index >= cpu.instructions.len() {
        return;
    }
    let instruction = &cpu.instructions[cpu.instr_index];
    match instruction {
        Instruction::Addx(value) => {
            cpu.x += value;
            cpu.cycle += 2;
        }
        Instruction::Noop => {
            cpu.cycle += 1;
        }
    }
    cpu.instr_index += 1;
}
