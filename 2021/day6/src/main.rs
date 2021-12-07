fn part_1(input: &str, days: usize) -> usize {
    let max_timer = 8;
    let mut age_counts = vec![0; max_timer+1];

    // construct counts of each timer
    input
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .for_each(|age| age_counts[age] += 1);

    // run simulation
    for _ in 1..=days {
        let num_zeroes = age_counts[0];
        for i in 0..max_timer {
            age_counts[i] = age_counts[i+1];
        }
        age_counts[6] += num_zeroes; // each day a 0 becomes a 6
        age_counts[max_timer] = num_zeroes; // each day a 0 adds a new 8
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
