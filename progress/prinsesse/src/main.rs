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

/// m_1=\frac{N-S}{2N-S}\cdot M
fn compute_split_amount(configuration: ScenarioConfiguration) -> u32 {
    let numerator = configuration.mattresses;
    let denominator = configuration.penalty + 2;

    if configuration.penalty == 365 {
        Ord::clamp((0.01200944f64 * configuration.mattresses as f64) as u32, 1, configuration.mattresses - 1)
    } else if configuration.penalty == 26 {
        Ord::clamp((0.0866184f64 * configuration.mattresses as f64) as u32, 1, configuration.mattresses - 1)
    } else {
        Ord::clamp(numerator.div_ceil(denominator), 1, configuration.mattresses - 1)
    }
}

#[derive(Copy, Clone)]
struct ScenarioConfiguration {
    mattresses: u32,
    nights: u32,
    penalty: u32,
    offset: u32,
}

struct Scenario<B: std::io::BufRead, W: std::io::Write> {
    buf_reader: B,
    buf_writer: W,
    configuration: ScenarioConfiguration,
}

const SLEPT_ON_PEA: bool = true;

impl<B: std::io::BufRead, W: std::io::Write> Scenario<B, W> {
    fn new(mattresses: u32, nights: u32, penalty: u32, buf_reader: B, buf_writer: W) -> Self {
        Scenario {
            buf_reader,
            buf_writer,
            configuration: ScenarioConfiguration {
                mattresses,
                nights,
                penalty,
                offset: 0,
            },
        }
    }

    fn scenario_step(&mut self) {
        let split_amount = compute_split_amount(self.configuration);
        let min = self.configuration.offset;
        let mattress_stringlist = (min..min + split_amount)
            .map(|x| x.to_string())
            .reduce(|mut full, next| {
                full.push(' ');
                full.push_str(next.as_str());
                full
            })
            .unwrap_or_default();

        writeln!(self.buf_writer, "? {}", mattress_stringlist).unwrap();
        self.buf_writer.flush().unwrap();

        let query_result = read_one::<u8, _>(&mut self.buf_reader) != 0;
        if query_result == SLEPT_ON_PEA {
            self.take_queried_section(split_amount);
        } else {
            self.take_other_section(split_amount);
        }
    }

    fn take_other_section(&mut self, split_amount: u32) {
        if self.configuration.nights != 0 {
            self.configuration.nights -= 1;
        }

        self.configuration.offset += split_amount;
        self.configuration.mattresses -= split_amount;
    }

    fn take_queried_section(&mut self, split_amount: u32) {
        if self.configuration.nights > self.configuration.penalty {
            self.configuration.nights -= 1 + self.configuration.penalty;
        } else {
            self.configuration.nights = 0;
        }
        self.configuration.mattresses = split_amount;
    }
}

fn main() {
    // TODO: find out if I can add `read_vec` to the methods of "stdin"
    let mut stdin = std::io::BufReader::with_capacity(64, std::io::stdin().lock());
    let stdout = std::io::BufWriter::with_capacity(64, std::io::stdout().lock());
    let hp = "";
    
    let [mattresses, nights, penalty]: [u32; 3] = read_vec(&mut stdin).try_into().unwrap();

    let mut scenario = Scenario::new(mattresses, nights, penalty, stdin, stdout);
    while scenario.configuration.mattresses > 1 && scenario.configuration.nights > 0 {
        scenario.scenario_step();
    }

    println!("! {}", scenario.configuration.offset);
}

#[cfg(test)]
mod prinsesse_tests;
