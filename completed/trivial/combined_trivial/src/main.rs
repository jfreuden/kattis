#![allow(dead_code)]
/// This is a single file that will handle all the "with input <trivial> print <trivial output>" problems
/// The top of the file will be the active space and will remain with a fn main() method.
/// Once complete, the subroutine will be renamed to the problem title.
/// This will allow the top to contain the helper methods, with the main method up top for copying

#[allow(unused)]
macro_rules! kattis_struct {
    ($name:ident { $($field_name:ident : $field_type:ty),* }) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name {
            $($field_name : $field_type),*
        }
        impl std::str::FromStr for $name {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut iter = s.split(' ');
                Ok($name {
                    $(
                        $field_name: iter.next().unwrap().parse::<$field_type>().map_err(|_| "parse error")?
                    ),*
                })
            }
        }
    };
    }

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

fn read_array<T: std::str::FromStr + std::fmt::Debug, const K: usize>() -> [T; K]
where
    T::Err: std::fmt::Debug,
{
    read_vec::<T>().try_into().unwrap()
}

fn main() {
    quiteaproblem();
}

fn quiteaproblem() {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    for line in buffer.split_terminator('\n') {
        let clean = line.trim_ascii().to_ascii_lowercase();
        if clean.contains("problem") {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn simon() {
    let count = read_one();

    for _ in 0..count {
        let input = read_str();

        if let Some((_, command)) = input.split_once("simon says ") {
            println!("{command}");
        } else {
            println!();
        }
    }
}

fn quickbrownfox() {
    let mut charmap = std::collections::HashMap::<char, bool>::new();

    let count = read_one();

    for _ in 0..count {
        let input = read_str();
        charmap.clear();
        // an inefficient approach is okay, given the input sized.
        for ch in input.chars().filter(|character| character.is_alphabetic()) {
            charmap.insert(ch.to_ascii_lowercase(), true);
        }

        let mut missing = Vec::new();
        for candidate in 'a'..='z' {
            if !charmap.contains_key(&candidate) {
                missing.push(candidate);
            }
        }

        if missing.is_empty() {
            println!("pangram");
        } else {
            print!("missing ");
            for missing_char in missing {
                print!("{}", missing_char);
            }
            println!();
        }
    }
}

fn rectanglearea() {
    let [x1, y1, x2, y2] = read_array::<f32, 4>();
    println!("{}", ((y2 - y1) * (x2 - x1)).abs());
}

fn counting() {
    let input: u64 = read_one();
    for i in 1..=12 {
        println!("{}", input * i);
    }
}

fn popcount() {
    let binary = u128::from_str_radix(read_str().as_str(), 2).unwrap();
    println!("{}", binary.count_ones());
}

fn findingana() {
    let input = read_str();
    let starting_place = input.find('a').unwrap();
    println!("{}", &input[starting_place..]);
}

fn brokencalculator() {
    let opcount = read_str().parse::<usize>().unwrap();

    let mut result = 1;
    for _ in 0..opcount {
        let opstr = read_str();
        let mut opsplit = opstr.split(' ');
        let a = opsplit.next().unwrap().parse::<i64>().unwrap();
        let opchar = opsplit.next().unwrap();
        let b = opsplit.next().unwrap().parse::<i64>().unwrap();

        /*
        When it performs “addition” it adds the two numbers entered, then subtracts the result from the previous operation.
        When it performs “subtraction” it subtracts the two numbers entered, then multiplies the result by the previous operation’s result.
        When it performs “multiplication” it squares its answer after multiplying the two numbers entered.
        When it performs “division” it divides the first number by 2 if it is even, otherwise it adds 1 to the first number and divides it by 2.
         */
        match opchar {
            "+" => {
                result = (a + b) - result;
            }
            "-" => {
                result *= a - b;
            }
            "*" => {
                result = (a * b).pow(2);
            }
            "/" => {
                if a % 2 == 0 {
                    result = a / 2;
                } else {
                    result = (a + 1) / 2;
                }
            }
            _ => unreachable!("Unknown operator"),
        }
        println!("{result}");
    }
}

fn lostlineup() {
    let _friend_count: usize = read_one();
    let separations = read_vec::<usize>();

    let mut intermediate = separations
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    intermediate.sort_by(|(_, separation_a), (_, separation_b)| separation_a.cmp(separation_b));

    print!("1");
    for (index, _) in intermediate {
        let friend_id = index + 2;
        print!(" {friend_id}");
    }
    println!();
}

fn eligibility() {
    let count: u32 = read_one();
    for _ in 0..count {
        let [name, college_date, birth_date, courses] = read_array::<String, 4>();
        let college_year = college_date
            .split('/')
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let birth_year = birth_date
            .split('/')
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let course_count = courses.parse::<u32>().unwrap();

        let classification = if college_year >= 2010 || birth_year >= 1991 {
            "eligible"
        } else if course_count >= 41 {
            "ineligible"
        } else {
            "coach petitions"
        };

        println!("{name} {classification}");
    }
}

fn inputscandal() {
    use std::io::{Read, Write};
    let mut buffer = Vec::with_capacity(1_200_000);
    std::io::stdin().read_to_end(&mut buffer).unwrap();
    let linecount = buffer.iter().filter(|&&x| x == b'\n').count();
    println!("{linecount}");
    std::io::stdout().write_all(&buffer).unwrap();
}

fn tolower() {
    let [problems, testcases_per_problem] = read_array::<u32, 2>();
    let mut solved_problems = 0;
    'problems_loop: for _ in 0..problems {
        let mut testcases = Vec::new();
        for _ in 0..testcases_per_problem {
            let input = read_str();
            testcases.push(input);
        }
        for case in testcases {
            if case[1..] != case[1..].to_ascii_lowercase() {
                continue 'problems_loop;
            }
        }
        solved_problems += 1;
    }
    println!("{solved_problems}");
}

fn buka() {
    // BigInt not required. A and B are powers of ten. String lengths can be standins.
    let a = read_str();
    let sign = read_str();
    let b = read_str();

    if sign == "*" {
        let count = a.len() - 1 + b.len() - 1;
        println!("1{}", "0".repeat(count));
    } else if sign == "+" {
        // The tricky case. Ones adding in different digits means simply determining which goes where
        // Ones adding in the same digits means handling flipping to a two.
        if a.len() == b.len() {
            println!("2{}", "0".repeat(a.len() - 1));
        } else {
            let longer_zerocount = std::cmp::max(a.len() - 1, b.len() - 1);
            let shorter_zerocount = std::cmp::min(a.len() - 1, b.len() - 1);
            println!(
                "1{}1{}",
                "0".repeat(longer_zerocount - shorter_zerocount - 1),
                "0".repeat(shorter_zerocount)
            );
        }
    } else {
        unreachable!("You promised only addition or multiplication!");
    }
}

fn averagecharacter() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");

    let mut sum = 0;
    let mut count = 0;
    for char in input.chars() {
        if (' '..='~').contains(&char) {
            sum += char as u8 as usize;
            count += 1;
        }
    }
    let average_char = sum.div_euclid(count) as u8 as char;
    println!("{}", average_char);
}

fn undeadoralive() {
    const SMILEY: &str = ":)";
    const SADFACE: &str = ":(";
    //If the line contains only smiley faces and no frowny faces, then output alive.
    // If the line contains only frowny faces and no smiley faces, then output undead.
    // If the line contains both smiley faces and frowny faces, then output double agent.
    // Otherwise, output machine.
    let input = read_str();
    if input.contains(SMILEY) && input.contains(SADFACE) {
        println!("double agent");
    } else if input.contains(SMILEY) {
        println!("alive");
    } else if input.contains(SADFACE) {
        println!("undead");
    } else {
        println!("machine");
    }
}

fn testdrive() {
    let [a, b, c] = read_array::<f32, 3>();
    let ab = b - a;
    let bc = c - b;
    let abs_ab = ab.abs();
    let abs_bc = bc.abs();
    if (ab.is_sign_positive() && bc.is_sign_negative())
        || (ab.is_sign_negative() && bc.is_sign_positive())
    {
        println!("turned");
    } else if ab == bc {
        println!("cruised");
    } else if abs_ab > abs_bc {
        println!("braked");
    } else {
        println!("accelerated");
    }
}

fn temperature() {
    let [zero, dx] = read_array::<f32, 2>();
    let diff_dx = 1f32 - dx;

    if diff_dx == 0f32 {
        if zero == 0f32 {
            println!("ALL GOOD");
        } else {
            println!("IMPOSSIBLE");
        }
    } else {
        println!("{}", zero / diff_dx);
    }
}

fn stafur() {
    let letter = read_str();

    if ["A", "E", "I", "O", "U"].contains(&letter.as_str()) {
        println!("Jebb");
    } else if letter == "Y" {
        println!("Kannski");
    } else {
        println!("Neibb");
    }
}

fn helpaphd() {
    let count_cases = read_one();

    for _ in 0..count_cases {
        let line = read_str();

        if line.contains("P=NP") {
            println!("skipped");
        } else {
            let [a, b] = line
                .split('+')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap();
            println!("{}", a + b);
        }
    }
}

fn sith() {
    let _name = read_str();
    let a: i32 = read_one();
    let b: i32 = read_one();
    let c: i32 = read_one();

    if a - b >= 0 {
        println!("VEIT EKKI");
    } else if c < 0 {
        println!("JEDI");
    } else {
        println!("SITH");
    }
}

fn provincesandgold() {
    /*
    Province (costs 8, worth 6 victory points)
    Duchy (costs 5, worth 3 VP)
    Estate (costs 2, worth 1 VP)

    Gold (cost 6, worth 3)
    Silver (cost 3, worth 2)
    Copper (cost 0, worth 1)
    */
    const VP_PROVINCE_COST: usize = 8;
    const VP_DUCHY_COST: usize = 5;
    const VP_ESTATE_COST: usize = 2;
    const TREASURE_GOLD_COST: usize = 6;
    const TREASURE_SILVER_COST: usize = 3;
    const TREASURE_COPPER_COST: usize = 0;
    const TREASURE_GOLD_POWER: usize = 3;
    const TREASURE_SILVER_POWER: usize = 2;
    const TREASURE_COPPER_POWER: usize = 1;

    let [golds, silvers, coppers] = read_array::<usize, 3>();
    let hand_buying_power = golds * TREASURE_GOLD_POWER
        + silvers * TREASURE_SILVER_POWER
        + coppers * TREASURE_COPPER_POWER;

    if hand_buying_power >= VP_PROVINCE_COST {
        print!("Province");
    } else if hand_buying_power >= VP_DUCHY_COST {
        print!("Duchy");
    } else if hand_buying_power >= VP_ESTATE_COST {
        print!("Estate");
    }

    if hand_buying_power >= VP_ESTATE_COST {
        print!(" or ");
    }

    if hand_buying_power >= TREASURE_GOLD_COST {
        println!("Gold");
    } else if hand_buying_power >= TREASURE_SILVER_COST {
        println!("Silver");
    } else {
        println!("Copper");
    }
}

fn onechicken() {
    let [attendees, amount_present] = read_array::<i32, 2>();

    let leftover = amount_present - attendees;
    if leftover == 1 {
        println!("Dr. Chaz will have 1 piece of chicken left over!");
    } else if leftover == -1 {
        println!("Dr. Chaz needs 1 more piece of chicken!");
    } else if leftover < 0 {
        println!("Dr. Chaz needs {} more pieces of chicken!", -leftover);
    } else {
        println!("Dr. Chaz will have {leftover} pieces of chicken left over!");
    }
}

fn judgingmoose() {
    let [left, right] = read_array::<u8, 2>();

    if left == 0 && right == 0 {
        println!("Not a moose");
    } else {
        if left != right {
            print!("Odd ");
        } else {
            print!("Even ");
        }
        print!("{}", std::cmp::max(left, right) * 2);
    }
}

fn internationaldates() {
    let date_string = read_str();
    let mut split = date_string.split('/');
    let first: u8 = split.next().unwrap().parse().unwrap();
    let second: u8 = split.next().unwrap().parse().unwrap();

    if first > 12 {
        println!("EU");
    } else if second > 12 {
        println!("US");
    } else {
        println!("either");
    }
}

fn vajningsplikt() {
    #[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    impl std::str::FromStr for Direction {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "North" => Ok(Direction::North),
                "South" => Ok(Direction::South),
                "East" => Ok(Direction::East),
                "West" => Ok(Direction::West),
                _ => Err(()),
            }
        }
    }

    fn going_straight(from: Direction, to: Direction) -> bool {
        (from == Direction::North && to == Direction::South)
            || (from == Direction::South && to == Direction::North)
            || (from == Direction::East && to == Direction::West)
            || (from == Direction::West && to == Direction::East)
    }

    fn opposite_direction(from: Direction, other: Direction) -> bool {
        going_straight(from, other)
    }

    fn on_my_right(from: Direction, other: Direction) -> bool {
        (from == Direction::North && other == Direction::West)
            || (from == Direction::East && other == Direction::North)
            || (from == Direction::South && other == Direction::East)
            || (from == Direction::West && other == Direction::South)
    }

    let [from, to, other_vehicle_from] = read_array::<Direction, 3>();

    // You want to pass straight through the intersection; another vehicle is approaching from your right.
    // You want to turn left at the intersection; another vehicle is approaching from the opposite direction or from your right.
    //
    // Note that it is not important in which direction the other vehicle wants to leave the intersection.
    // Write a program that decides if your car should yield the right-of-way to the other vehicle.

    let have_to_yield = (going_straight(from, to) && on_my_right(from, other_vehicle_from))
        || (on_my_right(to, from)
            && (going_straight(from, other_vehicle_from) || on_my_right(from, other_vehicle_from)));

    if have_to_yield {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn moscowdream() {
    let [
        easy_problems,
        medium_problems,
        hard_problems,
        total_required,
    ] = read_array::<u8, 4>();

    if easy_problems < 1
        || medium_problems < 1
        || hard_problems < 1
        || easy_problems + medium_problems + hard_problems < total_required
        || total_required < 3
    {
        println!("NO");
    } else {
        println!("YES");
    }
}

fn mork() {
    #[repr(u32)]
    #[derive(PartialOrd, PartialEq)]
    enum ScoringPattern {
        NeitherTeam = 0,
        SingleTeam = 1,
        BothTeams = 2,
    }

    let goals_scored: u64 = read_one();
    let scoring_pattern: ScoringPattern = unsafe { std::mem::transmute(read_one::<u32>()) };

    if scoring_pattern == ScoringPattern::NeitherTeam
        || (scoring_pattern == ScoringPattern::BothTeams && goals_scored == 2)
    {
        println!("Jebb");
    } else {
        println!("Neibb");
    }
}

fn mergjadmal() {
    let input = read_str();
    if input.contains("69") || input.contains("420") {
        println!("Mergjad!");
    } else {
        println!("Leim!");
    }
}

fn laptopsticker() {
    let [
        computer_width,
        computer_height,
        sticker_width,
        sticker_height,
    ] = read_array::<u32, 4>();
    if computer_width >= sticker_width + 2 && computer_height >= sticker_height + 2 {
        println!("1");
    } else {
        println!("0");
    }
}

fn hvertskalmaeta() {
    let attendee_home = read_str();
    if ["Fjardabyggd", "Mulathing", "Akureyri"].contains(&attendee_home.as_str()) {
        println!("Akureyri");
    } else {
        println!("Reykjavik");
    }
}

fn ginfizz() {
    let i: u32 = read_one();
    println!("{} ml gin", 45 * i);
    println!("{} ml fresh lemon juice", 30 * i);
    println!("{} ml simple syrup", 10 * i);
    if i == 1 {
        println!("{} slice of lemon", i);
    } else {
        println!("{} slices of lemon", i);
    }
}

fn conteststruggles() {
    let [total_problems, solved_so_far] = read_array::<f64, 2>();
    let [average_difficulty_all, estimate_difficulty_solved] = read_array::<f64, 2>();
    let remaining_points =
        average_difficulty_all * total_problems - estimate_difficulty_solved * solved_so_far;
    let estimated_remaining_difficulty = remaining_points / (total_problems - solved_so_far);
    if !(0.0f64..=100.0f64).contains(&estimated_remaining_difficulty) || remaining_points < 0f64 {
        println!("impossible");
    } else {
        println!("{estimated_remaining_difficulty}");
    }
}

fn chardonnay() {
    let deciliters_required: u64 = read_one();
    let tax = if deciliters_required % 7 > 0 { 1 } else { 0 };

    println!("{}", deciliters_required + tax);
}

fn canadianseh() {
    let suspect_sentence = read_str();

    if suspect_sentence.strip_suffix("eh?").is_some() {
        println!("Canadian!");
    } else {
        println!("Imposter!");
    }
}

fn blueberrywaffle() {
    let [flip_time, cook_duration] = read_array::<f32, 2>();
    if (cook_duration / flip_time).round() as u32 % 2 == 0 {
        println!("up");
    } else {
        println!("down");
    }
}

fn titlecost() {
    let entry = read_vec::<String>();

    let cost = f64::clamp(
        entry[0].len() as f64,
        0f64,
        entry[1].parse::<f64>().unwrap(),
    );
    println!("{cost}");
}

fn shandy() {
    let beer_bottles: u64 = read_one();
    let lemonade_bottles: u64 = read_one();
    println!("{}", std::cmp::min(beer_bottles * 2, lemonade_bottles * 2));
}

fn gcvwr() {
    let [gcvwr, truck_weight, _number_of_items]: [u32; 3] = read_vec().try_into().unwrap();
    let random_kram_weight = read_vec::<u32>().iter().sum::<u32>();
    let remaining_budget = gcvwr
        .saturating_sub(truck_weight)
        .wrapping_mul(9)
        .div_euclid(10)
        .saturating_sub(random_kram_weight);
    println!("{remaining_budget}");
}

fn ekkidaudi() {
    let line_one = read_str();
    let line_two = read_str();
    let mut split_one = line_one.split('|');
    let mut split_two = line_two.split('|');
    let human_text = [
        split_one.next(),
        split_two.next(),
        Some(" "),
        split_one.next(),
        split_two.next(),
    ]
    .iter()
    .copied()
    .collect::<Option<String>>();

    println!("{}", human_text.unwrap());
}

fn betting() {
    let input: u32 = read_one();
    println!("{}", (input as f64 / 100.0).recip());
    println!("{}", ((100.0 - input as f64) / 100.0).recip());
}

fn thelastproblem() {
    let name = read_str();
    println!("Thank you, {name}, and farewell!");
}

fn romans() {
    let miles: f64 = read_one();
    println!("{}", (miles * 1000.0 * 5280.0 / 4854.0).round());
}

fn planina() {
    let input: u32 = read_one();
    println!("{}", (2u64.pow(input) + 1).pow(2));
}

fn heroesofvelmar() {
    let heroes = std::collections::HashMap::<&str, Card>::from(
        [
            Card {
                name: "Shadow",
                energy_cost: 4,
                power_level: 6,
                ability: None,
            },
            Card {
                name: "Gale",
                energy_cost: 3,
                power_level: 5,
                ability: None,
            },
            Card {
                name: "Ranger",
                energy_cost: 2,
                power_level: 4,
                ability: None,
            },
            Card {
                name: "Anvil",
                energy_cost: 5,
                power_level: 7,
                ability: None,
            },
            Card {
                name: "Vexia",
                energy_cost: 2,
                power_level: 3,
                ability: None,
            },
            Card {
                name: "Guardian",
                energy_cost: 6,
                power_level: 8,
                ability: None,
            },
            Card {
                name: "Thunderheart",
                energy_cost: 5,
                power_level: 6,
                ability: Some(|_, location_cards| {
                    // "Phalanx - If the location this card is played at has 4 friendly cards, including this one, then its power is doubled.
                    // Note that other buffs the card receives are not doubled."
                    if location_cards.len() >= 4 {
                        6 // This card's power is doubled when 4 friendlies in this spot.
                    } else {
                        0
                    }
                }),
            },
            Card {
                name: "Frostwhisper",
                energy_cost: 1,
                power_level: 2,
                ability: None,
            },
            Card {
                name: "Voidclaw",
                energy_cost: 1,
                power_level: 3,
                ability: None,
            },
            Card {
                name: "Ironwood",
                energy_cost: 1,
                power_level: 3,
                ability: None,
            },
            Card {
                name: "Zenith",
                energy_cost: 6,
                power_level: 4,
                ability: Some(|location, _| {
                    // "Centered Focus - If this card is played at the center location, +5 power."
                    if location == 1 {
                        // Could use Enum here to denote center statt index.
                        5
                    } else {
                        0
                    }
                }),
            },
            Card {
                name: "Seraphina",
                energy_cost: 1,
                power_level: 1,
                ability: Some(|_, location_cards| {
                    // "Celestial Healing - Seraphina grants +1 power to each friendly card at this location, other than itself.
                    // This includes other Seraphina cards."
                    // location_cards.iter().filter(|card| card.name != "Seraphina").count() as PowerValue

                    // The ambiguous wording means I need to try this too:
                    location_cards.len().saturating_sub(1) as PowerValue
                    // figures it's this one. Dumb. so dumb.
                }),
            },
        ]
        .map(|card| (card.name, card)),
    );

    type LocationValue = u32;
    type EnergyValue = i32;
    type PowerValue = i32;

    #[derive(Debug, Default, Copy, Clone)]
    struct Card {
        name: &'static str,
        energy_cost: EnergyValue,
        power_level: PowerValue,
        ability: Option<fn(LocationValue, &Vec<Card>) -> PowerValue>, // An ability looks at the location and all (friendly) cards at said location and returns a total effect power.
    }

    // Gameplay goes by resolving each location and awarding a point to each won location
    let mut player_one_total_power = 0;
    let mut player_two_total_power = 0;
    let mut player_one_score = 0;
    let mut player_two_score = 0;
    for location in 0..=2 {
        let player_one_card_names: &[String] = &read_vec()[1..];
        let player_two_card_names: &[String] = &read_vec()[1..];
        let player_one_cards: Vec<Card> = player_one_card_names
            .iter()
            .map(|name| *heroes.get(name.as_str()).unwrap())
            .collect();
        let player_two_cards: Vec<Card> = player_two_card_names
            .iter()
            .map(|name| *heroes.get(name.as_str()).unwrap())
            .collect();

        let mut player_one_location_power = 0;
        for card in &player_one_cards {
            player_one_location_power += card.power_level;
            if let Some(closure) = card.ability {
                player_one_location_power += closure(location, &player_one_cards);
            }
        }

        let mut player_two_location_power = 0;
        for card in &player_two_cards {
            player_two_location_power += card.power_level;
            if let Some(closure) = card.ability {
                player_two_location_power += closure(location, &player_two_cards);
            }
        }

        player_one_total_power += player_one_location_power;
        player_two_total_power += player_two_location_power;

        // eprintln!("round {location} power {player_one_location_power} / {player_two_location_power}");

        match player_one_location_power.cmp(&player_two_location_power) {
            std::cmp::Ordering::Less => player_two_score += 1,
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => player_one_score += 1,
        }
    }

    match player_one_score
        .cmp(&player_two_score)
        .then(player_one_total_power.cmp(&player_two_total_power))
    {
        std::cmp::Ordering::Less => println!("Player 2"),
        std::cmp::Ordering::Equal => println!("Tie"),
        std::cmp::Ordering::Greater => println!("Player 1"),
    }
}

fn greetings2() {
    let greeting = read_str();
    // Greeting guaranteed to be of the form `h[e]+y`
    let e_count = greeting.len() - 2;
    let mut reply = "h".to_string();
    for _ in 0..e_count * 2 {
        reply.push('e');
    }
    reply.push('y');

    println!("{reply}");
}

fn fimmtudagstilbod() {
    let year: u32 = read_one();
    if year < 1993 {
        panic!("Mahjong Pizza doesn't exist yet.")
    } else if year <= 2020 {
        println!("1000");
    } else {
        let new_price = 1000 + 100 * (year - 2020);
        println!("{new_price}");
    }
}

fn dartscores() {
    kattis_struct!(Point { x: f32, y: f32 });
    let test_case_count = read_one();

    fn score_throw(point: Point) -> u32 {
        let distance = (point.x.powi(2) + point.y.powi(2)).sqrt();
        (11f32 - (distance / 20f32).ceil()).clamp(0f32, 10f32) as u32
    }

    for _ in 0..test_case_count {
        let throw_count = read_one();

        let mut score = 0;
        for _ in 0..throw_count {
            let point: Point = read_one();
            score += score_throw(point);
        }
        println!("{}", score);
    }
}

fn zyxab() {
    let suggestion_count: u64 = read_one();
    let mut names = Vec::with_capacity(suggestion_count as usize);

    for _ in 0..suggestion_count {
        names.push(read_str());
    }

    fn valid_name(name: &str) -> bool {
        if name.len() < 5 {
            return false;
        }

        let mut letter_set = std::collections::HashSet::<char>::with_capacity(name.len());
        for letter in name.chars() {
            if !letter_set.insert(letter) {
                return false;
            }
        }
        true
    }

    fn better_name(a: &str, b: &str) -> std::cmp::Ordering {
        a.len().cmp(&b.len()).then(a.cmp(b).reverse())
    }

    names.retain(|name| valid_name(name));
    names.sort_by(|a, b| better_name(a, b));

    if names.is_empty() {
        println!("Neibb");
    } else {
        println!("{}", names.first().unwrap())
    }
}

fn rollingthedice() {
    let input = read_str();

    let mut offset_splitter = input.split_terminator('+');
    let dice_string = offset_splitter.next().unwrap();
    let mut d_splitter = dice_string.split('d');
    let dice_count = d_splitter.next().unwrap().parse::<u64>().unwrap();
    let dice_sides = d_splitter.next().unwrap().parse::<u64>().unwrap();
    let offset = offset_splitter
        .next()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0u64);
    let roll_expected_value: f64 =
        (dice_count as f64) * ((dice_sides + 1) as f64 / 2f64) + (offset as f64);
    println!("{}", roll_expected_value)
}

