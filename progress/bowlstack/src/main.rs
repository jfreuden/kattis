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

#[derive(Debug, Copy, Clone)]
struct Bowl {
    height: BowlInt,
    bottom_radius: BowlInt,
    top_radius: BowlInt,
}

impl From<[BowlInt; 3]> for Bowl {
    fn from([height, bottom_radius, top_radius]: [BowlInt; 3]) -> Self {
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
        BowlInt::cmp(&a.bottom_radius, &b.bottom_radius)
            .then_with(|| BowlInt::cmp(&a.height, &b.height))
    })
};
#[allow(dead_code)]
const CMP_SHALLOWER: fn(&Bowl, &Bowl) -> core::cmp::Ordering = |a, b| {
    CMP_STEEPER(b, a)
};

impl Solvable<&mut Vec<Bowl>> for &mut Vec<Bowl> {
    fn solve(bowls: &mut Vec<Bowl>) -> BowlFloat {
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

            // is it an optimization that p22y > p21y and that p12y > p11y?
            let p11y = BowlFloat::default();
            let p12y = y(m1, p12x, BowlFloat::default());
            let b_floor = -y(m2, p21x, BowlFloat::default());
            
            let b2 = if r2 <= r1 {
                b_floor // Floor case -------- had let p21y = BowlFloat::default(); here, unsure if needed
            } else if r2 >= R1 {
                b_floor + h2 // Ceiling case (or rim case, if you prefer)
            } else if m2 >= m1 {
                if m2 == m1 {
                    BowlFloat::default() // Special case: for identical slopes, b_2 is zero
                } else {
                    b(m1, m2, p21x) // Steeper only allows P_21 case
                }
            } else if m2 < m1 { // Shallow could be P_12 case or P_22 case
                if R2 > R1 {
                    b(m1, m2, p12x) // P_12 case
                } else {
                    b(m1, m2, p22x) // P_22 case
                }
            } else {
                panic!("Holy shit! We are all gonna die, this case shouldn't happen, holy fucking hell, someone take a look at these demon bowls and tell me what's wrong! {:?}, {:?}", bottom, top)
            };

            let p21y = y(m2, p21x, b2);
            let p22y = y(m2, p22x, b2);

            // Do I even need to check that P22 is higher than P12?
            // In theory, the gap is simply p21y, and the contact points were preselected.
            p21y
    }

        fn solve_stack_for_sort(bowls: &mut Vec<Bowl>, sort: fn(&Bowl, &Bowl) -> core::cmp::Ordering) -> f32 {
            bowls.sort_by(sort);
            let mut stackheight = bowls.first().unwrap().height as BowlFloat;
            let mut highestbottom = BowlFloat::default();
            for window in bowls.windows(2) {
                let [bottom, top]: [Bowl; 2] = window.try_into().unwrap();
                let gap = find_gap(&bottom, &top);
                highestbottom += gap;
                stackheight = BowlFloat::max(highestbottom + top.height as BowlFloat, stackheight);
            }
            stackheight
        }

        let shallower = solve_stack_for_sort(bowls, CMP_SHALLOWER);
        let steeper = solve_stack_for_sort(bowls, CMP_STEEPER);
        BowlFloat::min(shallower, steeper)
    }
}

fn main() {
    let test_cases: u32 = read_one();
    let mut case_bowls: Vec<Vec<Bowl>> = Vec::with_capacity(test_cases as usize);
    for _ in 0..test_cases {
        let num_bowls: u32 = read_one();
        let mut bowls: Vec<Bowl> = Vec::with_capacity(num_bowls as usize);

        for _ in 0..num_bowls {
            bowls.push(Bowl::from(read_array()))
        }
        case_bowls.push(bowls);
    }

    for mut bowls in case_bowls {
        let stackheight = solve(&mut bowls);
        println!("{:?}", stackheight);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_kattis_example() {
        let mut case_bowls: Vec<Vec<Bowl>> = vec![
            vec![
                Bowl::from([60, 20, 30]),
                Bowl::from([40, 10, 50]),
            ],
            vec![
                Bowl::from([50, 30, 80]),
                Bowl::from([35, 25, 70]),
                Bowl::from([40, 10, 90]),
            ]
        ];

        let answers = case_bowls.iter_mut().map(solve).collect::<Vec<f32>>();
        assert_eq!(answers, vec![70., 55.])
    }

    #[test]
    fn test_solve_nested_sameslope() {
        let bowls = vec![
            Bowl::from([30, 10, 40]),
            Bowl::from([10, 12, 22]),
            Bowl::from([10, 14, 24]),
        ];

        for bowl in &bowls {
            println!("Bowl: {:?}, slope: {:?}", bowl, bowl.slope());
        }

        assert_eq!(solve(bowls), 30.)
    }
}
