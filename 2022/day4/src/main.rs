fn part_1(_input: &str) -> u32 {
    return 0;
}

fn part_2(_input: &str) -> u32 {
    return 0;
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
        assert_eq!(part_1(SAMPLE_DATA), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), 0);
    }
}
