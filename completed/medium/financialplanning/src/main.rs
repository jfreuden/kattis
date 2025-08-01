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

#[derive(Debug, Default, Copy, Clone)]
struct Investment {
    income: u64,
    cost: u64,
}
impl Investment {
    fn payoff_time(self) -> u64 {
        (self.cost as f64 / self.income as f64).ceil() as u64
    }
    fn retire_time(self, retire_amount: u64) -> u64 {
        ((self.cost + retire_amount) as f64 / self.income as f64).ceil() as u64
    }
}
impl From<[u64; 2]> for Investment {
    fn from([this_income, this_cost]: [u64; 2]) -> Self {
        Investment {
            income: this_income,
            cost: this_cost,
        }
    }
}

fn main() {
    let [number_options, retirement_plan]: [u64; 2] = read_array();
    let mut investments = Vec::<Investment>::with_capacity(number_options as usize);
    for _ in 0..number_options {
        let investment: Investment = read_array().into();
        investments.push(investment);
    }

    investments.sort_by(|a, b| a.payoff_time().cmp(&b.payoff_time()));

    let mut current_strategy = Investment::default();

    for investment in investments {
        let next_strategy = Investment {
            income: investment.income + current_strategy.income,
            cost: investment.cost + current_strategy.cost,
        };

        if cfg!(debug_assertions) {
            println!(
                "Current strategy: payoff: {}, retire: {}",
                current_strategy.payoff_time(),
                current_strategy.retire_time(retirement_plan)
            );
            println!(
                "This investment: payoff: {}, retire: {}",
                investment.payoff_time(),
                investment.retire_time(retirement_plan)
            );
            println!(
                "Next strategy: payoff: {}, retire: {}",
                next_strategy.payoff_time(),
                next_strategy.retire_time(retirement_plan)
            );
            println!("---");
        }

        if next_strategy.retire_time(retirement_plan)
            <= current_strategy.retire_time(retirement_plan)
        {
            current_strategy = next_strategy;
        } else {
            break;
        }
    }
    println!("{}", current_strategy.retire_time(retirement_plan));
}
