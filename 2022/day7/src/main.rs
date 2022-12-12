use std::collections::HashMap;

fn part_1(input: &str) -> u32 {
    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();

    let mut absolute_path = String::from("~");

    for section in input.split('$').skip(2) {
        let mut lines = section.lines();
        let command_line = lines.next().unwrap();
        if command_line == " ls" {
            for entry in lines {
                let (left, right) = entry.split_once(' ').unwrap();

                let path = absolute_path.clone();
                let child_path = format!("{}/{}", path, right);
                if left == "dir" {
                    children.entry(path).or_insert(vec![]).push(child_path);
                } else {
                    let size: u32 = left.parse().unwrap();
                    sizes.entry(path).and_modify(|f|*f += size).or_insert(size);
                }
            }
        } else { // " cd"
            let dir = command_line.split(' ').last().unwrap();
            let path = &absolute_path;
            if dir == ".." {
                absolute_path = path.rsplit_once('/').unwrap().0.to_string();
            } else {
                absolute_path = format!("{}/{}", path, dir);
            }
        }
    }

    let mut total_sizes = sizes.clone();
    let mut keys = sizes.keys().collect::<Vec<_>>();

    // sort by number of slashes
    keys.sort_by(|a, b| a.matches('/').count().cmp(&b.matches('/').count()));
    keys.reverse();
    
    for k in keys {
        if k == "~" {
            break;
        }
        let child_size = *total_sizes.get(k).unwrap();
        let p = k.rsplit_once('/').unwrap().0.to_string();
        total_sizes.entry(p).and_modify(|f|*f += child_size);
    }

    return total_sizes.values().filter(|v| **v <= 100000).map(|v| *v).sum();
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
