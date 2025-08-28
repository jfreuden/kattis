#![allow(dead_code)]
/// This is a single file that will handle all the "with input <trivial> print <trivial output>" problems
/// The top of the file will be the active space and will remain with a fn main() method.
/// Once complete, the subroutine will be renamed to the problem title.
/// This will allow the top to contain the helper methods, with the main method up top for copying
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
    mclimb();
}

fn mclimb() {
    let rocks: u32 = read_one();
    let liter_cost: u32 = read_one();
    let _: u64 = read_one();

    println!("{}", rocks * liter_cost);
}

fn _13floors() {
    let real_floor: u32 = read_one();
    println!("{}", real_floor + (real_floor >= 13) as u32);
}

fn flatterland() {
    let people: u64 = read_one();
    let abstand: u64 = read_one();
    println!("{}", (people - 1) * abstand);
}

fn hackaholics() {
    let hackathons: u64 = read_one();
    let _: u64 = read_one();
    let avg_prizes: u64 = read_one();

    println!("{}", hackathons * avg_prizes);
}

fn catinabox() {
    let [height, width, length, cat_volume] = read_array::<u64, 4, _>();
    match cat_volume.cmp(&(height * width * length)) {
        std::cmp::Ordering::Equal => println!("COZY"),
        std::cmp::Ordering::Less => println!("SO MUCH SPACE"),
        std::cmp::Ordering::Greater => println!("TOO TIGHT"),
    }
}

fn isithalloween() {
    let input = read_str();
    if input == "OCT 31" || input == "DEC 25" {
        println!("yup");
    } else {
        println!("nope");
    }
}

fn edays() {
    let miles: u32 = read_one();
    let _: u32 = read_one();
    let _: i32 = read_one();

    println!("{}", 2 * miles);
}

fn hissingmicrophone() {
    let input = read_str();

    if input.contains("ss") {
        println!("hiss");
    } else {
        println!("no hiss");
    }
}

