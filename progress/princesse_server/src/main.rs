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

fn main() {
    let [starting_mattresses, starting_nights, penalty, pea_location]: [u32; 4] = read_vec().try_into().unwrap();
    let mut nights = starting_nights as i32;
    loop {
        let line = read_vec::<String>();
        if line[0] == "!" {
            if line[1].parse::<u32>().unwrap() == pea_location {
                println!("Correct!");
                std::process::exit(0);
            } else {
                println!("There is no pea in mattress {}", line[1]);
                std::process::exit(1);
            }
        }

        if nights < 0 {
            println!("Ran out of nights");
            std::process::exit(1);
        }

        if line[0] == "#" {
            println!("{} nights remaining", nights);
            continue;
        } else if line[0] != "?" {
            eprintln!("Invalid input");
            std::process::exit(1);
        } else if line[1].contains("...") {
            let [range_min, range_max] = line[1].split("...").map(|x|x.parse::<u32>().unwrap()).collect::<Vec<u32>>().try_into().unwrap();
            if range_min > range_max || range_max >= starting_mattresses {
                eprintln!("range invalid");
                std::process::exit(1);
            }
            println!("Querying singles from {} through {}", range_min, range_max);
            for i in range_min..=range_max {
                println!("? {}", i);

                if i == pea_location {
                    nights -= 1 + penalty as i32;
                    println!("1");
                    break;
                } else {
                    nights -= 1;
                    println!("0");
                }

                if nights < 0 {
                    println!("Ran out of nights");
                    std::process::exit(1);
                }
            }
        } else if line[1].contains("..") {
            // Using custom range notation
            let [range_min, range_max] = line[1].split("..").map(|x|x.parse::<u32>().unwrap()).collect::<Vec<u32>>().try_into().unwrap();
            if range_min > range_max || range_max >= starting_mattresses {
                eprintln!("range invalid");
                std::process::exit(1);
            }

            if (range_min..=range_max).contains(&pea_location) {
                nights -= 1 + penalty as i32;
                println!("1")
            } else {
                nights -= 1;
                println!("0")
            }
        } else {
            let numbers = line[1..].iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

            for number in &numbers {
                if !(0..starting_mattresses).contains(number) {
                    eprintln!("{} is not a valid mattress", number);
                    std::process::exit(1);
                }
            }

            if numbers.contains(&pea_location) {
                nights -= 1 + penalty as i32;
                println!("1")
            } else {
                nights -= 1;
                println!("0")
            }
        }
    }
}
