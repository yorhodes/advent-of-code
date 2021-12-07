fn part_1(input: &str) -> usize {
    let positions: Vec<usize> = input
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap()).collect();

    let mut min_fuel = usize::MAX;
    for p1 in &positions {
        let mut fuel = 0;
        for p2 in &positions {
            let distance = p1.max(p2) - p1.min(p2);
            fuel += distance;
        }
        min_fuel = min_fuel.min(fuel);
    }

    return min_fuel;
}

fn part_2(input: &str) -> usize {
    let positions: Vec<usize> = input
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap()).collect();

    let min_pos = positions.iter().min().expect("min");
    let max_pos = positions.iter().max().expect("max");

    let mut min_fuel = usize::MAX;
    for p1 in *min_pos..=*max_pos {
        let mut fuel = 0;
        for p2 in &positions {
            let distance = p1.max(*p2) - p1.min(*p2);
            let cost = distance * (distance + 1) / 2;
            fuel += cost;
        }
        min_fuel = min_fuel.min(fuel);
    }

    return min_fuel;
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
        assert_eq!(part_1(SAMPLE_DATA), 37);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), 168);
    }
}