fn shatteredcake() {
    use std::io::Read;

    struct FastReader {
        buf: Vec<u8>,
        idx: usize,
        len: usize,
    }
    impl FastReader {
        fn new() -> Self {
            let mut buf = Vec::with_capacity(1 << 20);
            std::io::stdin().read_to_end(&mut buf).unwrap();
            let len = buf.len();
            FastReader { buf, idx: 0, len }
        }
        #[inline]
        fn skip_ws(&mut self) {
            while self.idx < self.len {
                let b = unsafe { self.buf.get_unchecked(self.idx) };
                if *b > b' ' {
                    break;
                }
                self.idx += 1;
            }
        }
        #[inline]
        fn next_usize(&mut self) -> usize {
            self.skip_ws();
            let mut v: usize = 0;
            while self.idx < self.len {
                let b = unsafe { self.buf.get_unchecked(self.idx) };
                if !b.is_ascii_digit() {
                    break;
                }
                v = v * 10 + (b - b'0') as usize;
                self.idx += 1;
            }
            v
        }
    }
    let mut rdr = FastReader::new();

    let cake_width = rdr.next_usize();
    let cake_pieces = rdr.next_usize();

    let mut total_area = 0;

    for _ in 0..cake_pieces {
        let shard_width = rdr.next_usize();
        let shard_height = rdr.next_usize();
        total_area += shard_width * shard_height;
    }
    println!("{}", total_area.div_euclid(cake_width));
}

