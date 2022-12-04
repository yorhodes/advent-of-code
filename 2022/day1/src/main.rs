fn part_1(input: &str) -> u32 {
    return input
        .split("\n\n")
        .map(|elf| elf.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();
}

fn part_2(input: &str) -> u32 {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    elves.sort();
    elves.reverse();
    return elves[0..3].iter().sum();
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
        assert_eq!(part_1(SAMPLE_DATA), 24000);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), 45000);
    }
}
