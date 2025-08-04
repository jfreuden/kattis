/// Bowlstack is a "Hard" (6.1 point) problem on Kattis. https://open.kattis.com/problems/bowlstack
fn read_one<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

fn try_read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> Result<[T; K], E>
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    read_vec::<T>().try_into()
}

fn read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> [T; K]
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    try_read_array().unwrap()
}

type BowlInt = u32;
type BowlFloat = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Bowl {
    height: BowlFloat,
    bottom_radius: BowlFloat,
    top_radius: BowlFloat,
    label: Option<char>,
}

impl From<[BowlInt; 3]> for Bowl {
    fn from([height, bottom_radius, top_radius]: [BowlInt; 3]) -> Self {
        Bowl {
            height: height as BowlFloat,
            bottom_radius: bottom_radius as BowlFloat,
            top_radius: top_radius as BowlFloat,
            label: None,
        }
    }
}

impl From<[BowlFloat; 3]> for Bowl {
    fn from([height, bottom_radius, top_radius]: [BowlFloat; 3]) -> Self {
        Bowl {
            height,
            bottom_radius,
            top_radius,
            label: None,
        }
    }
}

impl Bowl {
    fn new<T>(height: T, bottom_radius: T, top_radius: T) -> Self
    where
        f64: From<T>,
    {
        Bowl {
            height: f64::from(height) as BowlFloat,
            bottom_radius: f64::from(bottom_radius) as BowlFloat,
            top_radius: f64::from(top_radius) as BowlFloat,
            label: None,
        }
    }

    fn new_labelled<T>(height: T, bottom_radius: T, top_radius: T, label: char) -> Self
    where
        f64: From<T>,
    {
        Bowl {
            height: f64::from(height) as BowlFloat,
            bottom_radius: f64::from(bottom_radius) as BowlFloat,
            top_radius: f64::from(top_radius) as BowlFloat,
            label: Some(label),
        }
    }
    
    fn slope(&self) -> BowlFloat {
        self.height as BowlFloat / (self.top_radius - self.bottom_radius) as BowlFloat
    }
}

trait Solvable<T> {
    fn solve(bowls: T) -> BowlFloat;
}

fn solve<T: Solvable<T>>(value: T) -> BowlFloat {
    T::solve(value)
}

impl Solvable<Vec<Bowl>> for Vec<Bowl> {
    fn solve(bowls: Vec<Bowl>) -> BowlFloat {
        let mut my_bowls = bowls.clone();
        solve(&mut my_bowls)
    }
}

#[allow(dead_code)]
const CMP_STEEPER: fn(&Bowl, &Bowl) -> core::cmp::Ordering = |a, b| {
    BowlFloat::total_cmp(&a.slope(), &b.slope()).then_with(|| {
        BowlFloat::total_cmp(&a.bottom_radius, &b.bottom_radius)
            .then_with(|| BowlFloat::total_cmp(&a.height, &b.height))
    })
};
#[allow(dead_code)]
const CMP_SHALLOWER: fn(&Bowl, &Bowl) -> core::cmp::Ordering = |a, b| CMP_STEEPER(b, a);

#[inline]
fn y(m: BowlFloat, x: BowlFloat, b: BowlFloat) -> BowlFloat {
    m * x + b
}

#[inline]
fn b(m1: BowlFloat, m2: BowlFloat, x: BowlFloat) -> BowlFloat {
    (m1 - m2) * x
}

