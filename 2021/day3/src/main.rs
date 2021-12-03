fn main() {
    let num_bits = 12;

    let mut bit_counts = vec![0; num_bits];

    include_str!("../input.txt").lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, bit)| {
            bit_counts[i] += if bit == '1' { 1 } else { -1 };
        });
    });

    let rate = |invert: bool| {
        return usize::from_str_radix(
            &bit_counts
                .iter()
                .map(|count| (if (*count > 0) ^ invert { '1' } else { '0' }))
                .collect::<String>(),
            2,
        )
        .expect("binary string");
    };

    let gamma_rate = rate(false);
    let epsilon_rate = rate(true);

    println!("{}", gamma_rate * epsilon_rate);
}
