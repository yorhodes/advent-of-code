use core::panic;
use std::{collections::HashSet};

fn priority(c: char) -> u32 {
    let i = c as u32;
    if i > 96 { // a-z
        return i - 96;
    } else { // A-Z
        return i - 38;
    }
}

fn part_1(input: &str) -> u32 {
    return input.lines().map(|rucksack| -> u32 {
        let (first_half, second_half) = rucksack.split_at(rucksack.len()/2);
        let mut first_compartment: HashSet<char> = HashSet::new();
        for c in first_half.chars() {
            first_compartment.insert(c);
        }
        for c in second_half.chars() {
            if first_compartment.contains(&c) {
                return priority(c);
            }
        }
        panic!("invalid rucksack");
    }).sum();
}

fn part_2(input: &str) -> u32 {
    let mut group: HashSet<char> = HashSet::new();
    let mut group_index = 0;
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut rucksack: HashSet<char> = HashSet::new();
        for c in line.chars() {
            rucksack.insert(c);
        }
        if group_index == 0 {
            group = rucksack;
        } else {
            group = group.intersection(&rucksack).map(|&x| x.clone()).collect();
        }
        if group_index == 2 {
            sum += priority(group.drain().last().unwrap());
        }
        group_index = (group_index + 1) % 3;
    }
    return sum;
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
        assert_eq!(part_1(SAMPLE_DATA), 157);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), 70);
    }
}
