/// This is a single file which will handle all the "with input <trivial> print <trivial output>" problems
/// The top of the file will be the active space, and will remain with a fn main() method.
/// Once complete, the subroutine will be renamed to the problem title.
/// This will allow the top to contain the helper methods, with the main method up top for copying
/// The used read may be place

fn read_str() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

fn read_one<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

fn try_read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> Result<[T; K], E>
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    read_vec::<T>().try_into()
}

fn read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> [T; K]
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    try_read_array().unwrap()
}

fn main() {
    echoechoecho()
}

fn echoechoecho() {
    println!("{}", [read_str().as_str()].repeat(3).join(" "));
}

fn addtwonumbers() {
    let [a, b] = read_array::<u64, 2, _>();
    println!("{}", a + b);
}

fn jackolanternjuxtaposition() {
    let [eyes, noses, mouths] = read_array::<u64, 3, _>();
    println!("{}", eyes * noses * mouths);
}

fn qaly() {
    let count = read_one::<u64>();
    let mut quality_sum: f32 = 0.0;
    for _ in 0..count {
        let [quality, years] = read_array::<f32, 2, _>();
        quality_sum += quality * years;
    }
    println!("{:.3}", quality_sum);
}

fn quadrant() {
    let x: i64 = read_one();
    let y: i64 = read_one();
    match (x.is_positive(), y.is_positive()) {
        (true, true) => println!("1"),
        (false, true) => println!("2"),
        (false, false) => println!("3"),
        (true, false) => println!("4"),
    }
}

fn twostones() {
    let stones: u64 = read_one();
    let winner: String = if stones.is_multiple_of(2) {
        "Bob".to_string()
    } else {
        "Alice".to_string()
    };
    println!("{}", winner);
}

fn carrots() {
    let [contestants, solves]: [u64; 2] = read_array();
    for _ in 0..contestants {
        let _ = read_str();
    }
    println!("{}", solves);
}

fn get_root_count(coeffs: Vec<i64>) -> usize {
    let mut coefficients = coeffs;
    let discriminant = coefficients[1] * coefficients[1] - 4 * coefficients[0] * coefficients[2]; // b^2 - 4ac
    // This reminder of grade-school math is brought to you by: https://amsi.org.au/ESA_Senior_Years/SeniorTopic2/2a/2a_2content_5.html
    // Δ>0
    //  tells us the equation has two distinct real roots
    // Δ=0
    //  tells us the equation has one (repeated) real root
    // Δ<0
    //  tells us the equation has no real roots.

    if discriminant > 0 {
        2
    } else if discriminant == 0 {
        1
    } else {
        0
    }
}

#[test]
fn test_get_root_count() {
    assert_eq!(get_root_count(vec![-5, 1, 1]), 2);
    assert_eq!(get_root_count(vec![1, 1, 1]), 0);
    assert_eq!(get_root_count(vec![2, 4, 2]), 1);
    assert_eq!(get_root_count(vec![-5, 1, 1]), 2);
    assert_eq!(get_root_count(vec![-5, 1, 1]), 2);
    assert_eq!(get_root_count(vec![-5, 1, 1]), 2);

}

fn dfyrirdreki() {
    let a: i64 = read_one();
    let b: i64 = read_one();
    let c: i64 = read_one();
    let coefficients = vec![a, b, c];
    println!("{}", get_root_count(coefficients));
}

fn bestagjofin() {
    let guests: u64 = read_one();
    let mut best_guest = String::new();
    let mut best_gift_score: u64 = 0;
    for _ in 0..guests {
        let [name, gift_score_str] = read_array::<String, 2, _>();
        let gift_score: u64 = gift_score_str.parse().unwrap();
        if gift_score > best_gift_score {
            best_gift_score = gift_score;
            best_guest = name;
        }
    }
    println!("{}", best_guest);
}

fn parking2() {
    let test_cases: u64 = read_one();
    let mut distances: Vec<u64> = vec![];
    for _ in 0..test_cases {
        let _visit_count = read_one::<u64>();
        let mut positions: Vec<u64> = read_vec();
        positions.sort();
        let max_width = positions.last().unwrap() - positions.first().unwrap();
        distances.push(2 * max_width);
    }

    for distance in distances {
        println!("{}", distance);
    }
}

fn magictrick() {
    let configuration = read_str();
    let mut charset = std::collections::HashSet::<char>::new();
    let mut unique = true;
    for char in configuration.chars() {
        if charset.insert(char) != true {
            unique = false;
        }
    }
    println!("{}", unique as u8);
}

fn bookingaroom() {
    let [total_rooms, booked_rooms]: [u64; 2] = read_array();
    let mut all_rooms: std::collections::BTreeSet<u64> = (1..total_rooms + 1).collect();
    for _ in 0..booked_rooms {
        all_rooms.remove(&read_one());
    }
    println!(
        "{}",
        all_rooms
            .first()
            .map(|n| n.to_string())
            .unwrap_or("too late".to_string())
    );
}

