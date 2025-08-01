fn read_u64() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn main() {
    let mut map = std::collections::HashMap::new();

    map.insert("Monnei", read_u64());
    map.insert("Fjee", read_u64());
    map.insert("Dolladollabilljoll", read_u64());

    let (output, _fee) = map.iter().min_by_key(|&(_key, value)| value).unwrap();
    println!("{}", output);
}
