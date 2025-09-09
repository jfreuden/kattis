use super::*;


fn run_scenario(mattresses: u32, nights: u32, penalty: u32, pea_location: u32) -> u32 {
    println!("{} {} {} ({})", mattresses, nights, penalty, pea_location);

    let mut scenario = Scenario::new(
        mattresses,
        nights,
        penalty,
        std::io::stdin().lock(),
        std::io::stdout(),
    );

    while scenario.configuration.mattresses > 1 && scenario.configuration.nights > 0 {
        scenario.autonomous_step(pea_location);
    }
    println!("! {}", scenario.configuration.offset);

    assert_eq!(pea_location, scenario.configuration.offset);

    scenario.configuration.offset
}

#[test]
fn test_group1() {
    let [mattresses, nights, penalty]: [u32; 3] = [2, 2, 1];
    run_scenario(mattresses, nights, penalty, 0);
    run_scenario(mattresses, nights, penalty, 1);
}

#[test]
fn test_group2() {
    run_scenario(3,3,0, 0);
    run_scenario(3,3,0, 1);
    run_scenario(3,3,0, 2);

    run_scenario(1000,1000,365, 0);
    run_scenario(1000,1000,365, 1);
    run_scenario(1000,1000,365, 565);
    run_scenario(1000,1000,365, 998);
    run_scenario(1000,1000,365, 999);

    run_scenario(1000,1000,1000, 999);
}

#[test]
fn test_group3() {
    let mattresses = 6;
    let nights = 4;
    let penalty = 1;

    for mattress in 0..mattresses {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group4() {
    let mattresses = 100;
    let nights = 7;
    let penalty = 0;

    for mattress in 0..mattresses {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group5() {
    let mattresses = 100;
    let nights = 30;
    let penalty = 5;

    for mattress in 0..mattresses {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0100() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 0..100 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0200() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 100..200 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0300() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 200..300 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0400() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 300..400 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0500() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 400..500 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0600() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 500..600 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0700() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 600..700 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0800() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 700..800 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_0900() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 800..900 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}

#[test]
fn test_group6_1000() {
    let [mattresses, nights] = [1000, 72];
    let penalty = 31;
    for mattress in 900..1000 {
        run_scenario(mattresses, nights, penalty, mattress);
    }
}
