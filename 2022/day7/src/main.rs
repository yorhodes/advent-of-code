use std::collections::HashMap;

fn part_1(input: &str) -> u32 {
    let mut sizes: HashMap<&str, u32> = HashMap::new();
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut parent: HashMap<&str, &str> = HashMap::new();

    let mut curr = "/";
    let mut absolute_path = curr.to_string();

    for section in input.split('$').skip(2) {
        let mut lines = section.lines();
        let command_line = lines.next().unwrap();
        if command_line == " ls" {
            for entry in lines {
                let (left, right) = entry.split_once(' ').unwrap();

                if left == "dir" {
                    children.entry(curr).or_insert(vec![]).push(right);
                    parent.insert(right, curr);
                } else {
                    let size: u32 = left.parse().unwrap();
                    sizes.entry(curr).and_modify(|f|*f += size).or_insert(size);
                }
            }
        } else { // " cd"
            println!("{}", command_line);
            let dir = command_line.split(' ').last().unwrap();
            if dir == ".." {
                curr = parent.get(curr).unwrap();
                absolute_path = absolute_path[0..absolute_path.len() - curr.len()].to_string();
            } else {
                curr = dir;
                absolute_path = format!("{}{}", absolute_path, curr);
                sizes.insert(curr, 0);
            }
            println!("{}", absolute_path);
        }
    }


    let mut total_sizes = sizes.clone();
    let mut keys = parent.keys().collect::<Vec<_>>();
    keys.sort();
    keys.reverse();
    for k in keys {
        let p = parent.get(k).unwrap();
        let child_size = total_sizes.get(k).unwrap().clone();
        total_sizes.entry(p).and_modify(|f|*f += child_size);
    }

    let mut values = total_sizes.values().filter(|v| **v <= 100000).collect::<Vec<_>>();
    values.sort();
    values.reverse();
    
    return values.iter().take(2).map(|v| **v).sum();
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

    const TEST_DATA: &str = include_str!("../example.txt");

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_DATA), 95437);
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_DATA), 0);
    }
}