fn find_gap(bottom: &Bowl, top: &Bowl) -> BowlFloat {
    let h1 = bottom.height as BowlFloat;
    let h2 = top.height as BowlFloat;
    let r1 = bottom.bottom_radius as BowlFloat;
    let r2 = top.bottom_radius as BowlFloat;
    let R1 = bottom.top_radius as BowlFloat;
    let R2 = top.top_radius as BowlFloat;
    let m1 = bottom.slope();
    let m2 = top.slope();

    let p11x = BowlFloat::default();
    let p12x = R1 - r1;
    let p21x = r2 - r1;
    let p22x = R2 - r1;

    let p11y = BowlFloat::default();
    let p12y = y(m1, p12x, BowlFloat::default());
    let b_floor = -y(m2, p21x, BowlFloat::default());

    let b2 = if r2 <= r1 && y(m2, p22x, b_floor) >= y(m1, p22x, BowlFloat::default()) {
        b_floor // Floor case -------- had let p21y = BowlFloat::default(); here, unsure if needed
    } else if r2 >= R1 {
        b_floor + h1 // Ceiling case (or rim case, if you prefer)
    } else if m2 >= m1 {
        if m2 == m1 {
            BowlFloat::default() // Special case: for identical slopes, b_2 is zero
        } else {
            b(m1, m2, p21x) // Steeper only allows P_21 case
        }
    } else if m2 < m1 {
        // Shallow could be P_12 case or P_22 case
        if R2 > R1 {
            b(m1, m2, p12x) // P_12 case
        } else {
            b(m1, m2, p22x) // P_22 case
        }
    } else {
        panic!(
            "Holy shit! We are all gonna die, this case shouldn't happen, holy fucking hell, someone take a look at these demon bowls and tell me what's wrong! {:?}, {:?}",
            bottom, top
        )
    };

    let p21y = y(m2, p21x, b2);
    let p22y = y(m2, p22x, b2);

    // Do I even need to check that P22 is higher than P12?
    // In theory, the gap is simply p21y, and the contact points were preselected.
    p21y
}

fn solve_stack_for_sort(
    bowls: &mut Vec<Bowl>,
    sort: fn(&Bowl, &Bowl) -> core::cmp::Ordering,
) -> BowlFloat {
    bowls.sort_by(sort);
    bowls.sort_by(|a, b| {
        BowlFloat::total_cmp(&a.top_radius, &b.bottom_radius).reverse()
    });
    let mut rims: Vec<(BowlFloat, &Bowl)> =
        vec![(BowlFloat::default(), bowls.first().unwrap())];

    for top in bowls.iter() {
        // compare this top against all the bowls in the stack
        let thisbottom = rims
            .iter()
            .map(|&(floor, otherbottom)| find_gap(&otherbottom, &top) + floor)
            .reduce(BowlFloat::max)
            .unwrap();

        rims.push((thisbottom, &top));
        rims.retain(|&(bowl_bottom, bowl)| {
            bowl_bottom + bowl.height >= thisbottom + top.height
        });
    }
    rims.iter()
        .map(|&(floor, &bowl)| floor + bowl.height)
        .reduce(BowlFloat::max)
        .unwrap()
}

trait Permutations<T> {
    fn map_permutations<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&[T]) -> R,
        T: Clone;
}

impl<T> Permutations<T> for Vec<T> {
    fn map_permutations<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&[T]) -> R,
        T: Clone,
    {
        fn permute<T, F, R>(items: &[T], prefix: &mut Vec<T>, results: &mut Vec<R>, f: &F)
        where
            F: Fn(&[T]) -> R,
            T: Clone,
        {
            if items.is_empty() {
                results.push(f(prefix));
            }
            for i in 0..items.len() {
                let mut remaining = items.to_vec();
                let item = remaining.remove(i);
                prefix.push(item);
                permute(&remaining, prefix, results, f);
                prefix.pop();
            }
        }

        let mut results = Vec::new();
        let mut prefix = Vec::new();
        permute(self, &mut prefix, &mut results, &f);
        results
    }
}

impl Solvable<&mut Vec<Bowl>> for &mut Vec<Bowl> {
    fn solve(bowls: &mut Vec<Bowl>) -> BowlFloat {
        let shallower = solve_stack_for_sort(bowls, CMP_SHALLOWER);
        let steeper = solve_stack_for_sort(bowls, CMP_STEEPER);
        BowlFloat::min(shallower, steeper)
    }
}