fn numberreduction() {
    let mut number_of_items: u64 = read_one();

    let mut iterations = 0usize;
    while number_of_items > 1 {
        iterations += 1;
        if number_of_items % 2 == 0 {
            number_of_items /= 2;
        } else {
            number_of_items *= 3;
            number_of_items += 1;
        }
    }
    println!("{iterations}");
}

//Print a digit between 1 and 9 the result of repeatedly multiplying the nonzero digits of x
fn sifferprodukt() {
    let input_number: u64 = read_one();
    let mut temp = input_number;
    while temp >= 10 {
        let mut multiple = 1u64;
        while temp > 0 {
            let modulo = temp % 10;
            temp /= 10;
            if modulo != 0 {
                multiple *= modulo;
            }
        }
        temp = multiple;
    }

    println!("{}", temp);
}

fn veci() {
    let input_number: u64 = read_one();
    let mut digits = Vec::<u64>::with_capacity(7);

    let mut temp = input_number;
    while temp > 0 {
        let modulo = temp % 10;
        temp /= 10;
        digits.push(modulo);
    }
    digits.reverse();

    let mut new_number;

    let mut best_number = u64::MAX;

    for i in (0..digits.len()).rev() {
        let (high, low) = digits.split_at(i);
        let mut low_sorted = low.to_vec();
        low_sorted.sort();

        new_number = 0u64;
        for entry in high {
            new_number *= 10;
            new_number += *entry;
        }

        if let Some(relevant_digit) = low_sorted.extract_if(.., |x| *x > digits[i]).next() {
            new_number *= 10;
            new_number += relevant_digit;
        } else {
            continue;
        }

        for entry in low_sorted {
            new_number *= 10;
            new_number += entry;
        }

        eprintln!("{high:?}, {low:?}, {new_number}");

        if new_number > input_number {
            best_number = best_number.min(new_number);
        }
    }

    if best_number <= input_number || best_number == u64::MAX {
        println!("0");
    } else {
        println!("{best_number}");
    }
}

