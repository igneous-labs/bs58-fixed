//! This is a `#[serde(with = "bs58_fixed_wasm::serde")]` compatible module

use bs58_fixed::buf_len;
use serde::{de::Visitor, Deserializer, Serializer};

use crate::Bs58Array;

// Ser

#[inline]
pub fn serialize<S: Serializer, const BUF_LEN: usize, const MAX_STR_LEN: usize>(
    val: &Bs58Array<BUF_LEN, MAX_STR_LEN>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    const {
        assert!(buf_len(MAX_STR_LEN) == BUF_LEN);
    }

    let bs58_string = bs58_fixed::Bs58String::<MAX_STR_LEN>::encode(&val.0);
    ser.serialize_str(bs58_string.as_str())
}

// De

// impl visitor pattern so that it works for both String and &str

struct Bs58ArrayVisitor<const BUF_LEN: usize, const MAX_STR_LEN: usize>;

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> Visitor<'_>
    for Bs58ArrayVisitor<BUF_LEN, MAX_STR_LEN>
{
    type Value = Bs58Array<BUF_LEN, MAX_STR_LEN>;

    #[inline]
    fn expecting(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "base58 encoded string of byte buffer of len {BUF_LEN}")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        const {
            assert!(buf_len(MAX_STR_LEN) == BUF_LEN);
        }

        let (_, buf) =
            bs58_fixed::Bs58Str::<MAX_STR_LEN>::decode_from(v).map_err(serde::de::Error::custom)?;
        Ok(Bs58Array(buf))
    }
}

#[inline]
pub fn deserialize<'de, D: Deserializer<'de>, const BUF_LEN: usize, const MAX_STR_LEN: usize>(
    de: D,
) -> Result<Bs58Array<BUF_LEN, MAX_STR_LEN>, D::Error> {
    de.deserialize_str(Bs58ArrayVisitor)
}

mod impls {
    use serde::{Deserialize, Serialize};

    use super::{deserialize as deserialize_fn, serialize as serialize_fn, *};

    impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> Serialize for Bs58Array<BUF_LEN, MAX_STR_LEN> {
        #[inline]
        fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serialize_fn(self, ser)
        }
    }

    impl<'de, const BUF_LEN: usize, const MAX_STR_LEN: usize> Deserialize<'de>
        for Bs58Array<BUF_LEN, MAX_STR_LEN>
    {
        #[inline]
        fn deserialize<D>(de: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_fn(de)
        }
    }
}
