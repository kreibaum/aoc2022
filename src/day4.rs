use crate::util::*;
use lazy_regex::regex_captures;

// --- Day 4: Camp Cleanup ---

// Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.

// However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

// For example, consider the following list of section assignment pairs:

// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8

// For the first few pairs, this list means:

//     Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
//     The Elves in the second pair were each assigned two sections.
//     The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.

// This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

// .234.....  2-4
// .....678.  6-8

// .23......  2-3
// ...45....  4-5

// ....567..  5-7
// ......789  7-9

// .2345678.  2-8
// ..34567..  3-7

// .....6...  6-6
// ...456...  4-6

// .23456...  2-6
// ...45678.  4-8

// Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

// In how many assignment pairs does one range fully contain the other?

// --- Part Two ---

// It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.

// In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

//     5-7,7-9 overlaps in a single section, 7.
//     2-8,3-7 overlaps all of the sections 3 through 7.
//     6-6,4-6 overlaps in a single section, 6.
//     2-6,4-8 overlaps in sections 4, 5, and 6.

// So, in this example, the number of overlapping assignment pairs is 4.

// In how many assignment pairs do the ranges overlap?

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    /// Check if the range fully contains the given other range.
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    /// Check if the range overlaps with the given other range.
    /// This is only false, if the ranges are disjoint.
    /// We check this by checking if one range is fully in front or behind the other.
    fn overlaps(&self, other: &Range) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}

fn parse_line(line: &str) -> (Range, Range) {
    let captures = regex_captures!(r"^(\d+)-(\d+),(\d+)-(\d+)$", line).unwrap();
    let r1 = Range {
        start: captures.1.parse().unwrap(),
        end: captures.2.parse().unwrap(),
    };
    let r2 = Range {
        start: captures.3.parse().unwrap(),
        end: captures.4.parse().unwrap(),
    };
    (r1, r2)
}

pub fn day4(filename: &str) -> (u32, u32) {
    let input = read_file(filename);

    let ranges: Vec<(Range, Range)> = input.lines().map(parse_line).collect();

    // Check if either range fully contains the other range
    let mut count_fully_contained = 0;
    for (r1, r2) in &ranges {
        if r1.contains(r2) || r2.contains(r1) {
            count_fully_contained += 1;
        }
    }

    println!("Day 4: {}", count_fully_contained);

    // Part 2: Check if the ranges overlap at all
    let mut count_overlaps = 0;
    for (r1, r2) in &ranges {
        // If either bound is within the other range, they overlap
        // If a range completely contains the other, they overlap
        if r1.overlaps(r2) {
            count_overlaps += 1;
        }
    }
    println!("Day 4: {}", count_overlaps);

    (count_fully_contained, count_overlaps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4() {
        assert_eq!(day4("day4-test.txt"), (2, 4));
        assert_eq!(day4("day4.txt"), (560, 839));
    }
}
