fn range(range: &str) -> (u32, u32) {
    let (lower, upper) = range.split_once('-').unwrap();
    return (lower.parse().unwrap(), upper.parse().unwrap())
}

fn contains(range: (u32, u32), other: (u32, u32)) -> bool {
    return range.0 <= other.0 && range.1 >= other.1;
}

fn overlap(range: (u32, u32), other: (u32, u32)) -> bool {
    return range.0 <= other.1 && range.1 >= other.0 || other.0 <= range.1 && other.1 >= range.0;
}

fn part_1(input: &str) -> u32 {
    return input.lines().map(|line| -> u32 {
        let (first, second) = line.split_once(',').unwrap();
        let (first_range, second_range) = (range(first), range(second));
        let contained = contains(first_range, second_range) || contains(second_range, first_range);
        return contained as u32;
    }).sum()
}

fn part_2(input: &str) -> u32 {
    return input.lines().map(|line| -> u32 {
        let (first, second) = line.split_once(',').unwrap();
        let (first_range, second_range) = (range(first), range(second));
        return overlap(first_range, second_range) as u32;
    }).sum()
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
        assert_eq!(part_1(SAMPLE_DATA), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), 4);
    }
}
