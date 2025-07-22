fn read_u64() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn main() {
    let hannes_to_arnar = read_u64();
    let arnar_to_cinema = read_u64();
    let time_of_film = read_u64();

    println!("{}", time_of_film - (hannes_to_arnar + arnar_to_cinema));
}
