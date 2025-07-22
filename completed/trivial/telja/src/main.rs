fn read_i64() -> i64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<i64>().unwrap()
}

fn main() {
    let value = read_i64();
    for i in 1..value + 1  {
        println!("{}", i);
    }
}
