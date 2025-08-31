use std::str;

use encoding_rs::SHIFT_JIS;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct FixedArray<const S: usize> {
    #[serde(with = "BigArray")]
    value: [u8; S],
}

// #[derive(PartialEq, Debug, Deserialize, Serialize)]
// pub struct FixedTypeArray<T, const S: usize> {
//     #[serde(with = "BigArray")]
//     value: [T; S],
// }

impl<const S: usize> FixedArray<S> {
    pub fn as_str_utf8(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.value)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.value
    }

    pub fn from_str(input: &str) -> Self {
        let mut out = [0u8; S];

        let len = input.len().min(S);
        out[..len].copy_from_slice(&input.as_bytes()[..len]);

        Self { value: out }
    }

    pub fn from_str_to_sjis(input: &str) -> Self {
        let (input_sjis, _, errors) = SHIFT_JIS.encode(input);

        let mut out = [0u8; S];

        let len = input_sjis.len().min(S);
        out[..len].copy_from_slice(&input_sjis);

        Self { value: out }
    }
}

impl<const S: usize> Default for FixedArray<S> {
    fn default() -> Self {
        Self { value: [0u8; S] }
    }
}

impl<const S: usize> From<&str> for FixedArray<S> {
    fn from(input: &str) -> Self {
        Self::from_str(input)
    }
}

// impl<T, const S: usize> FixedTypeArray<T, S> {
//     pub fn as_arr(&self) -> &[T] {
//         &self.value
//     }
//
//     pub fn to_vec(&self) -> Vec<T> {
//         Vec::from(self.value)
//     }
//
//     pub fn from_array(input: &[T]) -> Self {
//         let mut out = [T; S];
//
//         let len = input.len().min(S);
//         out[..len].copy_from_slice(&input);
//
//         Self { value: out }
//     }
// }

// impl<T, const S: usize> From<&str> for FixedTypeArray<T, S> {
//     fn from(input: &str) -> Self {
//         Self::from_str(input)
//     }
// }