/// A reference implementation solver for generating test cases.
fn brute_solve(bowls: &mut Vec<Bowl>) -> BowlFloat {
    let all_stacksizes: Vec<BowlFloat> = bowls.map_permutations(|bowlstack| {
        let mut rims: Vec<(BowlFloat, &Bowl)> =
            vec![(BowlFloat::default(), bowlstack.first().unwrap())];

        for top in bowlstack {
            // compare this top against all the bowls in the stack
            let thisbottom = rims
                .iter()
                .map(|&(floor, otherbottom)| find_gap(&otherbottom, &top) + floor)
                .reduce(BowlFloat::max)
                .unwrap();

            rims.push((thisbottom, &top));
            rims.retain(|&(bowl_bottom, bowl)| {
                bowl_bottom + bowl.height >= thisbottom + top.height
            });
        }
        let output = rims.iter()
            .map(|&(floor, &bowl)| floor + bowl.height)
            .reduce(BowlFloat::max)
            .unwrap();
        if output == 81.818184 {
            println!("bf.label:    {:.2?}", bowlstack.iter().map(|x| x.label.unwrap()).collect::<Vec<char>>());
            println!("bf.slope:    {:.2?}", bowlstack.iter().map(|x| x.slope()).collect::<Vec<BowlFloat>>());
            println!("bf.height:   {:.2?}", bowlstack.iter().map(|x| x.height).collect::<Vec<BowlFloat>>());
            println!("bf.r:        {:.2?}", bowlstack.iter().map(|x| x.bottom_radius).collect::<Vec<BowlFloat>>());
            println!("bf.R:        {:.2?}", bowlstack.iter().map(|x| x.top_radius).collect::<Vec<BowlFloat>>());
            println!("br.floor:    {:.2?}", bowlstack.windows(2).filter(|&window| {
                let [top, bottom] = window else { todo!() };
                top.bottom_radius <= bottom.bottom_radius
            }).count());
            println!("br.ceil:     {:.2?}", bowlstack.windows(2).filter(|&window| {
                let [top, bottom] = window else { todo!() };
                top.bottom_radius >= bottom.top_radius
            }).count());
            println!("br.zerogap:  {:.2?}", bowlstack.windows(2).filter(|&window| {
                let [bottom, top] = window else { panic!() };
                find_gap(&bottom, &top) == BowlFloat::default()
            }).count());
        }
        output
    });
    let rval = all_stacksizes.into_iter().reduce(BowlFloat::min).unwrap();
    println!("{}", rval);
    rval
}

fn main() {
    let test_cases: u32 = read_one();
    let mut case_bowls: Vec<Vec<Bowl>> = Vec::with_capacity(test_cases as usize);
    for _ in 0..test_cases {
        let num_bowls: u32 = read_one();
        let mut bowls: Vec<Bowl> = Vec::with_capacity(num_bowls as usize);

        for _ in 0..num_bowls {
            bowls.push(Bowl::from(read_array::<BowlInt, 3, _>()))
        }
        case_bowls.push(bowls);
    }

    for mut bowls in case_bowls {
        let stackheight = brute_solve(&mut bowls).trunc() as u32;
        println!("{:?}", stackheight);
    }
}

#[cfg(test)]
mod bowlstack_tests {
    use super::*;
    const SELECTED_SOLVER: fn(&mut Vec<Bowl>) -> BowlFloat = |bowls| solve(bowls);

    #[test]
    fn test_solve_kattis_example() {
        let mut case_bowls: Vec<Vec<Bowl>> = vec![
            vec![
                Bowl::from([60, 20, 30]),
                Bowl::from([40, 10, 50])
            ],
            vec![
                Bowl::from([50, 30, 80]),
                Bowl::from([35, 25, 70]),
                Bowl::from([40, 10, 90]),
            ],
        ];

        let answers = case_bowls
            .iter_mut()
            .map(SELECTED_SOLVER)
            .collect::<Vec<BowlFloat>>();
        assert_eq!(answers, vec![70., 55.])
    }

