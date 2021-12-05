fn main() {
    let binary_lines: Vec<&str> = include_str!("../test.txt").lines().collect();
    let num_bits = binary_lines[0].len();
    let bit_counts_fn = |lines: &Vec<&str>, indices: Vec<usize>| {
        let mut bit_counts = vec![0; num_bits];
        lines.iter().for_each(|line| {
            let bits = line.as_bytes();
            indices.iter().for_each(|i| {
                bit_counts[*i] += if bits[*i] == '1' as u8 { 1 } else { -1 };
            });
        });
        return bit_counts;
    };

    let mut gamma = 0;
    let mut epsilon = 0;

    bit_counts_fn(&binary_lines, (0..num_bits).collect())
        .iter()
        .for_each(|count| {
            gamma <<= 1;
            epsilon <<= 1;
            if *count > 0 {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        });

    println!("gamma * epsilon {}", gamma * epsilon);

    let retain_fn = |candidates: &mut Vec<&str>, invert: bool| {
        for i in 0..num_bits {
            if candidates.len() > 1 {
                let counts = bit_counts_fn(&candidates, vec![i, 1]);
                let label = if (counts[i] >= 0) ^ invert { 1 } else { 0 };
                candidates.retain(|candidate| candidate.as_bytes()[i] - '0' as u8 == label);
            } else {
                break;
            }
        }
    };

    let mut oxygen_candidates = binary_lines.to_vec();
    retain_fn(&mut oxygen_candidates, false);
    let oxygen_rate = usize::from_str_radix(oxygen_candidates[0], 2).expect("binary string");

    let mut c02_candidates = binary_lines.to_vec();
    retain_fn(&mut c02_candidates, true);
    let c02_rate = usize::from_str_radix(c02_candidates[0], 2).expect("binary string");

    println!("oxygen * c02 {}", oxygen_rate as usize * c02_rate as usize);
}
