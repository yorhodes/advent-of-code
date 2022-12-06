fn build_columns(columns: &str) -> Vec<Vec<char>> {
    let num_columns = (columns.lines().next().unwrap().len() + 1) / 4;
    
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_columns];
    
    for column in columns.lines().rev() {
        for i in  0..num_columns {
            let char_idx = i * 4 + 1;
            let char = column.chars().nth(char_idx).unwrap();
            if char != ' ' && (char as u32) > 52 {
                stacks[i].push(char);
            }
        }
    }
    return stacks;
}

fn parse_move(m: &str) -> (usize, usize, usize) {
    let mut sections = m.split(' ').skip(1).step_by(2);
    let count = sections.next().unwrap().parse::<usize>().unwrap();
    let from = sections.next().unwrap().parse::<usize>().unwrap() - 1;
    let to = sections.next().unwrap().parse::<usize>().unwrap() - 1;
    return (count, from, to);
}

fn part_1(input: &str) -> String {
    let (columns, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = build_columns(columns);

    for move_str in moves.lines() {
        let (count, from, to) = parse_move(move_str);
        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    return stacks.iter().map(|stack| stack.last().unwrap()).collect();

}

fn part_2(input: &str) -> String {
    let (columns, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = build_columns(columns);

    for move_str in moves.lines() {
        let (count, from, to) = parse_move(move_str);
        let len = stacks[from].len();
        let slice: Vec<char> = stacks[from].drain(len - count..len).collect();
        stacks[to].extend(slice);
    }

    return stacks.iter().map(|stack| stack.last().unwrap()).collect();
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
        assert_eq!(part_1(SAMPLE_DATA), "CMZ");
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(SAMPLE_DATA), "MCD");
    }
}