fn lineup() {
    let count: u64 = read_one();
    let mut players: Vec<String> = vec![];
    for _ in 0..count {
        players.push(read_str());
    }

    if (players.is_sorted()) {
        println!("INCREASING")
    } else if players.iter().rev().is_sorted() {
        println!("DECREASING")
    } else {
        println!("NEITHER")
    }
}

fn fyi() {
    println!("{}", read_str().starts_with("555") as u8);
}

fn tarifa() {
    let data_cap: u64 = read_one();
    let months: u64 = read_one();

    let mut used_data = 0;
    for _ in 0..months {
        used_data += read_one::<u64>();
    }
    println!("{}", data_cap * (months + 1) - used_data);
}

fn coffeecupcombo() {
    let _count: u64 = read_one();
    let mut machinestring: Vec<char> = read_str().chars().collect::<Vec<char>>();
    let mut win_string = vec!['0'; 2];
    win_string.append(&mut machinestring);

    let classes_with_coffee = win_string
        .windows(3)
        .filter(|window| window.contains(&'1'))
        .count();
    println!("{}", classes_with_coffee);
}

fn jumbojavelin() {
    let count: u64 = read_one();
    let mut javelins: Vec<u64> = vec![];
    for _ in 0..count {
        let length: u64 = read_one();
        javelins.push(length);
    }
    let fused_size = javelins.iter().sum::<u64>() - javelins.iter().count() as u64 + 1;
    println!("{}", fused_size);
}

fn cold() {
    let _count: u64 = read_one();
    let numbers: Vec<i64> = read_vec();
    println!("{}", numbers.iter().filter(|x| x.is_negative()).count());
}

fn oddities() {
    let count: u64 = read_one();
    let mut numbers: Vec<i64> = vec![];
    for _ in 0..count {
        let number: i64 = read_one();
        numbers.push(number);
    }

    for number in numbers {
        match (number % 2).abs() {
            0 => println!("{} is even", number),
            1 => println!("{} is odd", number),
            _ => panic!("This shouldn't happen"),
        }
    }
}

fn timeloop() {
    let count: u64 = read_one();
    for i in 0..count {
        println!("{} Abracadabra", i + 1);
    }
}

fn bijele() {
    let piece_counts: [i64; 6] = read_array();
    const CORRECT_COUNTS: [i64; 6] = [1, 1, 2, 2, 2, 8];

    let adjustments = std::iter::zip(piece_counts.iter(), CORRECT_COUNTS.iter())
        .map(move |(&mine, &usual)| (usual - mine).to_string())
        .collect::<Vec<String>>();

    println!("{}", adjustments.join(" "));
}

fn goggi() {
    let tokens = read_vec::<String>();
    let ints = tokens
        .iter()
        .map(|s| s.parse::<u64>().unwrap_or_default())
        .collect::<Vec<u64>>();
    let [a, _, b] = ints.try_into().unwrap();

    if a > b {
        println!(">");
    } else if a < b {
        println!("<");
    } else {
        println!("Goggi svangur!")
    }
}

fn blandadbest() {
    let lines: u64 = read_one();
    if lines > 1 {
        println!("blandad best")
    } else {
        println!("{}", read_str())
    }
}

fn decimaldeletion() {
    println!("{}", read_one::<f64>().round());
}

fn spritt() {
    let [classrooms, bottles] = read_vec::<u64>().try_into().unwrap();

    let mut needs = 0;
    for _ in 0..classrooms {
        needs += read_one::<u64>();
    }

    if needs > bottles {
        println!("Neibb")
    } else {
        println!("Jebb")
    }
}

fn ameriskur() {
    const FOOTBALL_FIELD_IN_KM: f64 = 0.09144;
    println!("{}", read_one::<u64>() as f64 * FOOTBALL_FIELD_IN_KM);
}

fn oddecho() {
    let linecount: u64 = read_one();

    let mut words: Vec<String> = vec![];

    for i in 0..linecount {
        let word = read_str();
        if i % 2 == 0 {
            words.push(word);
        }
    }

    for word in words {
        println!("{}", word);
    }
}

fn hipphipphurra() {
    let name = read_str();
    let age: u64 = read_one();
    for _ in 0..age {
        println!("Hipp hipp hurra, {}!", name);
    }
}

fn bladra2() {
    let [v0, acc, time] = read_vec::<i64>()
        .iter()
        .map(|&x| x as f64)
        .collect::<Vec<f64>>()
        .try_into()
        .unwrap();

    println!("{}", v0 * time + 0.5 * acc * time * time);
}

fn dagatal() {
    let month: u64 = read_one();
    // Sad.
    let days_this_month = match month {
        1 => 31,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => panic!(),
    };
    println!("{}", days_this_month);
}

fn skak() {
    let [rook_x, rook_y] = read_vec::<u64>().try_into().unwrap();
    let [pawn_x, pawn_y] = read_vec::<u64>().try_into().unwrap();

    println!(
        "{}",
        2 - (rook_x == pawn_x) as u64 - (rook_y == pawn_y) as u64
    );
}

fn bergmal() {
    println!("{}", read_str());
}