trait UpgradedIterator: Iterator {
    fn my_avg(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: std::ops::Add<Output = Self::Item>
        + std::ops::Div<Output = Self::Item>
        + From<u8>
        + PartialEq
        + Clone,
    {
        let zero = <Self::Item as From<u8>>::from(0u8);
        let one = <Self::Item as From<u8>>::from(1u8);

        let (sum, count) = self.fold((zero.clone(), zero.clone()), |(s, c), x| (s + x, c + one.clone()));
        if count == zero { None } else { Some(sum / count) }
    }
}

impl<T: ?Sized> UpgradedIterator for T where T: Iterator {}

fn batterup() {
    let _at_bats: u32 = read_one();
    let bat_results: Vec<i32> = read_vec();

    let slugging_percentage = bat_results
        .iter()
        .filter(|&x| !x.is_negative())
        .map(|&x| x as f32)
        .my_avg()
        .unwrap();

    println!("{}", slugging_percentage);
}

fn filip() {
    let [mut str_a, mut str_b]: [String; 2] = read_array();
    str_a = str_a.chars().rev().collect::<String>();
    str_b = str_b.chars().rev().collect::<String>();
    let nums: Vec<u16> = vec![
        str_a.parse().unwrap(),
        str_b.parse().unwrap()
    ];

    println!("{}", nums.iter().max().unwrap());
}

fn faktor() {
    let [article_count, desired_impact]: [u32; 2] = read_array();

    let bribed_scientists = article_count * (desired_impact - 1) + 1;
    println!("{}", bribed_scientists);
}

fn grassseed() {
    let seed_cost: f32 = read_one();
    let num_lawns: usize = read_one();

    let mut total_lawn_area = 0f32;

    for _ in 0..num_lawns {
        let [length, width] = read_array::<f32, 2, _>();
        total_lawn_area += length * width;
    }

    println!("{}", total_lawn_area * seed_cost);
}

fn pot() {
    let number_of_terms: u64 = read_one();

    let mut terms = Vec::with_capacity(number_of_terms as usize);
    for _ in 0..number_of_terms {
        terms.push(read_one::<u64>());
    }

    let real_terms = terms.iter().map(|x| {
        let div = x / 10;
        let modulo = x % 10;
        div.pow(modulo as u32)
    });

    println!("{}", real_terms.sum::<u64>());
}

fn pet() {
    let gradeslist: Vec<(u8, u8)> = vec![
        (1, read_vec().iter().sum()),
        (2, read_vec().iter().sum()),
        (3, read_vec().iter().sum()),
        (4, read_vec().iter().sum()),
        (5, read_vec().iter().sum()),
    ];

    let best = gradeslist.iter().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap();
    println!("{} {}", best.0, best.1);
}

fn reversebinary() {
    let input: u64 = read_one();
    println!("{}", input.reverse_bits() >> input.leading_zeros());
}

fn cetvrta() {
    let [x1, y1] = read_array::<u16, 2, _>();
    let [x2, y2] = read_array::<u16, 2, _>();
    let [x3, y3] = read_array::<u16, 2, _>();


    let initial_xs = [x1];
    let initial_ys = [y1];

    let fourth_x = if initial_xs.contains(&x2) {
        x3
    } else if initial_xs.contains(&x3) {
        x2
    } else {
        x1
    };

    let fourth_y = if initial_ys.contains(&y2) {
        y3
    } else if initial_ys.contains(&y3) {
        y2
    } else {
        y1
    };

    println!("{} {}", fourth_x, fourth_y);
}

fn sibice() {
    let [num_matches, box_w, box_h] = read_array::<u16, 3, _>();
    let box_diag = (box_h * box_h + box_w * box_w).isqrt();
    for _ in 0..num_matches {
        let match_len: u16 = read_one();
        if match_len > box_diag {
            println!("NE")
        } else {
            println!("DA")
        }
    }
}

fn wakeupcall() {
    let _ = read_str();
    let first = read_vec::<i64>();
    let second = read_vec::<i64>();

    let first_sum: i64 = first.iter().sum();
    let second_sum = second.iter().sum();
    match first_sum.cmp(&second_sum) {
        std::cmp::Ordering::Less => println!("Button 2"),
        std::cmp::Ordering::Greater => println!("Button 1"),
        std::cmp::Ordering::Equal => println!("Oh no"),
    }
}

fn grafaholur() {
    let starting_worker_count: f32 = read_one();
    let starting_hours: f32 = read_one();
    let initial_dug: f32 = read_one();

    let downsized_worker_count: f32 = read_one();
    let downsized_worker_quota: f32 = read_one();

    // Given the previous rate and the new downsized workers and quotas, how long will it take.
    let dig_rate_per_worker: f32 = initial_dug / (starting_hours * starting_worker_count);
    let downsized_dig_time = downsized_worker_quota / (dig_rate_per_worker * downsized_worker_count);
    println!("{}", downsized_dig_time);
}

fn triangelfabriken() {
    let angles: Vec<u16> = vec![
        read_one(),
        read_one(),
        read_one(),
    ];

    let biggest_angle = *angles.iter().max().unwrap();
    if biggest_angle > 90 {
        println!("Trubbig Triangel");
    } else if biggest_angle == 90 {
        println!("Ratvinklig Triangel");
    } else {
        println!("Spetsig Triangel")
    }
}

fn monopol() {
    let _: usize = read_one();
    let sorted_distances: Vec<usize> = read_vec();

    let total_probability: f32 =sorted_distances.iter().map(|&x| {
        let dice_sides = 6;
        let chances_for_x = dice_sides - (dice_sides + 1 - x as i32).abs();
        chances_for_x as f32 / (dice_sides * dice_sides) as f32
    }).sum();

    println!("{}", total_probability);
}

fn hakkari() {
    let [rows, _]: [usize; 2] = read_array();
    let mut answers = Vec::<(usize, usize)>::new();
    for row in 0..rows {
        let line = read_str();
        for (column, symbol) in line.chars().enumerate() {
            if symbol == '*' {
                answers.push((row + 1, column + 1));
            }
        }
    }
    println!("{}", answers.len());
    for (row, column) in answers {
        println!("{} {}", row, column);
    }
}

fn lastfactorialdigit() {
    // The problem is silly and the input is limited to 10. Going with the ultra-naive solution.
    let num_test_cases = read_one::<usize>();
    // Okay I can't help but at least cache the results
    struct FactHelper {
        answers_cache: Vec<Option<usize>>
    }
    impl FactHelper {
        fn new(max: usize) -> FactHelper {
            FactHelper { answers_cache: vec![None; max] }
        }

        fn fact(&mut self, input: usize) -> usize {
            if input == 1 {
                return 1;
            } else if let Some(answer) = self.answers_cache[input - 1] {
                return answer;
            }
            input * self.fact(input - 1)
        }
    }

    let mut fact = FactHelper::new(10);

    for _ in 0..num_test_cases {
        let fact_input = read_one::<usize>();
        println!("{}", fact.fact(fact_input) % 10);
    }
}

fn autori() {
    let name_string = read_str();
    println!("{}", name_string.split('-').map(|x| *x.as_bytes().first().unwrap() as char).collect::<String>());
}

fn r2() {
    let [r1, mean] = read_array::<i32, 2, _>();

    println!("{}", 2i32 * mean - r1);
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
    let bags = read_vec::<i64>();
    let (bag_position, _bag) = bags
        .iter()
        .enumerate()
        .find(|&(_, &bag)| bag == target_bag)
        .unwrap();
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
        std::cmp::Ordering::Equal => println!("Lagom"),
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
    let avg = (total_cells - filled_cells) as f32 / total_cells as f32;
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
    println!(
        "{}",
        read_str().chars().filter(|&c| c != ' ').collect::<String>()
    );
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
    let num_vowels = input
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c))
        .count();
    let num_y = input.chars().filter(|c| c == &'y').count();
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
    println!(
        "{}",
        input
            .chars()
            .filter(|c| ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(c))
            .count()
    );
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
    let coefficients = coeffs;
    let discriminant = coefficients[1] * coefficients[1] - 4 * coefficients[0] * coefficients[2];
    // b^2 - 4a*c
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

    if players.is_sorted() {
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
