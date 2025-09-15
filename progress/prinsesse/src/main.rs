use std::io::Write;
fn read_vec<T: std::str::FromStr, B: std::io::BufRead>(bufreader: &mut B) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    bufreader.read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

fn read_one<T: std::str::FromStr, B: std::io::BufRead>(bufreader: &mut B) -> T
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    bufreader.read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

#[derive(Copy, Clone)]
struct ScenarioConfiguration {
    mattresses: u16,
    nights: u16,
    penalty: u16,
    offset: u16,
    split_amount: u16,
}

struct Scenario<B: std::io::BufRead, W: std::io::Write> {
    buf_reader: B,
    buf_writer: W,
    configuration: ScenarioConfiguration,
}

const SLEPT_ON_PEA: bool = true;

impl<B: std::io::BufRead, W: std::io::Write> Scenario<B, W> {
    fn new(mattresses: u16, nights: u16, penalty: u16, buf_reader: B, buf_writer: W) -> Self {
        Scenario {
            buf_reader,
            buf_writer,
            configuration: ScenarioConfiguration {
                mattresses,
                nights,
                penalty,
                offset: 0,
                split_amount: u16::MAX,
            },
        }
    }

    fn scenario_step(&mut self) {
        let mattress_indices = self.get_split_range();
        let query_result = self.make_query(mattress_indices);
        self.take_based_on_query(query_result);
    }

    fn make_query(&mut self, mattress_indices: std::ops::Range<u16>) -> bool {
        self.print_mattress_query(mattress_indices);
        read_one::<u8, _>(&mut self.buf_reader) != 0
    }

    fn print_mattress_query(&mut self, mattress_indices: std::ops::Range<u16>) {
        let mattress_stringlist = mattress_indices
            .map(|x| x.to_string())
            .reduce(|mut full, next| {
                full.push(' ');
                full.push_str(next.as_str());
                full
            })
            .unwrap_or_default();
        write!(self.buf_writer, "? ").unwrap();
        writeln!(self.buf_writer, "{}",mattress_stringlist).unwrap();
        self.buf_writer.flush().unwrap();
    }

    /// A version of the `scenario_step` call that answers its own queries for testing purposes
    #[allow(unused)]
    fn autonomous_step(&mut self, pea_location: u16) {
        let mattress_indices = self.get_split_range();
        self.print_mattress_query(mattress_indices.clone());
        let pea_in_range = mattress_indices.contains(&pea_location);
        writeln!(self.buf_writer, "{}", pea_in_range as u8).unwrap();
        self.buf_writer.flush().unwrap();
        self.take_based_on_query(pea_in_range);
    }

    fn get_split_range(&mut self) -> std::ops::Range<u16> {
        //println!("get_split_range M={}, N={}", self.configuration.mattresses, self.configuration.nights);

        if self.configuration.mattresses <= self.configuration.nights + 1 {
            self.configuration.split_amount = 1;
        } else if self.configuration.penalty <= 1 {
            self.configuration.split_amount = self.configuration.mattresses.div_ceil(2);
        } else {
            let n = self.configuration.nights;
            let s = self.configuration.penalty;
            let mut magic = (n.saturating_sub(1) % (s + 1)) + 2;

            while (magic * (magic + 1)).div_euclid(2) + self.configuration.penalty < self.configuration.mattresses.div_euclid(2) {
                magic = (magic * (magic + 1)).div_euclid(2) + self.configuration.penalty;
            }

            self.configuration.split_amount = Ord::clamp(magic, 1, self.configuration.mattresses);
        }

        let min = self.configuration.offset;
        let mattress_indices = min..min + self.configuration.split_amount;
        mattress_indices
    }

    fn take_based_on_query(&mut self, query_result: bool) {
        if query_result == SLEPT_ON_PEA {
            self.take_queried_section();
        } else {
            self.take_other_section();
        }
    }

    fn take_other_section(&mut self) {
        if self.configuration.nights != 0 {
            self.configuration.nights -= 1;
        } else {
            self.configuration.nights = u16::MAX;
        }

        self.configuration.offset += self.configuration.split_amount;
        self.configuration.mattresses -= self.configuration.split_amount;
    }

    fn take_queried_section(&mut self) {
        if self.configuration.nights > self.configuration.penalty {
            self.configuration.nights -= 1 + self.configuration.penalty;
        } else {
            self.configuration.nights = 0;
            self.configuration.nights = u16::MAX;
        }
        self.configuration.mattresses = self.configuration.split_amount;
    }
}

fn main() {
    let mut stdin = std::io::BufReader::with_capacity(64, std::io::stdin().lock());

    let [mattresses, nights, penalty]: [u16; 3] = read_vec(&mut stdin).try_into().unwrap();

    let mut scenario = Scenario::new(mattresses, nights, penalty, stdin, std::io::stdout().lock());
    while scenario.configuration.mattresses > 1
        && scenario.configuration.nights > 0
        && scenario.configuration.nights <= nights
    {
        scenario.scenario_step();
    }

    writeln!(scenario.buf_writer, "! {}", scenario.configuration.offset).unwrap();
    scenario.buf_writer.flush().unwrap();

}

#[cfg(test)]
mod prinsesse_tests;