    #[test]
    fn test_solve_nested_sameslope() {
        let mut bowls = vec![
            Bowl::from([30, 10, 40]),
            Bowl::from([10, 12, 22]),
            Bowl::from([10, 14, 24]),
        ];

        assert_eq!(SELECTED_SOLVER(&mut bowls), 30.)
    }

    #[test]
    fn test_gap_floor_case() {
        // P_11 case (although actually clamped to floor)
        let plate = Bowl::from([1, 100, 101]);
        let bowl = Bowl::from([9, 1, 9]);

        assert_eq!(find_gap(&plate, &bowl), BowlFloat::default());
    }

    #[test]
    fn test_gap_ceiling_case() {
        // Ceiling case when the base of the upper bowl is sitting on the rim
        let plate = Bowl::from([1, 100, 101]);
        let bowl = Bowl::from([9, 1, 9]);

        assert_eq!(find_gap(&bowl, &plate), bowl.height as BowlFloat);
    }

    #[test]
    fn test_gap_contact_lower_rim() {
        // P_12 case
        let bottom = Bowl::from([1, 1, 70]);
        let top = Bowl::from([1, 1, 9]);

        assert_eq!(find_gap(&bottom, &top), BowlFloat::default());
        assert!(find_gap(&top, &bottom) > BowlFloat::default());
    }

    #[test]
    fn test_gap_contact_upper_base() {
        // P_21 case -- when the bowl sits on it's base somewhere along the slope
        let bottom = Bowl::from([40, 10, 50]);
        let top = Bowl::from([60, 20, 30]);

        assert_eq!(find_gap(&bottom, &top), 10 as BowlFloat);
    }

    #[test]
    fn test_gap_contact_upper_rim() {
        // P_22 case -- when the bowl 'slips' down, and it's rim is on the slope of the lower bowl
        let bottom = Bowl::from([10, 1, 11]);
        assert_eq!(bottom.slope(), 1.0);
        let top = Bowl::from([1, 1, 6]);
        assert_eq!(top.slope(), 0.2);
        assert_eq!(find_gap(&bottom, &top), 4 as BowlFloat);
    }

    #[test]
    fn test_solve_nested_sameheight() {
        let mut bowls = vec![
            Bowl::new(30, 30, 40),
            Bowl::new(30, 29, 39),
            Bowl::new(30, 28, 38),
            Bowl::new(30, 27, 37),
            Bowl::new(30, 26, 36),
            Bowl::new(30, 25, 35),
        ];

        assert_eq!(SELECTED_SOLVER(&mut bowls), 30.)
    }


    #[allow(dead_code)]
    const CMP_STEEPER_BETTER: fn(&Bowl, &Bowl) -> core::cmp::Ordering = |a, b| {
        BowlFloat::total_cmp(&a.slope(), &b.slope()).then_with(|| {
            BowlFloat::total_cmp(&a.bottom_radius, &b.bottom_radius)
                .then_with(|| BowlFloat::total_cmp(&a.height, &b.height))
        })
    };
    #[allow(dead_code)]
    const CMP_SHALLOWER_BETTER: fn(&Bowl, &Bowl) -> core::cmp::Ordering = |a, b| CMP_STEEPER(b, a);

