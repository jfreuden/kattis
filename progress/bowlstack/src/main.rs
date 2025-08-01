/// Bowlstack is a "Hard" (6.1 point) problem on Kattis. https://open.kattis.com/problems/bowlstack

fn read_str() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

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

struct Bowl {
    height: u32,
    bottom_radius: u32,
    top_radius: u32,
}

impl From<[u32; 3]> for Bowl {
    fn from([height, bottom_radius, top_radius]: [u32; 3]) -> Self {
        Bowl {
            height,
            bottom_radius,
            top_radius,
        }
    }
}

impl Bowl {
    fn slope(&self) -> f32 {
        self.height as f32 / (self.top_radius - self.bottom_radius) as f32
    }
}

fn main() {
    let test_cases: u32 = read_one();

    for _ in 0..test_cases {
        let num_bowls: u32 = read_one();
        let mut bowls: Vec<Bowl> = Vec::with_capacity(num_bowls as usize);

        for _ in 0..num_bowls {
            bowls.push(Bowl::from(read_array()))
        }

        // TODO: solve the basic trig to determine the stacking height
        // sort by slope, then check whether increasing or decreasing slope is the answer
        // sorting by slope is easy
        // getting the final heights will require contact math
        // but it's "simply" find two circles that are congruent (or such that the smaller bowl sits inside)
        //
        // question:
        // does increasing slope always map to increasing radii and vice versa or no?
        // I would assume not.
        //
        // the bottom is always smaller than the top, at least.

    }

    println!("Hello, world!");
}
