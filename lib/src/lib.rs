#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod borrowed;
mod owned;

pub use borrowed::*;
pub use owned::*;

/// let `log_x()` be log base x
///
/// let `lb()` be log base 2 aka `log_2()`
///
/// ```md
/// 256 ^ BUF_LEN = 58 ^ MAX_STR_LEN
/// BUF_LEN = log_256(58 ^ MAX_STR_LEN)
///         = lb(58 ^ MAX_STR_LEN) / lb(256)
///         = MAX_STR_LEN * lb(58) / 8
///
/// lb(58) = 5.857980995127572
/// ```
///
/// Approximate this operation by multiplying 1000 on numerator
/// and denominator, then round off(up) numerator
/// so `BUF_LEN = MAX_STR_LEN * 5858 / 8000`.
///
/// Round down BUF_LEN to be conservative
pub const fn buf_len(max_str_len: usize) -> usize {
    max_str_len * 5858 / 8000
}
