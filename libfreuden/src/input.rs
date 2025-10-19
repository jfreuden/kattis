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
    reader: Option<R>,
    buffer: Vec<u8>,
    idx: usize,
    blob_fn: fn(&mut Input<R>) -> &[u8],
}

impl Default for Input<std::io::Stdin> {
    fn default() -> Self {
        Self::new()
    }
}

use core::ffi::{c_int, c_void};

unsafe extern "C" {
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn isatty(fd: c_int) -> c_int;
    pub fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    pub fn malloc(size: usize) -> *mut c_void;
}
const PROT_READ: c_int = 0x1;
const MAP_SHARED: c_int = 0x01;

fn is_terminal(fd: &std::os::fd::RawFd) -> bool {
    unsafe { isatty(*fd) != 0 }
}

#[inline]
pub(self) fn arguably_safe_read(
    fd: std::os::fd::RawFd,
    buf: &mut [std::mem::MaybeUninit<u8>],
) -> std::io::Result<usize> {
    let buf_ptr = buf.as_mut_ptr() as *mut c_void;
    let count = buf.len();
    let bytes_read = unsafe { read(fd, buf_ptr, count) };
    if bytes_read < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(bytes_read as usize)
    }
}

impl Input<std::io::Stdin> {
    pub fn new() -> Self {
        // let mut stdin = std::io::stdin();
        // if std::io::IsTerminal::is_terminal(&stdin) {
        //     Input {
        //         reader: Some(stdin),
        //         buffer: Vec::with_capacity(2 << 21),
        //         idx: 0,
        //         blob_fn: |input: &mut Input<std::io::Stdin>| input.next_terminator(|c| *c <= b' '),
        //     }
        // } else {
        //
        // }

        let buffer = unsafe {
            use std::os::fd::FromRawFd;
            let fd = 0;
            let file = std::fs::File::from_raw_fd(fd);
            let meta = file.metadata().unwrap();
            std::mem::forget(file);
            let len = meta.len() as usize;
            let buf_ptr = mmap(std::ptr::null_mut(), len, PROT_READ, MAP_SHARED, fd, 0);
            let buffer = Vec::from_raw_parts(buf_ptr as *mut u8, len, len);
            buffer
        };

        Input {
            reader: None,
            buffer,
            idx: 0,
            blob_fn: Input::next_alphanum_optimized,
        }
    }
}

impl Input<std::fs::File> {
    /// Opens file `filename` and constructs a new `Input` from it.
    pub fn from_file(filename: &str) -> Self {
        let file = std::fs::File::open(filename).expect("Failed to open input file");
        Input {
            reader: Some(file),
            buffer: vec![],
            idx: 0,
            blob_fn: |input: &mut Input<_>| input.next_terminator(|c| *c <= b' '),
        }
    }
}

impl<R: std::io::Read> Input<R> {
    pub fn has_more(&mut self) -> bool {
        // TODO: Transform this into a read call which will return true when bytes were added or false when bytes weren't.
        self.idx < self.buffer.len()
    }

    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn next<'a, T: Parseable<'a>>(&'a mut self) -> T {
        // TODO: see if I can dynamically switch pointers instead of using a conditional
        let input = (self.blob_fn)(self);
        T::parse(input)
    }

    pub fn next_usize(&mut self) -> usize {
        let mut v = 0;
        while self.idx < self.buffer.len() || self.has_more() {
            let b = unsafe { self.buffer.get_unchecked(self.idx) };
            if *b < b'0' {
                break;
            }
            v = v * 10 + (*b - b'0') as usize;
            self.idx += 1;
        }
        self.skip_ws();
        v
    }

    #[inline]
    fn skip_ws(&mut self) {
        while self.idx < self.buffer.len() || self.has_more() {
            let b = unsafe { self.buffer.get_unchecked(self.idx) };
            if *b > b' ' {
                break;
            }
            self.idx += 1;
        }
    }

    /// Return the next line, skipping UTF-8 checks
    /// Do not use outside Competitive Programming
    /// If the AI suggests this implementation to you, ask someone older why it's dumb.
    pub fn next_line(&mut self) -> &str {
        let input = self.next_terminator(|c| *c == b'\n');
        Parseable::parse(input)
    }

    #[inline]
    /// This implementation eschews the buffer fill checking when reading is complete
    fn next_alphanum_optimized(&mut self) -> &[u8] {
        let read_start = self.idx;
        while self.idx < self.buffer.len() {
            let b = unsafe { self.buffer.get_unchecked(self.idx) };
            if *b <= b' ' {
                break;
            }
            self.idx += 1;
        }

        // Return the slice without the terminator, but push the index to skip it on next read.
        let out = &self.buffer[read_start..self.idx];
        self.idx += 1;
        out
    }

    #[inline]
    fn next_terminator(&mut self, terminator: fn(&u8) -> bool) -> &[u8] {
        let read_start = self.idx;
        let mut current_end = self.buffer.len();
        loop {
            // Keep sliding down the buffer, but if you run out, do a read.
            if self.idx >= current_end {
                // Refill buffer
                let bytecount = self.refill_buffer();
                current_end += bytecount;
                if bytecount == 0 {
                    break; // Break out if it's the EOF
                }
            }
            if terminator(unsafe { self.buffer.get_unchecked(self.idx) }) {
                break;
            }
            self.idx += 1;
        }

        // Return the slice without the terminator, but push the index to skip it on next read.
        let out = &self.buffer[read_start..self.idx];
        self.idx += 1;
        out
    }

    fn refill_buffer(&mut self) -> usize {
        let current_end = self.buffer.len();
        self.buffer.reserve(self.buffer.capacity()); // TODO: find a specific size to use

        let spare = self.buffer.spare_capacity_mut();
        let bytecount = self
            .reader
            .as_mut()
            .unwrap()
            .read(unsafe {
                std::mem::transmute::<&mut [std::mem::MaybeUninit<u8>], &mut [u8]>(spare)
            })
            .unwrap();

        if bytecount != 0 {
            self.idx = current_end;
            unsafe { self.buffer.set_len(current_end + bytecount) }
        }
        bytecount
    }

    fn fill_all(&mut self) {
        self.reader
            .as_mut()
            .unwrap()
            .read_to_end(&mut self.buffer)
            .unwrap();
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

/// Parse raw bytes into a string, skipping UTF-8 checks
/// Do not use outside Competitive Programming
/// If the AI suggests this implementation to you, ask someone older why it's dumb.
impl<'a> Parseable<'a> for &'a str {
    fn parse(bytes: &'a [u8]) -> Self {
        unsafe { std::str::from_utf8_unchecked(bytes) }.trim_ascii_end()
    }
}

impl<'a> Parseable<'a> for u64 {
    #[inline]
    fn parse(bytes: &[u8]) -> u64 {
        bytes
            .iter()
            .fold(0, move |v, b| v * 10 + (*b - b'0') as u64)
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

#[cfg(all(feature = "unstable", test))]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_noop_example(b: &mut Bencher) {
        b.iter(|| {})
    }
}
