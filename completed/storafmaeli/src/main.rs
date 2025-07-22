fn prompt() -> u64 {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string().parse().unwrap()
}

fn main() {
    let age = prompt();
    let output = match age % 10 {
        0 => "Jebb",
        _ => "Neibb",
    };
    println!("{}", output);
}