fn karte() {
    const CARDS_PER_SUIT: usize = 13;
    const NUMBER_OF_SUITS: usize = 4;
    const ERROR_TEXT: &str = "GRESKA";

    let card_string = read_str();
    let mut index = 0usize;

    let mut set = std::collections::HashSet::<&str>::new();
    let mut counts = [0usize; NUMBER_OF_SUITS];

    while index + 3 <= card_string.len() {
        let temp = &card_string[index..index + 3];
        index += 3;

        match temp.chars().next().unwrap() {
            'P' => counts[0] += 1,
            'K' => counts[1] += 1,
            'H' => counts[2] += 1,
            'T' => counts[3] += 1,
            _ => panic!(),
        }

        if !set.insert(temp) {
            println!("{}", ERROR_TEXT);
            std::process::exit(0);
        }
    }

    let missing_cards_output = counts
        .iter()
        .map(|&count| CARDS_PER_SUIT.saturating_sub(count).to_string())
        .collect::<Vec<String>>()
        .join(" ");
    print!("{}", missing_cards_output);
}

fn modulo() {
    let mut set = std::collections::HashSet::<u64>::with_capacity(42);
    const DIVISOR: u64 = 42;
    for _ in 0..10 {
        set.insert(read_one::<u64>() % DIVISOR);
    }
    println!("{}", set.len());
}