    #[test]
    fn test_sort_adversarial_tiny_troll_bowl(){
        // Address the troll bowl with a new sorter, tested here.
        // must not stack as [tinybowl, plate, bowl] or [bowl, plate, tinybowl]

        // Possible solution to this would be to pre-detect floor and ceiling cases.
        // I assume that floor cases are desired and ceiling cases aren't.
        // What's tricky is that this isn't something I can detect in the sorting stage (or is it?)
        // but rather once I have a collection that will be fed into the thingy
        let tiny_bowl = Bowl::new(4, 1, 2);
        assert_eq!(tiny_bowl.slope(), 4.0);
        let plate = Bowl::new(2, 400, 401);
        assert_eq!(plate.slope(), 2.0);
        let bowl = Bowl::new(20, 100, 120);
        assert_eq!(bowl.slope(), 1.0);

        let mut bowlstack = vec![tiny_bowl, plate, bowl];
        bowlstack.sort_by(CMP_STEEPER);
        assert_eq!(bowlstack, vec![bowl, plate, tiny_bowl]);
        bowlstack.sort_by(CMP_SHALLOWER);
        assert_eq!(bowlstack, vec![tiny_bowl, plate, bowl]);

        //////// REMOVE THIS SECTION //////////
        //////// REMOVE THIS SECTION //////////
        //////// REMOVE THIS SECTION //////////
        //////// REMOVE THIS SECTION //////////
        // Try sorting with a bubble on the condition where r2 >= R1
        bowlstack.sort_by(CMP_STEEPER);
        bowlstack.sort_by(|b, a| {
            BowlFloat::total_cmp(&a.top_radius, &b.bottom_radius)
        });
        println!("{:?}", bowlstack);
        assert_ne!(bowlstack, vec![bowl, plate, tiny_bowl]);
        assert_ne!(bowlstack, vec![tiny_bowl, plate, bowl]);

        bowlstack.sort_by(CMP_SHALLOWER);
        bowlstack.sort_by(|b, a| {
            BowlFloat::total_cmp(&a.top_radius, &b.bottom_radius)
        });
        println!("{:?}", bowlstack);
        assert_ne!(bowlstack, vec![bowl, plate, tiny_bowl]);
        assert_ne!(bowlstack, vec![tiny_bowl, plate, bowl]);
        //////// REMOVE THIS SECTION //////////
        //////// REMOVE THIS SECTION //////////
        //////// REMOVE THIS SECTION //////////
        //////// REMOVE THIS SECTION //////////


    }

    #[test]
    fn test_solve_adversarial_tiny_troll_bowl() {
        // if a bowl is so tiny that it could sit inside the final bowl, it doesn't matter what it's slope is

        // tiny bowl with slope 4, plate with slope 2, bowl with slope 1 (don't put it on bottom)
        let tiny_bowl = Bowl::new(4, 1, 2);
        assert_eq!(tiny_bowl.slope(), 4.0);
        let plate = Bowl::new(2, 400, 401);
        assert_eq!(plate.slope(), 2.0);
        let bowl = Bowl::new(20, 100, 120);
        assert_eq!(bowl.slope(), 1.0);

        assert_eq!(find_gap(&tiny_bowl, &plate), tiny_bowl.height as BowlFloat);
        assert_eq!(find_gap(&tiny_bowl, &bowl), tiny_bowl.height as BowlFloat);
        assert_eq!(find_gap(&plate, &tiny_bowl), BowlFloat::default());
        assert_eq!(find_gap(&plate, &bowl), BowlFloat::default());
        assert_eq!(find_gap(&bowl, &tiny_bowl), BowlFloat::default());
        assert_eq!(find_gap(&bowl, &plate), bowl.height as BowlFloat);

        let mut bowlstack = vec![tiny_bowl, plate, bowl];

        // The issue with this example is that the naive STEEPER and SHALLOWER comparisons
        // love to stack such that it is either of:
        // tinybowl, plate, bowl
        // bowl, plate, tinybowl
        println!("{:?}", bowlstack);
        bowlstack.sort_by(CMP_STEEPER);
        println!("{:?}", bowlstack);
        bowlstack.sort_by(CMP_SHALLOWER);
        println!("{:?}", bowlstack);
        assert_eq!(SELECTED_SOLVER(&mut bowlstack), 20.);


        // Possible solution to this would be to pre-detect floor and ceiling cases.
        // I assume that floor cases are desired and ceiling cases aren't.
        // What's tricky is that this isn't something I can detect in the sorting stage (or is it?)
        // but rather once I have a collection that will be fed into the thingy





        // tiny bowl with slope 1, plate with slope 2, bowl with slope 4

        // tiny bowl with slope 2, plate with slope 1, bowl with slope 4

        //

        let mut case_bowls: Vec<Bowl> = vec![];
    }

