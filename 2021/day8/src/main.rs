fn part_1(input: &str) -> usize {
    let lines = input.trim().split('\n');
    let split_lines = lines.map(|line| line.split_once('|').expect("string | string"));

    return split_lines.map(|(_signals, outputs)| outputs.split_whitespace().filter(|val| match val.len() {
        2 | 3 | 4 | 7 => true,
        _ => false
    }).count()).sum();
}

fn part_2(input: &str) -> usize {
    return 0;
}

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("part 1: {}", part_1(DATA));
    // println!("part 2: {}", part_2(DATA));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("../test.txt");

    #[test]
    fn test_1() {
        assert_eq!(part_1(SAMPLE_DATA), 26);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), 0);
    }
}
