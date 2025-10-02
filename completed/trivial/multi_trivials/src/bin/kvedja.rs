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
    println!("Kvedja,\n{}", read_str());
}
