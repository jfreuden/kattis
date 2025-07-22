fn read_u64() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn main() {
    let initial = read_u64();
    let opened = read_u64();
    println!("{}", initial - opened);
}
