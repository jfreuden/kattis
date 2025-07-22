use std::io::Write;

fn read_u64() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn read_line() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

fn main() {
    let word = read_line();
    let duplicates = read_u64();
    for _ in 0..duplicates {
        print!("{}", word);
    }
    std::io::stdout().flush().unwrap();
}
