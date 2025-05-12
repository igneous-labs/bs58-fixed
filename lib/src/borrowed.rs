use core::{error::Error, fmt::Display, ops::Deref};

use crate::buf_len;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Bs58StrDecodeErr {
    NotOfBufLen,
    Bs58(bs58::decode::Error),
}

impl Display for Bs58StrDecodeErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotOfBufLen => f.write_str("bytes not of correct length"),
            Self::Bs58(e) => e.fmt(f),
        }
    }
}

impl Error for Bs58StrDecodeErr {}

/// A reference to a base58-encoded str
/// of a fixed-size buffer.
///
/// Referenced str is guaranteed to be valid (a base58-encoded byte buffer of the correct length)
/// at construction time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Bs58Str<'a, const MAX_STR_LEN: usize>(pub(crate) &'a str);

/// Constructors
impl<'a, const MAX_STR_LEN: usize> Bs58Str<'a, MAX_STR_LEN> {
    /// Same as [`Self::decode_from_onto`], but returns an owned buffer
    #[inline]
    pub fn decode_from<const BUF_LEN: usize>(
        from: &'a str,
    ) -> Result<(Self, [u8; BUF_LEN]), Bs58StrDecodeErr> {
        let mut buf = [0u8; BUF_LEN];
        let res = Self::decode_from_onto(from, &mut buf)?;
        Ok((res, buf))
    }

    // Need to use a const generic with comptime assertion
    // here instead of associated const
    // because we cant do `-> [u8; Self::BUF_LEN]` yet
    //
    /// Attempts to create [`Self`] by decoding the given str `from` onto `buf`,
    /// verifying that it is indeed a base58-encoded buffer of the correct size.
    ///
    /// Returns None if `from` is not a valid base58-encoded buffer of the correct size
    #[inline]
    pub fn decode_from_onto<const BUF_LEN: usize>(
        from: &'a str,
        buf: &mut [u8; BUF_LEN],
    ) -> Result<Self, Bs58StrDecodeErr> {
        const {
            assert!(BUF_LEN == Self::BUF_LEN);
        }

        let len = bs58::decode(from)
            .onto(buf)
            .map_err(Bs58StrDecodeErr::Bs58)?;
        if len != BUF_LEN {
            Err(Bs58StrDecodeErr::NotOfBufLen)
        } else {
            Ok(Self(from))
        }
    }
}

/// Accessors
impl<const MAX_STR_LEN: usize> Bs58Str<'_, MAX_STR_LEN> {
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        self.0
    }
}

/// Decode
impl<const MAX_STR_LEN: usize> Bs58Str<'_, MAX_STR_LEN> {
    pub const BUF_LEN: usize = buf_len(MAX_STR_LEN);

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

        // safety: len checked at compile time above, so will not error with BufferTooSmall
        // safety: struct guaranteed to be a valid base58-encoded str of
        // the correct length at construction time
        unsafe { bs58::decode(self.as_slice()).onto(buf).unwrap_unchecked() };
    }
}

// core traits

impl<const MAX_STR_LEN: usize> Deref for Bs58Str<'_, MAX_STR_LEN> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const MAX_STR_LEN: usize> AsRef<str> for Bs58Str<'_, MAX_STR_LEN> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<const MAX_STR_LEN: usize> AsRef<[u8]> for Bs58Str<'_, MAX_STR_LEN> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use proptest::{collection::vec, prelude::*};

    use super::*;

    macro_rules! test_suite {
        ($MAX_STR_LEN:expr, $BUF_LEN:expr, $test_name:ident) => {
            proptest! {
                #[test]
                fn $test_name(
                    v in vec(any::<u8>(), 0..=2 * $BUF_LEN),
                ) {
                    type S<'a> = Bs58Str<'a, $MAX_STR_LEN>;

                    let bs58_impl = bs58::encode(&v).into_string();
                    let us_res = S::decode_from::<$BUF_LEN>(&bs58_impl);
                    if v.len() == $BUF_LEN {
                        let (s, buf) = us_res.unwrap();
                        prop_assert_eq!(s.as_str(), &bs58_impl);
                        prop_assert_eq!(buf.as_slice(), v.as_slice());
                    } else {
                        us_res.unwrap_err();
                    }
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