fn bitteeinbit() {
    let input = read_str();
    println!("{}", input.chars().next().unwrap());
}

fn threeblindmice() {
    let initial_distance: u64 = read_one();
    let speed_of_wife: u64 = read_one();
    let speed_of_mouse_one: u64 = read_one();
    let speed_of_mouse_two: u64 = read_one();
    let speed_of_mouse_three: u64 = read_one();

    // Solve the three pairs equations
    // initial_distance - speed_of_wife * t = speed_of_mouse_one * t
    // therefore it's the t = d / (sum of speeds), then distance is speed * t

    let intercept_time_one =
        (initial_distance as f32) / (speed_of_wife as f32 + speed_of_mouse_one as f32);
    let intercept_time_two =
        (initial_distance as f32) / (speed_of_wife as f32 + speed_of_mouse_two as f32);
    let intercept_time_three =
        (initial_distance as f32) / (speed_of_wife as f32 + speed_of_mouse_three as f32);

    let intercept_point_one = (speed_of_mouse_one as f32) * intercept_time_one;
    let intercept_point_two = (speed_of_mouse_two as f32) * intercept_time_two;
    let intercept_point_three = (speed_of_mouse_three as f32) * intercept_time_three;

    println!(
        "{}",
        (2f32 * intercept_point_one + 2f32 * intercept_point_two + 2f32 * intercept_point_three)
            .round() as u32
    );
}

