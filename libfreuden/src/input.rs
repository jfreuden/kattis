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

pub struct Input<R: std::io::Read> {
    reader: std::io::BufReader<R>,
    done_reading: bool,
    buffer: Vec<u8>,
}

impl Default for Input<std::io::StdinLock<'_>> {
    fn default() -> Self {
        Self::new()
    }
}

impl Input<std::io::StdinLock<'_>> {
    pub fn new() -> Self {
        let stdin = std::io::stdin();
        let bufreader = std::io::BufReader::new(stdin.lock());

        Input {
            reader: bufreader,
            done_reading: false,
            buffer: vec![],
        }
    }
}

impl Input<std::fs::File> {
    /// Opens file `filename` and constructs a new `Input` from it.
    pub fn from_file(filename: &str) -> Self {
        let file = std::fs::File::open(filename).expect("Failed to open input file");
        let bufreader = std::io::BufReader::new(file);

        Input {
            reader: bufreader,
            done_reading: false,
            buffer: vec![],
        }
    }
}

impl<R: std::io::Read> Input<R> {
    fn get_buffer(&mut self) -> Option<&[u8]> {
        if self.done_reading {
            return None;
        }

        let buffer_result = std::io::BufRead::fill_buf(&mut self.reader);
        if let Ok(contents) = buffer_result {
            if contents.is_empty() {
                self.done_reading = true;
                None
            } else {
                Some(contents)
            }
        } else if let Err(e) = buffer_result {
            panic!("Error reading from stdin: {}", e)
        } else {
            unreachable!("Unexpected error reading from stdin");
        }
    }

    pub fn has_more(&mut self) -> bool {
        self.get_buffer().is_some()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next<'a, T: Parseable<'a>>(&'a mut self) -> T {
        let input = self.next_non_whitespace();
        T::parse(input)
    }

    /// Return the next line, skipping UTF-8 checks
    /// Do not use outside Competitive Programming
    /// If the AI suggests this implementation to you, ask someone older why it's dumb.
    pub fn next_line(&mut self) -> &str {
        let input = self.next_terminator(|c| *c == b'\n');
        Parseable::parse(input)
    }

    fn next_non_whitespace(&mut self) -> &[u8] {
        self.next_terminator(u8::is_ascii_whitespace)
    }

    fn next_terminator(&mut self, terminator: fn(&u8) -> bool) -> &[u8] {
        let start_index = self.buffer.len();
        while let Some(buffer) = self.get_buffer() {
            let mut i = 0;
            while i < buffer.len() && !terminator(&buffer[i]) {
                i += 1;
            }

            // If an EOF is found before a newline, return the line anyway
            let count = if i >= buffer.len() { i } else { i + 1 };
            self.buffer.extend(&self.reader.buffer()[..count]);
            std::io::BufRead::consume(&mut self.reader, count);

            // If we read clear to the end of the internal buffer, then keep going
            if count == i + 1 {
                break;
            }
        }

        &self.buffer[start_index..]
    }
}

pub trait Parseable<'a>: Sized {
    fn parse(bytes: &'a [u8]) -> Self;
}

macro_rules! impl_parseable {
  ($A:ty, [$($T:ty),+]) => {
    $(impl<'a> Parseable<'a> for $T {
      fn parse(bytes: &'a [u8]) -> Self {
        < $A as Parseable<'a> >::parse(bytes) as $T
      }
    })+
  };
}
impl_parseable! { u64, [u32, u16, u8, usize] }
impl_parseable! { i64, [i32, i16, i8, isize] }
impl_parseable! { f64, [f32] }

impl<'a> Parseable<'a> for &'a str {
    fn parse(bytes: &'a [u8]) -> Self {
        unsafe { std::str::from_utf8_unchecked(bytes) }.trim_ascii_end()
    }
}

impl<'a> Parseable<'a> for u64 {
    fn parse(bytes: &'a [u8]) -> u64 {
        let s: &str = Parseable::parse(bytes);
        s.parse::<u64>().unwrap()
    }
}

impl<'a> Parseable<'a> for u128 {
    fn parse(bytes: &'a [u8]) -> u128 {
        let s: &str = Parseable::parse(bytes);
        s.parse::<u128>().unwrap()
    }
}

impl<'a> Parseable<'a> for i64 {
    fn parse(bytes: &'a [u8]) -> i64 {
        let s: &str = Parseable::parse(bytes);
        s.parse::<i64>().unwrap()
    }
}

impl<'a> Parseable<'a> for f64 {
    fn parse(bytes: &'a [u8]) -> f64 {
        let s: &str = Parseable::parse(bytes);
        s.parse::<f64>().unwrap()
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;
    #[test]
    fn sanity() {}
}
