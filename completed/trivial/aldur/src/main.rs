fn read_u64() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn main() {
    let count = read_u64();
    let mut min_age = u64::max_value();

    for _ in 0..count {
        let input = read_u64();
        if input < min_age {
            min_age = input;
        }
    }
    println!("{}", min_age);
}
