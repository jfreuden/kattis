fn read_line() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}
fn main() {
    let dna = read_line();

    match dna.contains("COV") {
        true => {
            println!("Veikur!")
        }
        false => {
            println!("Ekki veikur!");
        },
    };
}
