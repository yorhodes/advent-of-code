use std::io::{self, Read};
use std::fs::File;

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let whole_file = filename_to_string("input.txt").unwrap();
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    whole_file.lines().for_each(|line| {
        let (direction_str, amount_str) = line.split_once(' ').unwrap();
        let amount = amount_str.parse::<u32>().unwrap();
        // part 1:
        // match direction_str {
        //     "forward" => { horizontal_position += amount; },
        //     "up" => { depth -= amount; },
        //     "down" => { depth += amount; },
        //     _ => {}
        // }

        match direction_str {
            "forward" => { 
                horizontal_position += amount;
                depth += aim * amount;
            },
            "up" => { aim -= amount; },
            "down" => { aim += amount; },
            _ => {}
        }
    });
    println!("result {}", horizontal_position * depth);
}
