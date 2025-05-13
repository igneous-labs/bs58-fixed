#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

pub mod serde;

// impl notes:
// would really only like BUF_LEN const-generic here, but limits of const-generics
// interactions with traits (serde) mean we have to make MAX_STR_LEN part of the type
// instead of computing it as an associated const or smth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Bs58Array<const BUF_LEN: usize, const MAX_STR_LEN: usize>(pub [u8; BUF_LEN]);

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> Bs58Array<BUF_LEN, MAX_STR_LEN> {
    pub const ZERO: Self = Self([0u8; BUF_LEN]);
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> Default for Bs58Array<BUF_LEN, MAX_STR_LEN> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}
