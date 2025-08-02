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
        fn subsolve(bowls: &mut Vec<Bowl>) -> f32 {
            let mut stackheight = BowlFloat::default();
            let mut highestbottom = BowlFloat::default();
            for window in bowls.windows(2) {
                let [bottom, top]: [Bowl; 2] = window.try_into().unwrap();
                let m1 = bottom.slope();
                let m2 = top.slope();
                let r1 = bottom.bottom_radius as BowlFloat;
                let r2 = top.bottom_radius as BowlFloat;

                let gap = (m1 * r1 - m2 * r2) - m1 * r1 - m2 * r2; // THIS IS WRONG
                highestbottom += gap;
                stackheight = BowlFloat::max(highestbottom + top.height as BowlFloat, stackheight);
            }
            stackheight
        }

        bowls.sort_by(CMP_SHALLOWER);
        let shallower = subsolve(bowls);
        bowls.sort_by(CMP_STEEPER);
        let steeper = subsolve(bowls);
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
