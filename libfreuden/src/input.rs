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
    fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> isize;
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

impl<R: std::io::Read> Drop for Input<R> {
    fn drop(&mut self) {
        if self.reader.is_none() {
            unsafe {
                munmap(self.buffer.as_mut_ptr() as *mut c_void, self.buffer.len());
            }
        }
    }
}

impl Input<std::io::Stdin> {
    pub fn new() -> Self {
        let buffer = unsafe {
            use std::os::fd::FromRawFd;
            let fd = 0;
            let file = std::fs::File::from_raw_fd(fd);
            let meta = file.metadata().unwrap();
            std::mem::forget(file);
            let len = meta.len() as usize;

            // let buf_ptr = malloc(len);
            // read(fd, buf_ptr, len);

            let buf_ptr = mmap(std::ptr::null_mut(), len, PROT_READ, MAP_SHARED, fd, 0);
            let buffer = std::slice::from_raw_parts(buf_ptr as *mut u8, len);
            buffer
        };

        // let mut buffer = Vec::new();
        // std::io::Read::read_to_end(&mut std::io::stdin(), &mut buffer).unwrap();
        Input {
            reader: None,
            buffer: buffer.to_vec(),
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

    #[inline]
    pub fn next_usize(&mut self) -> usize {
        // self.skip_ws();

        let mut v = 0;
        while self.idx < self.buffer.len() {
            let b = unsafe { self.buffer.get_unchecked(self.idx) };
            if *b < b'0' {
                break;
            }
            v = v * 10 + (*b - b'0') as usize;
            self.idx += 1;
        }
        // self.skip_ws();
        self.idx += 1;
        // unsafe { std::arch::x86_64::_mm_prefetch::<{ std::arch::x86_64::_MM_HINT_T1 }>(std::ptr::from_ref(&self.buffer.get_unchecked(self.idx)) as *const _) }

        v
    }

    #[inline]
    pub fn next_u32(&mut self) -> usize {
        // let read_ptr = unsafe { self.buffer.as_ptr().add(self.idx).cast() as *const u64 };
        let read_ptr = std::ptr::from_ref(&self.buffer[self.idx]) as *const u64;
        // let chunk = unsafe { std::ptr::read_unaligned(read_ptr) }; // max 8 chars
        // let chunk = std::hint::black_box(chunk);
        let (value, len) = Self::parse_1e8(read_ptr);
        self.idx += len as usize;
        self.idx += 1;

        value
    }
    #[inline]
    fn parse_1e8(data: *const u64) -> (usize, u8) {
        // let chunk_a = unsafe { std::ptr::read_unaligned(data) }; // max 8 chars;
        let mut chunk_a: u64;
        unsafe {
            core::arch::asm!(
            "mov {0}, [{1}]",
            out(reg) chunk_a,
            in(reg) data,
            options(nostack)
            );
        }

        let zero_to_nine = chunk_a ^ 0x3030303030303030;
        let non_zero = zero_to_nine & 0xF0F0F0F0F0F0F0F0;
        let len = non_zero.trailing_zeros() as u8 >> 3;

        // https://lemire.me/blog/2022/01/21/swar-explained-parsing-eight-digits/
        let chunk_b = zero_to_nine << (8 * (8 - len));
        let out = Self::parse_chunk(chunk_b);
        (out as usize, len)
    }

    #[inline]
    fn parse_chunk(mut chunk: u64) -> usize {
        // let mut chunk: u64 = *chunk as u64;
        // let mut chunk: u64;
        let mut _intermediate1: u64;
        let mut _intermediate2: u64;

        unsafe {
            core::arch::asm!(
                // "mov {0}, [{1}]",
                "mov {1}, {0}",
                "shr {1}, 0x8",
                "imul {0}, 0xA",
                "add {0}, {1}",

                "mov {1}, {0}",
                "shr {1}, 0x10",

                "mov {2}, 0x000000FF000000FF",
                "and {0}, {2}",
                "and {1}, {2}",

                "mov {2}, 0x000F424000000064",
                "imul {0}, {2}",

                "mov {2}, 0x0000271000000001",
                "imul {1}, {2}",
                "add {0}, {1}",
                "shr {0}, 0x20",

                inout(reg) chunk,
                lateout(reg) _intermediate1,
                lateout(reg) _intermediate2,

                options(nostack)
            );
        }

        /*
                    uint32_t  parse_eight_digits_unrolled(uint64_t val) {
                    const uint64_t mask = 0x000000FF000000FF;
                    const uint64_t mul1 = 0x000F424000000064; // 100 + (1000000ULL << 32)
                    const uint64_t mul2 = 0x0000271000000001; // 1 + (10000ULL << 32)

                    val = ((val1 * mul1) + ((val2 * mul2)) >> 32;
                    return val;


        }
                 */

        // const MASK: u64 = 0x000000FF000000FF;
        // const MUL1: u64 = 0x000F424000000064; // 100 + (1000000ULL << 32)
        // const MUL2: u64 = 0x0000271000000001; // 1 + (10000ULL << 32)
        // println!("{chunk:x}");
        // chunk = (chunk * 10) + (chunk >> 8); // val = (val * 2561) >> 8;
        // println!("{chunk:x}");
        // chunk = (((chunk & MASK) * MUL1) + (((chunk >> 16) & MASK) * MUL2)) >> 32;
        // println!("{chunk:x}");

        let out = chunk as u32;
        debug_assert!(out <= 9999_9999);
        unsafe { std::hint::assert_unchecked(out <= 9999_9999) };
        out as usize
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
