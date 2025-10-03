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
    let mut phrase_set = std::collections::HashSet::<String>::with_capacity(count as usize);
    for _ in 0..count {
        let input = read_str();
        phrase_set.insert(input);
    }
    println!("{}", phrase_set.len());
}