    #[test]
    fn test_solve_normal_stack_in_pots() {
        // Test a normal stack inside a couple of pots.
        let mut bowls = vec![
            Bowl::from([50, 30, 80]),
            Bowl::from([35, 25, 70]),
            Bowl::from([40, 10, 90]),
            Bowl::new(200, 100, 120),
            Bowl::new(100, 110, 130),
            Bowl::new(120, 45, 50),
        ];

        assert_eq!(SELECTED_SOLVER(&mut bowls), 200.)
    }

    #[test]
    fn test_solve_track_viable_rims() {
        // This one is hard to describe, but I have a pic on a sticky note.
        // Essentially just trying to catch errors when you only track the last bowl,
        // and the errors when you only track the tallest and last bowl.
        // Essentially the situation is one where a pair of plates rest along a slope,
        // but the slope they rest on belongs to an earlier bowl in the stack that is neither the
        // previous bowl nor the tallest, nor other bowls of interest.
        todo!()
    }
    
    #[test]
    fn test_solve_10_bowls() {
        let mut bowlstack = vec![
            Bowl::new_labelled(60, 20, 30, 'P'),
            Bowl::new_labelled(50, 30, 80, 'Q'),
            Bowl::new_labelled(40, 10, 90, 'R'),
            Bowl::new_labelled(40, 10, 50, 'S'),
            Bowl::new_labelled(15, 17, 60, 'T'),
            Bowl::new_labelled(19, 41, 70, 'U'),
            Bowl::new_labelled(9, 1, 9, 'V'),
            Bowl::new_labelled(30, 25, 35, 'W'),
            Bowl::new_labelled(10, 14, 29, 'X'),
            Bowl::new_labelled(10, 9, 53, 'Y'),
            Bowl::new_labelled(10, 1, 11, 'Z'),
            // Bowl::from([35, 25, 70]),
            // Bowl::from([50, 30, 80]),
            // Bowl::from([35, 25, 70]),
            // Bowl::from([40, 10, 90]),
            // Bowl::from([30, 10, 40]),
            // Bowl::from([10, 12, 22]),
            // Bowl::from([10, 14, 24]),
            // Bowl::from([1, 1, 9]),
            // Bowl::from([1, 100, 101]),
            // Bowl::new(30, 30, 40),
            // Bowl::new(30, 29, 39),
            // Bowl::new(30, 28, 38),
            // Bowl::new(30, 27, 37),
            // Bowl::new(30, 26, 36),
            // Bowl::new(4, 1, 2),
            // Bowl::new(2, 400, 401),
            // Bowl::new(20, 100, 120),
            // Bowl::from([10, 2, 20]),
            // Bowl::from([10, 10, 15]),
            // Bowl::from([10, 12, 27]),
            // Bowl::from([10, 30, 31]),
            // Bowl::from([10, 7, 51]),
            // Bowl::from([10, 11, 55]),
            // Bowl::from([10, 13, 57]),
        ];

        
        // let brtu = brute_solve(&mut bowlstack);
        // println!("Brute: {:?}", brtu);
        
        // println!("bf.label:    {:2.2?}", bowls.iter().enumerate().map(|(i, _)| {('A' as u8 + i as u8) as char}).collect::<Vec<char>>());
        // println!("bf.slope:    {:2.2?}", bowls.iter().map(|x| x.slope()).collect::<Vec<BowlFloat>>());
        // println!("bf.height:   {:.2?}", bowls.iter().map(|x| x.height).collect::<Vec<BowlFloat>>());
        // println!("bf.r:        {:.2?}", bowls.iter().map(|x| x.bottom_radius).collect::<Vec<BowlFloat>>());
        // println!("bf.R:        {:.2?}", bowls.iter().map(|x| x.top_radius).collect::<Vec<BowlFloat>>());
        // 
        // println!("\
        // bf.label:    ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K']\n\
        // bf.slope:    [6.00, 1.00, 0.50, 1.00, 0.20, 0.01, 1.12, 3.00, 0.67, 0.23, 1.00]\n\
        // bf.height:   [60.00, 50.00, 40.00, 40.00, 1.00, 1.00, 9.00, 30.00, 10.00, 10.00, 10.00]\n\
        // bf.r:        [20.00, 30.00, 10.00, 10.00, 1.00, 1.00, 1.00, 25.00, 14.00, 9.00, 1.00]\n\
        // bf.R:        [30.00, 80.00, 90.00, 50.00, 6.00, 70.00, 9.00, 35.00, 29.00, 53.00, 11.00]"
        // );
        
        // let result = SELECTED_SOLVER(&mut bowls).trunc();
        let result = solve_stack_for_sort(&mut bowlstack, CMP_SHALLOWER).trunc();

        println!("opti.label:  {:.2?}", bowlstack.iter().map(|x| x.label.unwrap()).collect::<Vec<char>>());
        println!("opti.slope:  {:.2?}", bowlstack.iter().map(|x| x.slope()).collect::<Vec<BowlFloat>>());
        println!("opti.height: {:.2?}", bowlstack.iter().map(|x| x.height).collect::<Vec<BowlFloat>>());
        println!("opti.r:      {:.2?}", bowlstack.iter().map(|x| x.bottom_radius).collect::<Vec<BowlFloat>>());
        println!("opti.R:      {:.2?}", bowlstack.iter().map(|x| x.top_radius).collect::<Vec<BowlFloat>>());
        println!("opti.floor:  {:.2?}", bowlstack.windows(2).filter(|&window| {
            let [top, bottom] = window else { todo!() };
            top.bottom_radius <= bottom.bottom_radius
        }).count());
        println!("opti.ceil:   {:.2?}", bowlstack.windows(2).filter(|&window| {
            let [top, bottom] = window else { todo!() };
            top.bottom_radius >= bottom.top_radius
        }).count());
        println!("opti.zerogap:{:.2?}", bowlstack.windows(2).filter(|&window| {
            let [bottom, top] = window else { panic!() };
            find_gap(&bottom, &top) == BowlFloat::default()
        }).count());
        
        assert_eq!(result.trunc(), 81.0);
    }

