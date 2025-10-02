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
    let mut map = std::collections::HashMap::new();

    map.insert("Monnei", read_one::<u64>());
    map.insert("Fjee", read_one());
    map.insert("Dolladollabilljoll", read_one());

    let (output, _fee) = map.iter().min_by_key(|&(_key, value)| value).unwrap();
    println!("{}", output);
}