fn matchstickmen() {
    // Do I want to solve using the triangles or the rhombuses?
    // I know how to get the triangle heights trivially. But then in the next layer, the bottom isn't parallel.
    // i.e., the triangle is rotated. So then I need some way to generally solve where two circles of length l intersect.

    // Draw a straight line from me to you, compute its vector and get distance d.
    // Using hypotenuse t (matchstick_length) and d/2, you get the height of this (rotated) triangle h.
    // make vector for this h, perpendicular to the other vector between me and you. The endpoint is the top of this triangle.

    let [_number_of_bottom_row_matchsticks, matchstick_length] = read_array::<f64, 2>();
    let foot_positions = read_vec::<f64>();

    let mut tops = std::iter::once(0f64)
        .chain(foot_positions)
        .scan(0f64, |start, x| {
            let out_x = x + *start;
            *start = out_x;
            Some((out_x, 0f64))
        })
        .collect::<Vec<(f64, f64)>>();

    while tops.len() > 1 {
        let mut next_tops = Vec::<(f64, f64)>::with_capacity(tops.len() - 1);

        for window in tops.windows(2) {
            let [(me_x, me_y), (you_x, you_y)]: [(f64, f64); 2] = window.try_into().unwrap();
            let dx = you_x - me_x;
            let dy = you_y - me_y;
            let d = (dx.powi(2) + dy.powi(2)).sqrt();
            let d2_2 = (dx.powi(2) + dy.powi(2)) / 4f64; // d, but precomputed (d/ 2)^2 for precision reasons.
            let h = (matchstick_length.powi(2) - d2_2).sqrt();

            let middle_x = me_x + (dx / 2f64);
            let middle_y = me_y + (dy / 2f64);
            let norm_dx = dx / d;
            let norm_dy = dy / d;
            let perp_dx = -norm_dy;
            let perp_dy = norm_dx;

            let next_x = middle_x + h * perp_dx;
            let next_y = middle_y + h * perp_dy;

            // println!("\t{:?}", (next_x, next_y));
            // println!("{:?}\t{:?}", (me_x, me_y), (you_x, you_y));

            next_tops.push((next_x, next_y));
        }

        tops = next_tops;
    }

    let [_peak_x, tower_height]: [f64; 2] = tops[0].into();
    println!("{}", tower_height);
}

