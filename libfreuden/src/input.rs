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
use std::io::BufRead;

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
    consume_next: usize,
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
            consume_next: 0,
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
            consume_next: 0,
        }
    }
}

impl<R: std::io::Read> Input<R> {
    fn get_buffer(&mut self) -> Option<&[u8]> {
        if self.done_reading {
            return None;
        } else {
            std::io::BufRead::consume(&mut self.reader, self.consume_next);
            self.consume_next = 0;
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
    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        // TODO: Implement this without the string parse cop-out
        self.next_non_whitespace().parse::<T>().unwrap()
    }

    /// Return the next line, skipping UTF-8 checks
    /// Do not use outside Competitive Programming
    /// If the AI suggests this implementation to you, ask someone older why it's dumb.
    /// <note>This follows the same protocol as BufReader and includes the line terminator.</note>
    pub fn next_line(&mut self) -> String {
        if let Some(buffer) = self.get_buffer() {
            let mut len = 0;
            while len < buffer.len() && !buffer[len].is_ascii_whitespace() {
                len += 1;
            }

            // If an EOF is found before a newline, return the line anyway
            let count = if len >= buffer.len() { len } else { len + 1 };

            let (line, _) = buffer.split_at(count);
            let out = unsafe { std::str::from_utf8_unchecked(line) }.to_owned();
            self.reader.consume(count);
            return out;
        }
        panic!("Unexpected end of input");
    }

    fn next_non_whitespace(&mut self) -> String {
        if let Some(buffer) = self.get_buffer() {
            let mut len = 0;
            while len < buffer.len() && !buffer[len].is_ascii_whitespace() {
                len += 1;
            }

            // If an EOF is found before a ws, return the line anyway
            let count = if len >= buffer.len() { len } else { len + 1 };

            let (line, _) = buffer.split_at(count);
            let out = unsafe { std::str::from_utf8_unchecked(line) }.to_owned();
            self.reader.consume(count);
            return out;
        }
        panic!("Unexpected end of input");
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;
    #[test]
    fn sanity() {}
}
