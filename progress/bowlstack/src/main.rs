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
type BowlFloat = f64;

#[derive(Debug, Copy, Clone)]
struct Bowl {
    height: BowlFloat,
    bottom_radius: BowlFloat,
    top_radius: BowlFloat,
}

impl From<[BowlInt; 3]> for Bowl {
    fn from([height, bottom_radius, top_radius]: [BowlInt; 3]) -> Self {
        Bowl {
            height: height as BowlFloat,
            bottom_radius: bottom_radius as BowlFloat,
            top_radius: top_radius as BowlFloat,
        }
    }
}

impl From<[BowlFloat; 3]> for Bowl {
    fn from([height, bottom_radius, top_radius]: [BowlFloat; 3]) -> Self {
        Bowl {
            height,
            bottom_radius,
            top_radius,
        }
    }
}

impl Bowl {
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
        b_floor + h2 // Ceiling case (or rim case, if you prefer)
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
    
    // TODO: if a bowl is so tiny that it could sit inside the final bowl, it doesn't matter what it's slope is
    
    let mut stackheight = bowls.first().unwrap().height as BowlFloat;
    let mut highestbottom = BowlFloat::default();
    let mut highestbowl = bowls.first().unwrap().clone();
    for window in bowls.windows(2) {
        let [bottom, top]: [Bowl; 2] = window.try_into().unwrap();
        let gap_bottom = find_gap(&bottom, &top);

        let synth_bowl = Bowl::from([
            stackheight - highestbottom,
            bottom.bottom_radius,
            highestbowl.top_radius,
        ]);

        let gap_highest = find_gap(&synth_bowl, &top);
        highestbottom += BowlFloat::max(gap_bottom, gap_highest);

        let candidate = highestbottom + top.height as BowlFloat;
        if candidate > stackheight {
            stackheight = candidate;
            highestbowl = top.clone();
        }

        stackheight = BowlFloat::max(candidate, stackheight);
    }
    stackheight
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
        let mut rims: Vec<(BowlFloat, &Bowl)> = vec![(BowlFloat::default(), bowlstack.first().unwrap())];

        for top in bowlstack {
            // compare this top against all the bowls in the stack
            let thisbottom = rims.iter().map(|&(floor, otherbottom)| {
                find_gap(&otherbottom, &top) + floor
            }).reduce(BowlFloat::max).unwrap();
            
            rims.push((thisbottom, &top));
        }
        rims.iter().map(|&(floor, &bowl)| floor + bowl.height).reduce(BowlFloat::max).unwrap()
    });
    all_stacksizes.into_iter().reduce(BowlFloat::min).unwrap()
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

    #[test]
    fn test_solve_kattis_example() {
        let mut case_bowls: Vec<Vec<Bowl>> = vec![
            vec![Bowl::from([60, 20, 30]), Bowl::from([40, 10, 50])],
            vec![
                Bowl::from([50, 30, 80]),
                Bowl::from([35, 25, 70]),
                Bowl::from([40, 10, 90]),
            ],
        ];

        let answers = case_bowls.iter_mut().map(brute_solve).collect::<Vec<BowlFloat>>();
        assert_eq!(answers, vec![70., 55.])
    }

    #[test]
    fn test_solve_nested_sameslope() {
        let mut bowls = vec![
            Bowl::from([30, 10, 40]),
            Bowl::from([10, 12, 22]),
            Bowl::from([10, 14, 24]),
        ];

        for bowl in &bowls {
            println!("Bowl: {:?}, slope: {:?}", bowl, bowl.slope());
        }

        assert_eq!(brute_solve(&mut bowls), 30.)
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

        assert_eq!(find_gap(&bowl, &plate), plate.height as BowlFloat);
    }

    #[test]
    fn test_gap_contact_lower_rim() {
        // P_12 case
        let bottom = Bowl::from([1, 1, 70]);
        let top = Bowl::from([1, 1, 9]);

        assert_eq!(find_gap(&bottom, &top), BowlFloat::default());

        assert_eq!(solve_stack_for_sort(&mut vec![bottom, top], CMP_STEEPER), bottom.height as BowlFloat);
        assert!(solve_stack_for_sort(&mut vec![bottom, top], CMP_SHALLOWER) >  bottom.height as BowlFloat);
    }

    // #[test]
    // fn test_gap_contact_upper_base() {
    //     // P_21 case
    //     todo!()
    // }
    // 
    // #[test]
    // fn test_gap_contact_upper_rim() {
    //     // P_22 case
    //     todo!()
    // }
    
    fn test_solve_adversarial_tiny_troll_bowl() {
        // if a bowl is so tiny that it could sit inside the final bowl, it doesn't matter what it's slope is
        
        // tiny bowl with slope 4, plate with slope 2, bowl with slope 1 (don't put it on bottom)
        
        // tiny bowl with slope 1, plate with slope 2, bowl with slope 4
        
        // tiny bowl with slope 2, plate with slope 1, bowl with slope 4
        
        // 
        
        
        let mut case_bowls: Vec<Bowl> = vec![];
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

    // fn test_solve_same_heights() {
    //     let mut bowls = vec![
    //         Bowl::from([10, 2, 20]),
    //         Bowl::from([10, 10, 15]),
    //         Bowl::from([10, 12, 27]),
    //         Bowl::from([10, 14, 29]),
    //         Bowl::from([10, 30, 31]),
    //         Bowl::from([10, 7, 51]),
    //         Bowl::from([10, 9, 53]),
    //     ];
    // }
    
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
        
        assert_eq!(bowls.map_permutations(|bowlstack| {BowlFloat::default()}).len(), 362880)
    }
    
    #[test]
    fn test_map_permutations_contents() {
        let mut bowls: Vec<BowlInt> = vec![1,2,3];
        let out = bowls.map_permutations(|x| {x.to_vec()});
        assert_eq!(out, vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,1,2], vec![3,2,1]]);
    }
}
