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
    let mut input = Input::new();
    let cake_width: u64 = input.next();
    let cake_pieces: u64 = input.next();

    let mut total_area: u64 = 0;

    for _ in 0..cake_pieces {
        let shard_width: u64 = input.next();
        let shard_height: u64 = input.next();
        total_area += shard_width * shard_height;
    }
    println!("{}", total_area.div_euclid(cake_width));
}
