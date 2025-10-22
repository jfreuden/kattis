pub struct Input<'a> {
    buffer: &'a [u8],
    idx: usize,
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

impl Drop for Input<'_> {
    fn drop(&mut self) {
        unsafe {
            munmap(self.buffer.as_ptr() as *mut c_void, self.buffer.len());
        }
    }
}

impl Input<'_> {
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
        Input { buffer, idx: 0 }
    }
}

impl Input<'_> {
    #[inline]
    fn next_usize(&mut self) -> u32 {
        // self.skip_ws();

        let mut v = 0;
        while self.idx < self.buffer.len() {
            let b = unsafe { self.buffer.get_unchecked(self.idx) };
            if *b < b'0' {
                break;
            }
            v = v * 10 + (*b - b'0') as u32;
            self.idx += 1;
        }
        // self.skip_ws();
        self.idx += 1;
        v
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let read_ptr = std::ptr::from_ref(&self.buffer[self.idx]) as *const u64;
        let mut chunk = unsafe { std::ptr::read_unaligned(read_ptr) }; // max 8 chars;
        let zero_to_nine = chunk ^ 0x3030303030303030;
        let non_zero = zero_to_nine & 0xF0F0F0F0F0F0F0F0;
        let len = non_zero.trailing_zeros() as u8 >> 3;
        self.idx += len as usize;
        self.idx += 1;

        chunk = zero_to_nine << (8 * (8 - len));

        const MASK: u64 = 0x000000FF000000FF;
        const MUL1: u64 = 0x000F424000000064; // 100 + (1000000ULL << 32)
        const MUL2: u64 = 0x0000271000000001; // 1 + (10000ULL << 32)
        chunk = (chunk * 10) + (chunk >> 8); // val = (val * 2561) >> 8;
        chunk = (((chunk & MASK) * MUL1) + (((chunk >> 16) & MASK) * MUL2)) >> 32;

        let out = chunk as u32;
        debug_assert!(out <= 9999_9999);
        unsafe { std::hint::assert_unchecked(out <= 9999_9999) };
        out
    }

    #[inline]
    fn next_asm(&mut self) -> u32 {
        let read_ptr = std::ptr::from_ref(&self.buffer[self.idx]) as *const u64;
        let mut chunk: u64;
        unsafe {
            core::arch::asm!(
            "mov {0}, [{1}]",
            out(reg) chunk,
            in(reg) read_ptr,
            options(nostack)
            );

            let zero_to_nine = chunk ^ 0x3030303030303030;
            let non_zero = zero_to_nine & 0xF0F0F0F0F0F0F0F0;
            let len = non_zero.trailing_zeros() as u8 >> 3;
            self.idx += len as usize;
            self.idx += 1;
            chunk = zero_to_nine << (8 * (8 - len));

            let _intermediate1: u64;
            let _intermediate2: u64;

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

        let out = chunk as u32;
        debug_assert!(out <= 9999_9999);
        unsafe { std::hint::assert_unchecked(out <= 9999_9999) };
        out
    }

    #[inline]
    pub fn next(&mut self) -> u32 {
        self.next_usize()
    }
}

fn main() {
    let start = std::time::Instant::now();

    let mut input = Input::new();
    let cake_width = input.next();
    let cake_pieces = input.next();
    let mut total_area: u32 = 0;

    for _ in 0..cake_pieces {
        let shard_width = input.next();
        let shard_height = input.next();

        total_area += shard_width * shard_height;
    }

    println!("{}", total_area.div_euclid(cake_width));
    // let wait_until = start + std::time::Instant::now().duration_since(start) * 50;
    // while std::time::Instant::now() < wait_until {}
    eprintln!("{:?}", std::time::Instant::now().duration_since(start));
}
