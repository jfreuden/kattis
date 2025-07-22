fn read_line() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}
fn main() {
    println!("{}{}", read_line(), read_line());
}
