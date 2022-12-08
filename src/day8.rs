use crate::util::*;

// --- Day 8: Treetop Tree House ---

// The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

// First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

// The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

// 30373
// 25512
// 65332
// 33549
// 35390

// Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

// All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

//     The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
//     The top-middle 5 is visible from the top and right.
//     The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
//     The left-middle 5 is visible, but only from the right.
//     The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
//     The right-middle 3 is visible from the right.
//     In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

// With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

// Consider your map; how many trees are visible from outside the grid?

pub fn day8(filename: &str) {
    let input = read_file(filename);

    // Read input into an array of arrays of u8
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(row);
    }
    //println!("Day 8: {:?}", grid);

    // Count visible trees
    let mut visible = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_visible(x, y, &grid) {
                visible += 1;
            }
        }
    }
    println!("Day 8: {}", visible);

    scenic_score(56, 22, &grid);
    if true {
        // Find the tree with the highest scenic_score, then output the scenic_score.
        let mut max_score = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let score = scenic_score(x, y, &grid);
                if score > max_score {
                    max_score = score;
                    max_x = x;
                    max_y = y;
                }
            }
        }
        println!("Day 8: {}", max_score);
        println!("Day 8: ({}, {})", max_x, max_y);
        scenic_score(max_x, max_y, &grid);
    }

    // 166320 is wrong.
}

// A tree's scenic score is found by multiplying together its viewing distance
// in each of the four directions.
// For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
fn scenic_score(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut product = 1;
    let height = forest[y][x];
    println!("Checking tree at ({}, {}), height {}", x, y, height);

    // Find the first tree in each direction that has the same height or is taller.
    // If you reach the edge stop as well.

    // Check left
    let mut distance = 0;
    for i in (0..x).rev() {
        distance += 1;
        println!("    Checking ({}, {}), height {}", i, y, forest[y][i]);
        if forest[y][i] >= height {
            break;
        }
    }
    product *= distance;
    println!("  Left: {}", distance);

    // Check right
    let mut distance = 0;
    for i in x + 1..forest[y].len() {
        distance += 1;
        println!("    Checking ({}, {}), height {}", i, y, forest[y][i]);
        if forest[y][i] >= height {
            break;
        }
    }
    product *= distance;
    println!("  Right: {}", distance);

    // Check up
    let mut distance = 0;
    for i in (0..y).rev() {
        distance += 1;
        println!("    Checking ({}, {}), height {}", x, i, forest[i][x]);
        if forest[i][x] >= height {
            break;
        }
    }
    product *= distance;
    println!("  Up: {}", distance);

    // Check down
    let mut distance = 0;
    for i in y + 1..forest.len() {
        distance += 1;
        println!("    Checking ({}, {}), height {}", x, i, forest[i][x]);
        if forest[i][x] >= height {
            break;
        }
    }
    product *= distance;
    println!("  Down: {}", distance);

    product
}

fn is_visible(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> bool {
    let height = forest[y][x];
    let mut visible = true;

    // Check left
    for i in 0..x {
        if forest[y][i] >= height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    let mut visible = true;

    // Check right
    for i in x + 1..forest[y].len() {
        if forest[y][i] >= height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    let mut visible = true;

    // Check up
    for i in 0..y {
        if forest[i][x] >= height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    let mut visible = true;

    // Check down
    for i in y + 1..forest.len() {
        if forest[i][x] >= height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    false
}
