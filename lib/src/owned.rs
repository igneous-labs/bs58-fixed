use core::{cmp::Ordering, fmt::Display, ops::Deref};

use bs58::encode::EncodeTarget;

/// A constant max-size base58-encoded string
/// for encoding of fixed-size buffers
#[derive(Debug, Clone, Copy)]
pub struct Bs58String<const MAX_STR_LEN: usize> {
    len: usize,

    // dont use MaybeUninit because `EncodeTarget::encode_with` requires &mut [u8],
    // and it is UB to make a ref to uninitialized data
    buf: [u8; MAX_STR_LEN],
}

/// Constructors
impl<const MAX_STR_LEN: usize> Bs58String<MAX_STR_LEN> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            buf: [0u8; MAX_STR_LEN],
            len: 0,
        }
    }
}

/// Accessors
impl<const MAX_STR_LEN: usize> Bs58String<MAX_STR_LEN> {
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        // safety: valid initialized memory and len
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr(), self.len) }
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        // safety: bs58 alphabet is valid ascii/utf8
        unsafe { core::str::from_utf8_unchecked(self.as_slice()) }
    }
}

/// Codec
impl<const MAX_STR_LEN: usize> Bs58String<MAX_STR_LEN> {
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
    pub const BUF_LEN: usize = { MAX_STR_LEN * 5858 / 8000 };

    // Need to use a const generic with comptime assertion
    // here instead of associated const
    // because we cant do `buf: &[u8; Self::BUF_LEN]` yet
    #[inline]
    pub fn encode<const BUF_LEN: usize>(buf: &[u8; BUF_LEN]) -> Self {
        let mut res = Self::new();
        res.encode_from(buf);
        res
    }

    // Need to use a const generic with comptime assertion
    // here instead of associated const
    // because we cant do `buf: &[u8; Self::BUF_LEN]` yet
    /// Encodes `buf` onto `self`, overwriting previous data
    #[inline]
    pub fn encode_from<const BUF_LEN: usize>(&mut self, buf: &[u8; BUF_LEN]) {
        const {
            assert!(BUF_LEN == Self::BUF_LEN);
        }

        // safety: len checked at compile time above
        unsafe {
            bs58::encode(buf).onto(self).unwrap_unchecked();
        }
    }

    // Need to use a const generic with comptime assertion
    // here instead of associated const
    // because we cant do `-> [u8; Self::BUF_LEN]` yet
    #[inline]
    pub fn decode<const BUF_LEN: usize>(&self) -> [u8; BUF_LEN] {
        let mut res = [0u8; BUF_LEN];
        self.decode_onto(&mut res);
        res
    }

    // Need to use a const generic with comptime assertion
    // here instead of associated const
    // because we cant do `-> [u8; Self::BUF_LEN]` yet
    /// Decodes `self` onto `buf`, overwriting previous data
    #[inline]
    pub fn decode_onto<const BUF_LEN: usize>(&self, buf: &mut [u8; BUF_LEN]) {
        const {
            assert!(BUF_LEN == Self::BUF_LEN);
        }

        // safety: len checked at compile time above
        unsafe { bs58::decode(self.as_slice()).onto(buf).unwrap_unchecked() };
    }
}

impl<const MAX_STR_LEN: usize> EncodeTarget for Bs58String<MAX_STR_LEN> {
    #[inline]
    fn encode_with(
        &mut self,
        _max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> bs58::encode::Result<usize>,
    ) -> bs58::encode::Result<usize> {
        let len = f(&mut self.buf)?;
        if len > MAX_STR_LEN {
            Err(bs58::encode::Error::BufferTooSmall)
        } else {
            self.len = len;
            Ok(len)
        }
    }
}

// core traits

impl<const MAX_STR_LEN: usize> Deref for Bs58String<MAX_STR_LEN> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const MAX_STR_LEN: usize> Default for Bs58String<MAX_STR_LEN> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// Use `self.as_slice()` for Ord, Eq, Hash

impl<const MAX_STR_LEN: usize> PartialEq for Bs58String<MAX_STR_LEN> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<const MAX_STR_LEN: usize> Eq for Bs58String<MAX_STR_LEN> {}

impl<const MAX_STR_LEN: usize> Ord for Bs58String<MAX_STR_LEN> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<const MAX_STR_LEN: usize> PartialOrd for Bs58String<MAX_STR_LEN> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const MAX_STR_LEN: usize> core::hash::Hash for Bs58String<MAX_STR_LEN> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<const MAX_STR_LEN: usize> Display for Bs58String<MAX_STR_LEN> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    macro_rules! test_suite {
        ($MAX_STR_LEN:expr, $BUF_LEN:expr, $test_name:ident) => {
            proptest! {
                #[test]
                fn $test_name(buf: [u8; $BUF_LEN]) {
                    type S = Bs58String<$MAX_STR_LEN>;

                    // round-trip
                    let encoded = S::encode(&buf);
                    let decoded = encoded.decode();
                    prop_assert_eq!(decoded, buf);

                    // check against bs58 impl
                    let bs58_impl = bs58::encode(buf).into_string();
                    prop_assert_eq!(bs58_impl.as_str(), encoded.as_str());
                }
            }
        };
    }

    test_suite!(0, 0, b_0_0_test);
    test_suite!(1, 0, b_1_0_test);
    test_suite!(2, 1, b_2_1_test);
    test_suite!(3, 2, b_3_2_test);
    test_suite!(4, 2, b_4_2_test);
    test_suite!(5, 3, b_5_3_test);
    test_suite!(6, 4, b_6_4_test);
    test_suite!(7, 5, b_7_5_test);
    test_suite!(8, 5, b_8_5_test);

    test_suite!(11, 8, b_11_8_test);

    test_suite!(16, 11, b_16_11_test);

    test_suite!(22, 16, b_22_16_test);

    test_suite!(44, 32, b_44_32_test);

    test_suite!(88, 64, b_88_64_test);
}
