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
    vedurheidar()
}

fn vedurheidar() {
    let wind_speed: u64 = read_one();
    let number_roads: u64 = read_one();
    let mut road_limits: Vec<(String, u64)> = Vec::new();
    for _ in 0..number_roads {
        let [name, limit_str] = read_array::<String, 2, _>();
        road_limits.push((name, limit_str.parse().unwrap()))
    }
    for (name, limit) in road_limits {
        match wind_speed.cmp(&limit) {
            std::cmp::Ordering::Greater => println!("{} lokud", name),
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => println!("{} opin", name),
        }
    }
}

fn barcelona() {
    let [_bag_count, target_bag]: [i64; 2] = read_array();
    let bags= read_vec::<i64>();
    let (bag_position, _bag) = bags.iter().enumerate().find(|&(_, &bag)| bag == target_bag).unwrap();
    match bag_position {
        0 => println!("fyrst"),
        1 => println!("naestfyrst"),
        _ => println!("{} fyrst", bag_position + 1),
    }
}

fn heysata() {
    let _characters: u64 = read_one();
    let char: char = read_one();
    let haystack = read_str();
    match haystack.contains(char) {
        true => println!("Unnar fann hana!"),
        false => println!("Unnar fann hana ekki!"),
    }
}

fn pobudget() {
    let lineitem_count = read_one::<usize>();
    let mut account_value: i64 = 0;
    for _ in 0..lineitem_count {
        let _item_name = read_str();
        account_value += read_one::<i64>();
    }

    match account_value.cmp(&0) {
        std::cmp::Ordering::Less => println!("Nekad"),
        std::cmp::Ordering::Greater => println!("Usch, vinst"),
        std::cmp::Ordering::Equal => println!("Lagom")
    }
}

fn umferd() {
    let cells_per_lane: u64 = read_one();
    let lanes: u64 = read_one();
    let mut filled_cells: u64 = 0;
    for _ in 0..lanes {
        let road = read_str();
        let cars = road.chars().filter(|&c| c == '#').count() as u64;
        filled_cells += cars;
    }
    let total_cells: u64 = lanes * cells_per_lane;
    let avg = (total_cells - filled_cells) as f32 / (total_cells) as f32;
    println!("{}", avg);
}

fn fjoldibokstafa() {
    let input = read_str();
    println!("{}", input.chars().filter(|&c| c.is_alphabetic()).count());
}

fn kikiboba() {
    let input = read_str();
    let count_b = input.chars().filter(|&c| c == 'b').count();
    let count_k = input.chars().filter(|&c| c == 'k').count();
    if count_b == 0 && count_k == 0 {
        println!("none");
    } else {
        match count_b.cmp(&count_k) {
            std::cmp::Ordering::Greater => println!("boba"),
            std::cmp::Ordering::Equal => println!("boki"),
            std::cmp::Ordering::Less => println!("kiki"),
        }
    }
}

fn leynithjonusta() {
    println!("{}", read_str().chars().filter(|&c| c != ' ').collect::<String>());
}

fn ofugsnuid() {
    let count = read_one::<u64>();
    let mut numbers = vec![];
    for _ in 0..count {
        numbers.push(read_one::<u64>());
    }
    numbers.reverse();
    for number in numbers {
        println!("{}", number);
    }
}

fn aterriblefortress() {
    let count = read_one::<u64>();
    let mut sum: u64 = 0;
    for _ in 0..count {
        sum += read_one::<u64>();
    }
    println!("{}", sum);
}

fn isyavowel() {
    let input = read_str();
    let num_vowels = input.chars().filter(|c| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c)).count();
    let num_y = input.chars().filter(|c|c == &'y').count();
    println!("{} {}", num_vowels, num_vowels + num_y);
}

fn addingtrouble() {
    let [a, b, c] = read_array::<i64, 3, _>();
    match a + b == c {
        true => println!("correct!"),
        false => println!("wrong!"),
    }
}

fn whichisgreater() {
    let [a, b] = read_array::<u64, 2, _>();
    println!("{}", a.gt(&b) as u8)
}

fn countthevowels() {
    let input = read_str();
    println!("{}", input.chars().filter(|c| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c)).count());
}

fn twosum() {
    addtwonumbers()
}

fn triarea() {
    let [height, base] = read_array::<u64, 2, _>().try_into().unwrap();
    println!("{}", base as f32 * height as f32 / 2.0);
}

fn nsum() {
    let _count: usize = read_one();
    println!("{}", read_vec().iter().sum::<u64>())
}

fn digitswap() {
    println!("{}", read_str().chars().rev().collect::<String>());
}

fn sorttwonumbers() {
    let mut numbers: [u64; 2] = read_array();
    numbers.sort();
    println!("{}", numbers.map(|n| n.to_string()).join(" "));
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
