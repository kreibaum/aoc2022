use std::collections::HashSet;

use crate::util::*;

// Day 9: Rope Bridge ---

// This rope bridge creaks as you walk along it. You aren't sure how old it is, or whether it can even support your weight.

// It seems to support the Elves just fine, though. The bridge spans a gorge which was carved out by the massive river far below you.

// You step carefully; as you do, the ropes stretch and twist. You decide to distract yourself by modeling rope physics; maybe you can even figure out where not to step.

// Consider a rope with a knot at each end; these knots mark the head and the tail of the rope. If the head moves far enough away from the tail, the tail is pulled toward the head.

// Due to nebulous reasoning involving Planck lengths, you should be able to model the positions of the knots on a two-dimensional grid. Then, by following a hypothetical series of motions (your puzzle input) for the head, you can determine how the tail will move.

// Due to the aforementioned Planck lengths, the rope must be quite short; in fact, the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching):

// ....
// .TH.
// ....

// ....
// .H..
// ..T.
// ....

// ...
// .H. (H covers T)
// ...

// If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:

// .....    .....    .....
// .TH.. -> .T.H. -> ..TH.
// .....    .....    .....

// ...    ...    ...
// .T.    .T.    ...
// .H. -> ... -> .T.
// ...    .H.    .H.
// ...    ...    ...

// Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

// .....    .....    .....
// .....    ..H..    ..H..
// ..H.. -> ..... -> ..T..
// .T...    .T...    .....
// .....    .....    .....

// .....    .....    .....
// .....    .....    .....
// ..H.. -> ...H. -> ..TH.
// .T...    .T...    .....
// .....    .....    .....

// You just need to work out where the tail goes as the head follows a series of motions. Assume the head and the tail both start at the same position, overlapping.

pub fn day9(filename: &str) {
    let input = read_file(filename);

    // Parse Instructions: "U 19" is converted to "('U', 19)"
    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<usize>().unwrap();
        instructions.push((direction, distance));
    }

    let mut spaces_seen: HashSet<(isize, isize)> = HashSet::new();
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);

    // Add the starting position to the set of spaces seen
    spaces_seen.insert(head);

    for (direction, distance) in instructions {
        for _ in 0..distance {
            let last_head_position = head;
            match direction {
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => panic!("Unknown direction: {}", direction),
            }
            // If the head is two steps away from the tail, put tail on last head position
            if (head.0 as isize - tail.0 as isize).abs() == 2
                || (head.1 as isize - tail.1 as isize).abs() == 2
            {
                tail = last_head_position;
            }
            // track the spaces seen by the tail
            spaces_seen.insert(tail);
        }
    }

    // Print count of spaces seen by the tail
    println!("Day 9: {}", spaces_seen.len());

    spaces_seen.clear();

    // In part two we have a longer rope with a slightly different following implementation.
    // First, set up the rope with everything being in the same position.
    // Intitalize it to 10 time (0, 0)
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); 10];

    // Parse Instructions: "U 19" is converted to "('U', 19)"
    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<usize>().unwrap();
        instructions.push((direction, distance));
    }

    // Now, for each instruction, move the head and rest of the rope.
    // If the head is two steps away from the tail, make the tail follow the same movement.

    for (direction, distance) in instructions {
        for _ in 0..distance {
            let head = &mut rope[0];
            match direction {
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => panic!("Unknown direction: {}", direction),
            }
            // Make the tail follow the head if it is two steps away
            // If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:
            // Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:
            for i in 1..10 {
                let local_head = rope[i - 1];
                let mut local_tail = rope[i];
                if (local_head.0 as isize - local_tail.0 as isize).abs() == 2
                    && (local_head.1 as isize == local_tail.1 as isize)
                {
                    rope[i].0 = (local_head.0 + rope[i].0) / 2;
                } else if (local_head.1 as isize - local_tail.1 as isize).abs() == 2
                    && (local_head.0 as isize == local_tail.0 as isize)
                {
                    rope[i].1 = (local_head.1 + rope[i].1) / 2;
                } else if (local_head.0 as isize - local_tail.0 as isize).abs() == 2
                    || (local_head.1 as isize - local_tail.1 as isize).abs() == 2
                {
                    // Not touching, we need to move diagonally to catch up.
                    if local_head.0 > local_tail.0 {
                        rope[i].0 += 1;
                    } else if local_head.0 < local_tail.0 {
                        rope[i].0 -= 1;
                    }
                    if local_head.1 > local_tail.1 {
                        rope[i].1 += 1;
                    } else if local_head.1 < local_tail.1 {
                        rope[i].1 -= 1;
                    }
                }
            }
            // track the spaces seen by the tail
            spaces_seen.insert(rope[9]);
        }
        // println!("direction: {}", direction);
        // // Show the rope
        // for i in 0..10 {
        //     println!("{}: {:?}", i, rope[i]);
        // }
    }
    println!("Day 9.2: {}", spaces_seen.len());
    // 2612 is wrong
    // 2607 is right
}