    #[test]
    fn test_solve_nested_kattis_example() {
        let mut case_bowls: Vec<Bowl> = vec![
            Bowl::from([60, 20, 30]),
            Bowl::from([40, 10, 50]),
            Bowl::from([50, 30, 80]),
            Bowl::from([35, 25, 70]),
            Bowl::from([40, 10, 90]),
        ];

        let answers = brute_solve(&mut case_bowls).trunc() as u32;
        assert_eq!(answers, 81) // Wrong answer says it's 87.5 (outputting 87)  (brute_solve says 81)
    }

    #[test]
    fn test_map_permutations_len() {
        let mut bowls = vec![
            Bowl::from([10, 2, 20]),
            Bowl::from([10, 10, 15]),
            Bowl::from([10, 12, 27]),
            Bowl::from([10, 14, 29]),
            Bowl::from([10, 30, 31]),
            Bowl::from([10, 7, 51]),
            Bowl::from([10, 9, 53]),
            Bowl::from([10, 11, 55]),
            Bowl::from([10, 13, 57]),
        ];

        assert_eq!(
            bowls
                .map_permutations(|bowlstack| { BowlFloat::default() })
                .len(),
            362880
        )
    }

    #[test]
    fn test_map_permutations_contents() {
        let mut bowls: Vec<BowlInt> = vec![1, 2, 3];
        let out = bowls.map_permutations(|x| x.to_vec());
        assert_eq!(
            out,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
