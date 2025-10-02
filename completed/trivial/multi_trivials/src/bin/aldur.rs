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

fn main() {
    let count = read_one();
    let mut min_age = u64::max_value();

    for _ in 0..count {
        let input = read_one();
        if input < min_age {
            min_age = input;
        }
    }
    println!("{}", min_age);
}
