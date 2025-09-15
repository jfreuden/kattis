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

fn remoatseating() {
    const SUB_SMALL: u64 = 2;
    const SUB_BIG: u64 = 3;
    const FULL_TEAM: u64 = SUB_SMALL + SUB_BIG;

    let team_count: u64 = read_one();
    let teams = read_vec::<u64>();

    let sub_count = teams.iter().filter(|&&x| x == SUB_SMALL).count() as u32;
    let dom_count = teams.iter().filter(|&&x| x == FULL_TEAM).count() as u32;
    let pairwise = if sub_count > 0 { 2u64 } else { 1 };

    // 10! / 1050 = 3456 = 2^7 * 3^3 ~= (3*2*1)^3 * (2*1)^3 * 2

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
    }

    let mut fact = FactHelper::new(20);

    let numerator = (fact.fact(SUB_SMALL).pow(sub_count) * fact.fact(SUB_BIG).pow(sub_count) * fact.fact(FULL_TEAM).pow(dom_count)) *
        fact.fact((sub_count + dom_count) as u64) *
        2 * // This and the "choose" below are wrong and need adjustment.
        fact.fact((sub_count * 2) as u64)
            .div_ceil(fact.fact(2) * fact.fact((2 * sub_count).saturating_sub(2) as u64));
    let denominator = fact.fact(teams.iter().sum());

    println!("Numerator: {}", numerator);
    println!("Denominator: {}", denominator);
    //println!("Answer: 1/{}", denominator.div_euclid(numerator))
    println!("1/{}", denominator.div_euclid(numerator));




    /*
    Each subgroup has their own permutations of seating
        each group of 2 has 2! permutations.
        each group of 3 has 3! permutations.
        each group of 5 has 5! permutations.

    where each team block sits is a normal permutation.

    there are 2 ways that each 2-3 pair can go together.

    each subgroup has choices. I think this is the deal. it's like 4 choose 2 for the 2nd.
    
    
    or it is a trick? are the 2's and 3's supposed to be joined?
    Each group of 5 has 5! permutations; that's not possible in the second example, so it's not that kind of trick.


    (2!)^(subcount) *
    (3!)^(subcount) *
 {  (4 choose 2) *
 {  2 ways of fitting 2v3 together *
    N! ways of fitting N full teams together.



     */


}
