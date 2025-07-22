fn read_i64() -> i64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i64>().unwrap()
}

fn main() {
    let value = read_i64();
    println!("{}", (value + 5) * 3 - 10); // not even gonna bother simplifying
}
