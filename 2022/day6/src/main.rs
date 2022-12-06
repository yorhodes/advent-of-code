use std::collections::HashSet;

fn part_1(input: &str) -> u32 {
    let chars = input.chars();
    let mut one = chars.clone();
    let mut two = chars.clone().skip(1);
    let mut three = chars.clone().skip(2);
    let mut four = chars.clone().skip(3);
    let mut i = 0;
    while HashSet::from([one.next(), two.next(), three.next(), four.next()]).len() != 4 {
        i += 1;
    }
    return i + 4;
}

fn part_2(input: &str) -> u32 {
    let chars = input.chars();
    let mut one = chars.clone();
    let mut two = chars.clone().skip(1);
    let mut three = chars.clone().skip(2);
    let mut four = chars.clone().skip(3);
    let mut five = chars.clone().skip(4);
    let mut six = chars.clone().skip(5);
    let mut seven = chars.clone().skip(6);
    let mut eight = chars.clone().skip(7);
    let mut nine = chars.clone().skip(8);
    let mut ten = chars.clone().skip(9);
    let mut eleven = chars.clone().skip(10);
    let mut twelve = chars.clone().skip(11);
    let mut thirteen = chars.clone().skip(12);
    let mut fourteen = chars.clone().skip(13);

    let mut i = 0;
    while HashSet::from([one.next(), two.next(), three.next(), four.next(), five.next(), six.next(), seven.next(), eight.next(), nine.next(), ten.next(), eleven.next(), twelve.next(), thirteen.next(), fourteen.next()]).len() != 14 {
        i += 1;
    }
    return i + 14;
}

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("part 1: {}", part_1(DATA));
    println!("part 2: {}", part_2(DATA));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
