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
    let _len = read_one::<usize>();
    let word = read_str();

    // 2 - contains("l") - contains("lv")
    // neither = 2
    // one of = 1
    // both, correct = 0
    // both, not lv (vl or l***********v) = 1
    println!(
        "{}",
        2 - word.contains("l") as u8 - word.contains("lv") as u8
    );
}
