fn read_u64() -> u64 {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response
        .trim_end()
        .to_string()
        .parse()
        .expect("not a number")
}

fn main() {
    let arnar_parked = read_u64();
    let hannes_parked = read_u64();

    println!("{}", arnar_parked + hannes_parked);
}
