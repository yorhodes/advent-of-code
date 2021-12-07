fn part_1(input: &str, days: usize) -> usize {
    let mut age_counts = [0; 9];

    // construct counts of each timer
    input
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .for_each(|age| age_counts[age] += 1);

    // run simulation
    for _ in 1..=days {
        age_counts.rotate_left(1); // 0 wraps to 8
        age_counts[6] += age_counts[8]; // 6 increment by prior 0 count
    }

    // sum counts of each timer
    return age_counts.iter().sum();
}

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("part 1: {}", part_1(DATA, 80));
    println!("part 2: {}", part_1(DATA, 256));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("../test.txt");

    #[test]
    fn test_1() {
        assert_eq!(part_1(SAMPLE_DATA, 18), 26);
        assert_eq!(part_1(SAMPLE_DATA, 80), 5934);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_1(SAMPLE_DATA, 256), 26984457539);
    }
}
