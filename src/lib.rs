pub mod template;

// Use this file to add helper functions and additional modules.

// https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html
#[inline(always)]
pub fn parse_int(s: &str) -> usize {
    s.bytes().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}
