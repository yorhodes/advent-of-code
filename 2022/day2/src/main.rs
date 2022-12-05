use core::panic;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

fn parse_move(input: char) -> Move {
    return match input {
        'A' | 'X' => Move::ROCK,
        'B' | 'Y' => Move::PAPER,
        'C' | 'Z' => Move::SCISSORS,
        _ => panic!("invalid move"),
    };
}

#[repr(u32)]
#[derive(PartialEq)]
enum Result {
    LOSS,
    DRAW,
    WIN,
}

fn result(opponent: Move, player: Move) -> Result {
    let diff = (3 + opponent as u32 - player as u32) % 3;
    return match diff {
        0 => Result::DRAW,
        1 => Result::LOSS,
        2 => Result::WIN,
        _ => panic!("invalid result")
    };
}

fn score(opponent: Move, player: Move) -> u32 {
    let result_score = match result(opponent, player) {
        Result::LOSS => 0,
        Result::DRAW => 3,
        Result::WIN => 6
    };
    let move_score = match player {
        Move::ROCK => 1,
        Move::PAPER => 2,
        Move::SCISSORS => 3
    };
    return result_score + move_score;
}

fn part_1(input: &str) -> u32 {
    return input
        .lines()
        .map(|move_string| -> u32 {
            let opponent = parse_move(move_string.chars().nth(0).unwrap());
            let player = parse_move(move_string.chars().nth(2).unwrap());
            return score(opponent, player);
        })
        .sum();
}

fn gen_move(opponent: Move, player: char) -> Move {
    let desired = match player {
        'X' => Result::LOSS,
        'Y' => Result::DRAW,
        'Z' => Result::WIN,
        _ => panic!("invalid result")
    };
    for potential_move in [Move::ROCK, Move::PAPER, Move::SCISSORS] {
        if result(opponent, potential_move) == desired {
            return potential_move;
        }
    }
    panic!("invalid move");
}

fn part_2(input: &str) -> u32 {
    return input
        .lines()
        .map(|move_string| -> u32 {
            let opponent = parse_move(move_string.chars().nth(0).unwrap());
            let player = gen_move(opponent, move_string.chars().nth(2).unwrap());
            return score(opponent, player);
        })
        .sum();
}

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("part 1: {}", part_1(DATA));
    println!("part 2: {}", part_2(DATA));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("../test.txt");

    #[test]
    fn test_1() {
        assert_eq!(part_1(SAMPLE_DATA), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), 12);
    }
}
