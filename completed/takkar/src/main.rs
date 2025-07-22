use std::cmp::Ordering;

fn prompt() -> u64 {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string().parse().unwrap()
}


fn main() {
    let trump = prompt();
    let un = prompt();
    let output = match trump.cmp(&un) {
        Ordering::Less => { "FAKE NEWS!" }
        Ordering::Equal => { "WORLD WAR 3!" }
        Ordering::Greater => { "MAGA!" }
    };
    println!("{}", output);
}
