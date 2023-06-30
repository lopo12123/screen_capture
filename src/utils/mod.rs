use std::cmp::{max, min};

pub fn clamp(suppose: i32, low: i32, high: i32) -> i32 {
    min(high, max(low, suppose))
}