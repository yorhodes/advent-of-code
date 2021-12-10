use std::collections::HashSet;
use std::iter::FromIterator;

const SENTINEL_DIGIT: usize = 10;

fn len_to_digit(len: usize) -> usize {
    match len {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => SENTINEL_DIGIT
    }
}

fn match_known_len(s: &str) -> bool {
    len_to_digit(s.len()) != SENTINEL_DIGIT
}

fn match_known_size(s: &HashSet<char>) -> bool {
    len_to_digit(s.len()) != SENTINEL_DIGIT
}

fn part_1(input: &str) -> usize {
    let split_lines = input
        .trim()
        .split('\n')
        .map(|line| line.split_once('|').expect("string | string"));
    return split_lines
        .map(|(_signals, outputs)| {
            outputs
                .split_whitespace()
                .filter(|s| match_known_len(s))
                .count()
        })
        .sum();
}

fn char_set(s: &str) -> HashSet<char> {
    HashSet::from_iter(s.chars())
}

fn part_2(input: &str) -> usize {
    let split_lines = input
        .trim()
        .split('\n')
        .map(|line| line.split_once('|').expect("string | string"));

    return split_lines
        .map(|(signal, output)| -> usize {
            let mut digit_signal_sets = vec![HashSet::new(); 10];
            let signals = signal.split_whitespace().map(|s| char_set(s));

            // determine known digit signals
            signals.clone().filter(|s| match_known_size(s)).for_each(|set| {
                let digit = len_to_digit(set.len());
                digit_signal_sets[digit] = set;
            });

            // determine 6 signal digits
            signals.clone().filter(|s| s.len() == 6).for_each(|six_chars| {
                let digit: usize;
                if six_chars.is_superset(&digit_signal_sets[4]) {
                    digit = 9;
                } else if six_chars.is_superset(&digit_signal_sets[1]) {
                    digit = 0;
                } else {
                    digit = 6;
                }
                digit_signal_sets[digit] = six_chars;
            });

            // determine 5 signal digits
            signals.filter(|s| s.len() == 5).for_each(|five_chars| {
                let digit: usize;
                if five_chars.is_subset(&digit_signal_sets[6]) {
                    digit = 5;
                } else if five_chars.is_superset(&digit_signal_sets[1]) {
                    digit = 3;
                } else {
                    digit = 2;
                }
                digit_signal_sets[digit] = five_chars;
            });

            // map signal digits across output
            return output.split_whitespace().map(|s| {
                let set = char_set(s);
                for digit in 0..=9 {
                    if set == digit_signal_sets[digit] {
                        return digit;
                    }
                }
                return SENTINEL_DIGIT;
            }).reduce(|a, b| a * 10 + b).expect("value");
        })
        .sum();
}

fn main() {
    const DATA: &str = include_str!("../input.txt");
    println!("part 1: {}", part_1(DATA));
    println!("part 2: {}", part_2(DATA));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_1_DATA: &str = include_str!("../test1.txt");
    const SAMPLE_2_DATA: &str = include_str!("../test2.txt");

    #[test]
    fn test_1() {
        assert_eq!(part_1(SAMPLE_1_DATA), 0);
        assert_eq!(part_1(SAMPLE_2_DATA), 26);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_1_DATA), 5353);
        assert_eq!(part_2(SAMPLE_2_DATA), 61229);
    }
}
