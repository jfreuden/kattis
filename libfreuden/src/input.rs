#[allow(unused)]
#[macro_export]
macro_rules! kattis_struct {
    ($name:ident { $($field_name:ident : $field_type:ty),* }) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $name {
            $($field_name : $field_type),*
        }
        impl std::str::FromStr for $name {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut iter = s.split(' ');
                Ok($name {
                    $(
                        $field_name: iter.next().unwrap().parse::<$field_type>().map_err(|_| "parse error")?
                    ),*
                })
            }
        }
    };}
pub use kattis_struct;

pub fn read_vec_source<T: std::str::FromStr, R: std::io::Read>(
    buf_reader: &mut std::io::BufReader<R>,
) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    use std::io::BufRead;
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

pub fn read_str() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

pub fn read_one<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

pub fn read_vec<T: std::str::FromStr>() -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

pub fn read_array<T: std::str::FromStr + std::fmt::Debug, const K: usize>() -> [T; K]
where
    T::Err: std::fmt::Debug,
{
    read_vec::<T>().try_into().unwrap()
}