fn fastestestfunction() {
    let [before_percent, after_percent]: [f64; 2] = read_array();
    let before = before_percent / 100.0;
    let after = after_percent / 100.0;
    /*
    foo + other = total
    foo / total = A
    new_foo + other = new_total
    new_foo / new_total = B
    ---
    foo = A * total
    new_foo = B * new_total

    A * total + other = total
    B * new_total + other = new_total

    (1 - A) * total = other
    (1 - B) * new_total = other

    ANSWER: new_foo / foo = (B * new_total) / (A * total)

    (B * new_total) / (A * total)

    --

    other = (1 - A) * total = (1 - B) * new_total

    new_total = (1-A)/(1-B)*total;
    (B * ((1-A)/(1-B))*total) / (A * total)
    (B * ((1-A)/(1-B))) / (A)

    B/A * (1-A)/(1-B)

    Answer is actually inverse because speedup. so A/B * (1-B)/(1-A)
     */

    println!("{}", (before / after) * ((1.0 - after) / (1.0 - before)));
}

fn abovesealevel() {
    let input: f64 = read_one();
    println!("{}", input - 0.3f64);
}

fn equalshots() {
    let [first_count, second_count] = read_array();
    let mut first_milliliters = 0;
    let mut second_milliliters = 0;
    for _ in 0..first_count {
        let [volume, percent]: [u16; 2] = read_array();
        first_milliliters += volume * (100 - percent);
    }

    for _ in 0..second_count {
        let [volume, percent]: [u16; 2] = read_array();
        second_milliliters += volume * (100 - percent);
    }

    if first_milliliters == second_milliliters {
        println!("same");
    } else {
        println!("different")
    }
}

fn grading() {
    let grade_boundaries = read_vec().iter().copied().take(5).collect::<Vec<u16>>();
    let grade_letter = ["A", "B", "C", "D", "E", "F"];
    let grade = read_one::<u16>();

    for (bound, letter) in grade_boundaries.iter().copied().zip(grade_letter) {
        if bound <= grade {
            println!("{}", letter);
            break;
        }
    }
    if *grade_boundaries.last().unwrap() > grade {
        println!("{}", grade_letter.last().unwrap());
    }
}

fn knotknowledge() {
    let knot_count = read_one::<u32>();
    let knots_tested = read_vec::<u32>();
    let mut knots_learned = std::collections::HashSet::<u32>::with_capacity(knot_count as usize);
    for knot in read_vec::<u32>() {
        knots_learned.insert(knot);
    }
    for knot in knots_tested {
        if !knots_learned.contains(&knot) {
            println!("{}", knot);
        }
    }
}

