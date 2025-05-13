use core::ops::Deref;

use tsify_next::Tsify;
use wasm_bindgen::{
    convert::{
        FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi,
        VectorFromWasmAbi, VectorIntoWasmAbi,
    },
    describe::{WasmDescribe, WasmDescribeVector},
    prelude::*,
};

use crate::Bs58Array;

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(typescript_type = "Bs58Array")]
    pub type JsType;
}

const DECL: &str = "export type Bs58Array = string";

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = DECL;

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> Tsify for Bs58Array<BUF_LEN, MAX_STR_LEN> {
    type JsType = JsType;
    const DECL: &'static str = DECL;
}

// Below snippets are copied from expansion of #[derive(Tsify)] macro
// with #[tsify(into_wasm_abi, from_wasm_abi)]

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> WasmDescribe
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    #[inline]
    fn describe() {
        JsType::describe();
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> WasmDescribeVector
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    #[inline]
    fn describe_vector() {
        JsType::describe_vector();
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> IntoWasmAbi
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    type Abi = <JsType as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        // wasm_bindgen doesn't forward the error message from the `into_js` result.
        // https://github.com/rustwasm/wasm-bindgen/issues/2732
        // Until that issue is fixed, we don't directly use `unwrap_throw()` and instead build our
        // own error message.
        // Convert to `self.into_js().unwrap_throw().into_abi()` when fixed.
        match self.into_js() {
            Ok(js) => js.into_abi(),
            Err(err) => {
                let loc = core::panic::Location::caller();
                // In theory, `wasm_bindgen::throw_str(&msg)` should work,
                // but the error emitted by `wasm_bindgen::throw_str` cannot be picked up by
                // `#[should_panic(expect = ...)]` in tests, so we use a regular panic.
                panic!(
                    "(Converting type failed) {} ({}:{}:{})",
                    err,
                    loc.file(),
                    loc.line(),
                    loc.column()
                );
            }
        }
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> OptionIntoWasmAbi
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    #[inline]
    fn none() -> Self::Abi {
        <JsType as OptionIntoWasmAbi>::none()
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> From<Bs58Array<BUF_LEN, MAX_STR_LEN>>
    for JsValue
{
    #[inline]
    fn from(value: Bs58Array<BUF_LEN, MAX_STR_LEN>) -> Self {
        // wasm_bindgen doesn't forward the error message from the `into_js` result.
        // https://github.com/rustwasm/wasm-bindgen/issues/2732
        // Until that issue is fixed, we don't directly use `unwrap_throw()` and instead build our
        // own error message.
        // Convert to `value.into_js().unwrap_throw().into()` when fixed.
        match value.into_js() {
            Ok(js) => js.into(),
            Err(err) => {
                let loc = core::panic::Location::caller();
                // In theory, `wasm_bindgen::throw_str(&msg)` should work,
                // but the error emitted by `wasm_bindgen::throw_str` cannot be picked up by
                // `#[should_panic(expect = ...)]` in tests, so we use a regular panic.
                panic!(
                    "(Converting type failed) {} ({}:{}:{})",
                    err,
                    loc.file(),
                    loc.line(),
                    loc.column()
                );
            }
        }
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> VectorIntoWasmAbi
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    type Abi = <JsType as VectorIntoWasmAbi>::Abi;

    #[inline]
    fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
        // TODO: intermediate vec allocation looks unnecessary here
        // but idk how to remove it
        let values = vector
            .iter()
            // wasm_bindgen doesn't forward the error message from the `into_js` result.
            // https://github.com/rustwasm/wasm-bindgen/issues/2732
            // Until that issue is fixed, we don't directly use `unwrap_throw()` and instead build our
            // own error message.
            .map(|value| match value.into_js() {
                Ok(js) => js.into(),
                Err(err) => {
                    let loc = core::panic::Location::caller();
                    // In theory, `wasm_bindgen::throw_str(&msg)` should work,
                    // but the error emitted by `wasm_bindgen::throw_str` cannot be picked up by
                    // `#[should_panic(expect = ...)]` in tests, so we use a regular panic.
                    panic!(
                        "(Converting type failed) {} ({}:{}:{})",
                        err,
                        loc.file(),
                        loc.line(),
                        loc.column()
                    );
                }
            })
            .collect();

        JsValue::vector_into_abi(values)
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> FromWasmAbi
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    type Abi = <JsType as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        let result = Self::from_js(JsType::from_abi(js));
        if let Err(err) = result {
            wasm_bindgen::throw_str(err.to_string().as_ref());
        }
        result.unwrap_throw()
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> OptionFromWasmAbi
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    #[inline]
    fn is_none(js: &Self::Abi) -> bool {
        <JsType as OptionFromWasmAbi>::is_none(js)
    }
}

// Self does not impl Deref<Self> but wasm_bindgen traits
// need RefFromWasmAbi::Anchor to impl Deref<Self>, so just make this stupid wrapper
#[repr(transparent)]
pub struct Bs58ArrayAnchor<T>(pub T);

impl<T> Deref for Bs58ArrayAnchor<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> RefFromWasmAbi
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    type Abi = <JsType as RefFromWasmAbi>::Abi;

    type Anchor = Bs58ArrayAnchor<Self>;

    #[inline]
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        let result = Self::from_js(&*JsType::ref_from_abi(js));
        if let Err(err) = result {
            wasm_bindgen::throw_str(err.to_string().as_ref());
        }
        Bs58ArrayAnchor(result.unwrap_throw())
    }
}

impl<const BUF_LEN: usize, const MAX_STR_LEN: usize> VectorFromWasmAbi
    for Bs58Array<BUF_LEN, MAX_STR_LEN>
{
    type Abi = <JsType as VectorFromWasmAbi>::Abi;

    #[inline]
    unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
        // TODO: intermediate vec allocation looks unnecessary here
        // but idk how to remove it
        JsValue::vector_from_abi(js)
            .iter()
            .map(|value| {
                let result = Self::from_js(value);
                if let Err(err) = result {
                    wasm_bindgen::throw_str(err.to_string().as_ref());
                }
                result.unwrap_throw()
            })
            .collect()
    }
}
