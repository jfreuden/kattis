fn read_vec<T: std::str::FromStr, B: std::io::BufRead>(bufreader: &mut B) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    bufreader.read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}



fn main() {
    // TODO: find out if I can add `read_vec` to the methods of "stdin"
    let stdin = std::io::BufReader::with_capacity(64, std::io::stdin().lock());
    let stdout = std::io::BufWriter::with_capacity(64, std::io::stdout().lock());



    println!("Hello, world!");
}

#[cfg(test)]
mod prinsesse_tests;