fn sumkindofproblem() {
    /*
    Compute:
    The sum of the first N positive integers: N*(N+1)/2
    The sum of the first N odd integers: N*N
    The sum of the first N even integers: N*(N+1) or (N*N)+N
     */
    let datasets = read_one::<u64>();
    for _ in 0..datasets {
        let [dataset_id, n]: [u64; 2] = read_array();
        println!(
            "{dataset_id} {} {} {}",
            (n * n + n).div_euclid(2),
            n * n,
            n * n + n
        );
    }
}

/// Replace each input character with a matrix that is "mag_rows & mag_cols" in dimension
fn skener() {
    let [rows, columns, row_magnification, column_magnification]: [u32; 4] = read_array();

    let mut output_matrix = vec![
        vec![' '; (columns * column_magnification) as usize];
        (rows * row_magnification) as usize
    ];

    let mut input_matrix = Vec::new();
    for _ in 0..rows {
        input_matrix.push(
            read_str()
                .chars()
                .take(columns as usize)
                .collect::<Vec<char>>(),
        );
    }

    // Go through each "pixel" of the output and remap backward to the input
    for (row, row_chars) in output_matrix.iter_mut().enumerate() {
        for (column, char) in row_chars.iter_mut().enumerate() {
            *char = input_matrix[row.div_euclid(row_magnification as usize)]
                [column.div_euclid(column_magnification as usize)];
        }
    }

    for line in output_matrix {
        println!("{}", line.iter().collect::<String>());
    }
}

fn parkingpandemonium() {
    let circle_time: u32 = read_one();
    let _irrelevant: u32 = read_one();
    let circle_count: u32 = read_one();
    println!("{}", circle_count * circle_time)
}

fn cosmicpathoptimization() {
    let temp_count: u32 = read_one();
    let temperatures = read_vec::<u32>();
    println!(
        "{}",
        temperatures.iter().sum::<u32>().div_euclid(temp_count)
    )
}

fn fizzbuzz() {
    let [fizz_div, buzz_div, count] = read_array::<u32, 3>();

    for i in 1..=count {
        if i.is_multiple_of(fizz_div) && i.is_multiple_of(buzz_div) {
            print!("FizzBuzz")
        } else if i.is_multiple_of(fizz_div) {
            print!("Fizz")
        } else if i.is_multiple_of(buzz_div) {
            print!("Buzz")
        } else {
            print!("{}", i)
        }
        println!();
    }
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
    let [height, width, length, cat_volume] = read_array::<u64, 4>();
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

        let (sum, count) = self.fold((zero.clone(), zero.clone()), |(s, c), x| {
            (s + x, c + one.clone())
        });
        if count == zero {
            None
        } else {
            Some(sum / count)
        }
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
    let nums: Vec<u16> = vec![str_a.parse().unwrap(), str_b.parse().unwrap()];

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
        let [length, width] = read_array::<f32, 2>();
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
    let [x1, y1] = read_array::<u16, 2>();
    let [x2, y2] = read_array::<u16, 2>();
    let [x3, y3] = read_array::<u16, 2>();

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
    let [num_matches, box_w, box_h] = read_array::<u16, 3>();
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
    let downsized_dig_time =
        downsized_worker_quota / (dig_rate_per_worker * downsized_worker_count);
    println!("{}", downsized_dig_time);
}

fn triangelfabriken() {
    let angles: Vec<u16> = vec![read_one(), read_one(), read_one()];

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

    let total_probability: f32 = sorted_distances
        .iter()
        .map(|&x| {
            let dice_sides = 6;
            let chances_for_x = dice_sides - (dice_sides + 1 - x as i32).abs();
            chances_for_x as f32 / (dice_sides * dice_sides) as f32
        })
        .sum();

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
        answers_cache: Vec<Option<usize>>,
    }
    impl FactHelper {
        fn new(max: usize) -> FactHelper {
            FactHelper {
                answers_cache: vec![None; max],
            }
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
    println!(
        "{}",
        name_string
            .split('-')
            .map(|x| *x.as_bytes().first().unwrap() as char)
            .collect::<String>()
    );
}

fn r2() {
    let [r1, mean] = read_array::<i32, 2>();

    println!("{}", 2i32 * mean - r1);
}

fn vedurheidar() {
    let wind_speed: u64 = read_one();
    let number_roads: u64 = read_one();
    let mut road_limits: Vec<(String, u64)> = Vec::new();
    for _ in 0..number_roads {
        let [name, limit_str] = read_array::<String, 2>();
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
    let [a, b, c] = read_array::<i64, 3>();
    match a + b == c {
        true => println!("correct!"),
        false => println!("wrong!"),
    }
}

fn whichisgreater() {
    let [a, b] = read_array::<u64, 2>();
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
    let [height, base] = read_array::<u64, 2>().try_into().unwrap();
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
    let [a, b] = read_array::<u64, 2>();
    println!("{}", a + b);
}

fn jackolanternjuxtaposition() {
    let [eyes, noses, mouths] = read_array::<u64, 3>();
    println!("{}", eyes * noses * mouths);
}

fn qaly() {
    let count = read_one::<u64>();
    let mut quality_sum: f32 = 0.0;
    for _ in 0..count {
        let [quality, years] = read_array::<f32, 2>();
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
        let [name, gift_score_str] = read_array::<String, 2>();
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
