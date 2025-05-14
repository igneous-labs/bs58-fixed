use bs58_fixed_wasm::Bs58Array;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

/// Need this to also declare the type alias in ts
/// if Bs58Pk is used as a field in other Tsify objects
#[tsify_next::declare]
pub type Bs58Pk = Bs58Array<32, 44>;

/// This fn:
/// - takes in a base58 encoded 32-byte buffer
/// - sets the last byte to 0
/// - returns it back as a base58-encoded string
#[wasm_bindgen(js_name = zeroLast)]
pub fn zero_last(mut s: Bs58Pk) -> Bs58Pk {
    s.0[31] = 0;
    s
}

/// Same as {@link zeroLast}, but operates on option and
/// returns all zeros if `s` is None
#[wasm_bindgen(js_name = zeroLastOpt)]
pub fn zero_last_opt(s: Option<Bs58Pk>) -> Bs58Pk {
    s.map(zero_last).unwrap_or_default()
}

/// Same as {@link zeroLast}, but operates by ref
#[wasm_bindgen(js_name = zeroLastRef)]
pub fn zero_last_ref(s: &Bs58Pk) -> Bs58Pk {
    let mut res = *s;
    res.0[31] = 0;
    res
}

/// Same as {@link zeroLast}, but does it for all elements in the array
#[allow(clippy::boxed_local)] // clippy dont work well with wasm_bindgen?
#[wasm_bindgen(js_name = zeroLastVec)]
pub fn zero_last_vec(s: Box<[Bs58Pk]>) -> Box<[Bs58Pk]> {
    s.iter().map(zero_last_ref).collect()
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ZeroLastObj {
    pub arg: Bs58Pk,
}

/// Same as {@link zeroLast}, but the arg and return types are
/// a `Tsify` struct/object, to test that Bs58Pk can be composed
/// as fields of other such structs
#[wasm_bindgen(js_name = zeroLastObj)]
pub fn zero_last_obj(ZeroLastObj { arg }: ZeroLastObj) -> ZeroLastObj {
    ZeroLastObj {
        arg: zero_last(arg),
    }
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ZeroLastOptObj {
    pub arg: Option<Bs58Pk>,
}

/// Same as {@link zeroLastOpt}, but the arg and return types are
/// a `Tsify` struct/object, to test that Bs58Pk can be composed
/// as fields of other such structs
#[wasm_bindgen(js_name = zeroLastOptObj)]
pub fn zero_last_opt_obj(ZeroLastOptObj { arg }: ZeroLastOptObj) -> ZeroLastObj {
    ZeroLastObj {
        arg: zero_last_opt(arg),
    }
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ZeroLastVecObj {
    pub arg: Box<[Bs58Pk]>,
}

/// Same as {@link zeroLastVec}, but the arg and return types are
/// a `Tsify` struct/object, to test that Bs58Pk can be composed
/// as fields of other such structs
#[wasm_bindgen(js_name = zeroLastVecObj)]
pub fn zero_last_vec_obj(ZeroLastVecObj { arg }: ZeroLastVecObj) -> ZeroLastVecObj {
    ZeroLastVecObj {
        arg: zero_last_vec(arg),
    }
}
