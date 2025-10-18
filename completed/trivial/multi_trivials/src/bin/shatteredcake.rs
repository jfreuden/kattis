use std::time::Instant;

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
    // let start = Instant::now();

    let mut input = Input::new();
    let cake_width = input.next();
    let cake_pieces: usize = input.next();

    let mut total_area: usize = 0;

    for _ in 0..cake_pieces {
        let shard_width: usize = input.next();
        let shard_height: usize = input.next();
        total_area += shard_width * shard_height;
    }

    println!("{}", total_area.div_euclid(cake_width));

    std::process::exit(0);
    // let wait_until = start + Instant::now().duration_since(start) * 50;
    // while Instant::now() < wait_until {}
}
