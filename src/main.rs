mod day1;
mod util;

use util::*;

fn main() {
    day1::day1();

    print!("{:?}", day2());
    // 15684 is wrong
    // 13966 is wrong
    // 15632 is right. Only the score function was broken.
    // Part 2: 14416 is right.
}

// --- Day 2: Rock Paper Scissors ---

// The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

// Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

// The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

// Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

// For example, suppose you were given the following strategy guide:

// A Y
// B X
// C Z

// This strategy guide predicts and recommends the following:

//     In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
//     In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
//     The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

// What would your total score be if everything goes exactly according to your strategy guide?

// --- Part Two ---

// The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

// The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

//     In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
//     In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
//     In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

// Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

#[derive(Debug, PartialEq)]
struct Round {
    opponent: char,
    you: char,
}

fn day2() -> (usize, usize) {
    let input = read_file("day2.txt");

    let mut rounds = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent = chars.next().unwrap();
        chars.next().unwrap();
        let you = chars.next().unwrap();
        rounds.push(Round { opponent, you });
    }

    // Print all the scores
    for round in &rounds {
        println!("{:?}", round);
        println!("Score: {}", score(round));
    }

    // Get the sum of the scores for each round.
    (
        rounds.iter().map(|r| score(r)).sum(),
        rounds.iter().map(|r| score(&score2(r))).sum(),
    )
}

// A = Rock, B = Paper, C = Scissors
// X = Lose, Y = Draw, Z = Win
fn score2(round: &Round) -> Round {
    match *round {
        Round {
            opponent: 'A',
            you: 'X',
        } => Round {
            opponent: 'A',
            you: 'Z',
        }, // Lose to Rock with Scissors
        Round {
            opponent: 'A',
            you: 'Y',
        } => Round {
            opponent: 'A',
            you: 'X',
        }, // Draw with Rock with Rock
        Round {
            opponent: 'A',
            you: 'Z',
        } => Round {
            opponent: 'A',
            you: 'Y',
        }, // Win to Rock with Paper
        Round {
            opponent: 'B',
            you: 'X',
        } => Round {
            opponent: 'B',
            you: 'X',
        }, // Lose to Paper with Rock
        Round {
            opponent: 'B',
            you: 'Y',
        } => Round {
            opponent: 'B',
            you: 'Y',
        }, // Draw with Paper with Paper
        Round {
            opponent: 'B',
            you: 'Z',
        } => Round {
            opponent: 'B',
            you: 'Z',
        }, // Win to Paper with Scissors
        Round {
            opponent: 'C',
            you: 'X',
        } => Round {
            opponent: 'C',
            you: 'Y',
        }, // Lose to Scissors with Paper
        Round {
            opponent: 'C',
            you: 'Y',
        } => Round {
            opponent: 'C',
            you: 'Z',
        }, // Draw with Scissors with Scissors
        Round {
            opponent: 'C',
            you: 'Z',
        } => Round {
            opponent: 'C',
            you: 'X',
        }, // Win to Scissors with Rock
        _ => panic!("Invalid round"),
    }
}

// The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
fn score(round: &Round) -> usize {
    match *round {
        Round {
            opponent: 'A',
            you: 'X',
        } => 1 + 3,
        Round {
            opponent: 'A',
            you: 'Y',
        } => 2 + 6,
        Round {
            opponent: 'A',
            you: 'Z',
        } => 3 + 0,
        Round {
            opponent: 'B',
            you: 'X',
        } => 1 + 0,
        Round {
            opponent: 'B',
            you: 'Y',
        } => 2 + 3,
        Round {
            opponent: 'B',
            you: 'Z',
        } => 3 + 6,
        Round {
            opponent: 'C',
            you: 'X',
        } => 1 + 6,
        Round {
            opponent: 'C',
            you: 'Y',
        } => 2 + 0,
        Round {
            opponent: 'C',
            you: 'Z',
        } => 3 + 3,
        _ => panic!("Invalid round"),
    }
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