use std::cmp::max;

fn read_line() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

fn main() {
    let unnar_quote = read_line();
    // This is a slightly more complex variant that allows for Unnar to say "uuunuuuuuuuuuu"
    let uncertainties: (u64, u64) = unnar_quote.chars().fold((0, 0), |(longest, current), c| {
        if c == 'u' {
            let new_longest = max(longest, current + 1);
            (new_longest, current + 1)
        } else {
            // Reset the current streak
            (longest, 0)
        }
    });

    println!("{}", uncertainties.0);
}
