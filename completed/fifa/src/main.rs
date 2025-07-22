fn read_u64() -> u64 {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string().parse().expect("not a number")
}

fn main() {
    let number_improvements = read_u64();
    let improvements_per_year = read_u64();

    const START_YEAR: u64 = 2022;

    println!("{}", START_YEAR + number_improvements / improvements_per_year);
}
