fn read_u64() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn main() {
    let song_ticks = read_u64();
    println!("{:.2}", song_ticks as f64 / 4.0);
}
