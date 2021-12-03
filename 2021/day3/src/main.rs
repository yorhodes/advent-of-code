fn main() {
    let num_bits = 12;

    let mut bit_counts = vec![0; num_bits];

    include_str!("../input.txt").lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, bit)| {
            bit_counts[i] += if bit == '1' { 1 } else { -1 };
        });
    });

    let gamma_rate = u16::from_str_radix(
        &bit_counts
            .iter()
            .map(|count| (if *count > 0 { '1' } else { '0' }))
            .collect::<String>(),
        2,
    ).expect("binary string");

    let epsilon_rate = !(gamma_rate << 4) >> 4;

    println!("gamma * epsilon {}", gamma_rate as usize * epsilon_rate as usize);

    
}
