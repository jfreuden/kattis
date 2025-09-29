fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: u64 = line.trim().parse().unwrap();

    let too_many_digits_string = modular_factorial_slim(n).to_string();
    println!(
        "{}",
        too_many_digits_string
            .chars()
            .rev()
            .take(3)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>()
    );
}

fn modular_factorial_slim(n: u64) -> u64 {
    let mut modular_factorial: u64 = 1;
    for initial_i in 1..=n {
        let mut i = initial_i;
        while i % 10 == 0 {
            i /= 10;
        }
        modular_factorial = modular_factorial.wrapping_mul(i);
        if modular_factorial == 0 {
            panic!("wrapped badly at {}", i);
        }
        while modular_factorial % 10 == 0 {
            modular_factorial /= 10;
        }
        modular_factorial %= 1_000_000_000_000
    }

    modular_factorial
}
