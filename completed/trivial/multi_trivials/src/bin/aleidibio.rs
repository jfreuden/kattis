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
    let hannes_to_arnar = read_one::<u64>();
    let arnar_to_cinema = read_one::<u64>();
    let time_of_film = read_one::<u64>();

    println!("{}", time_of_film - (hannes_to_arnar + arnar_to_cinema));
}
