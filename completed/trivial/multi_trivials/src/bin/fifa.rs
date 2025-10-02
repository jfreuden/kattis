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
    let number_improvements = read_one::<u64>();
    let improvements_per_year = read_one::<u64>();

    const START_YEAR: u64 = 2022;

    println!(
        "{}",
        START_YEAR + number_improvements / improvements_per_year
    );
}
