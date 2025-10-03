// noinspection
#[allow(unused)]
macro_rules! import {
    ($name:ident) => {
        #[cfg(not(feature = "libfreuden"))]
        mod $name;
        #[cfg(not(feature = "libfreuden"))]
        use $name::*;

        #[cfg(feature = "libfreuden")]
        #[allow(unused_imports)]
        use libfreuden::$name::*;
    };
}

import!(input);
import!(mediocre_bigint);

fn main() {
    use std::io::Read;
    use std::str::FromStr;
    let mut all_lines = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut all_lines)
        .unwrap();
    for untrimmed in all_lines.split_terminator('\n') {
        let line = untrimmed.trim();

        let mut accumulator: MediocreBigint = MediocreBigint::from_str("1").unwrap();
        let mut charmap = std::collections::HashMap::<char, usize>::new();
        for (i, char) in line.chars().enumerate() {
            let letter_count = i + 1;
            *charmap.entry(char).or_insert(0) += 1;

            let char_count = *charmap.entry(char).or_insert(0);
            accumulator.normalize();
            accumulator *= MediocreBigint::from_str(&letter_count.to_string()).unwrap();
            accumulator /= MediocreBigint::from_str(&char_count.to_string()).unwrap();
        }
        println!("{}", accumulator);
    }
}
