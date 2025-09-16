#![allow(dead_code)]
/// This is a single file that will handle all the "with input <trivial> print <trivial output>" problems
/// Note: this is a variant of the `combined_trivial` crate that is meant for trivial problems of the medium variety
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
    remoatseating();
}

struct FactHelper {
    answers_cache: Vec<Option<u64>>
}
impl FactHelper {
    fn new(max: usize) -> FactHelper {
        FactHelper { answers_cache: vec![None; max] }
    }

    fn fact(&mut self, input: u64) -> u64 {
        if input == 1 || input == 0 {
            return 1;
        } else if let Some(answer) = self.answers_cache[(input - 1) as usize] {
            return answer;
        }
        input * self.fact(input - 1)
    }

    fn nchoose(&mut self, n: u64, choose: u64) -> u64 {
        self.fact(n)
            .div_ceil(self.fact(choose) * self.fact(n.saturating_sub(choose)))
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn remoatseating() {
    const SUB_SMALL: u64 = 2;
    const SUB_BIG: u64 = 3;
    const FULL_TEAM: u64 = SUB_SMALL + SUB_BIG;

    let _team_count: u64 = read_one();
    let teams = read_vec::<u64>();

    let mini_count = teams.iter().filter(|&&x| x == SUB_SMALL).count() as u32;
    let sub_count = teams.iter().filter(|&&x| x == SUB_BIG).count() as u32;
    let dom_count = teams.iter().filter(|&&x| x == FULL_TEAM).count() as u32;

    let mut fact = FactHelper::new(20);
    let internal_permutations = fact.fact(SUB_SMALL).pow(mini_count) *
        fact.fact(SUB_BIG).pow(sub_count) *
        fact.fact(FULL_TEAM).pow(dom_count);
    let team_configurations = fact.fact((mini_count + sub_count + dom_count) as u64);
    let numerator = internal_permutations * team_configurations;
    let denominator = fact.fact(teams.iter().sum());

    let divisor = gcd(numerator, denominator);
    println!("{}/{}", numerator / divisor , denominator / divisor);
}
