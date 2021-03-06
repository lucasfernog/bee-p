// Copyright 2020 IOTA Stiftung
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with
// the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on
// an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

//! This module contains logic to convert an integer encoded by 243 trits to the same
//! integer encoded by 384 bits (or 48 signed bytes, `i8`).
//!
//! At the core of this a slice of binary-coded, balanced trits is interpreted
//! fanned-out `t243`, where `t243` is used analogous to `i64` or `u64`. If the latter
//! are 64-bit signed/unsigned integer types, then `t243` is a 243-trit integer type.
//! Analogous to fanning out a `u64` into 64 individual bits, `t243` is fanned out into
//! 243 trits, each (rather inefficiently) represented by one `u8`.

use std::cmp::Ordering;

use crate::bigint::{
    common::{BigEndian, LittleEndian, U32Repr, U8Repr},
    I384, T242, U384,
};
use bee_ternary::{raw::RawEncoding, Btrit, ShiftTernary, T1B1Buf, Trit, TritBuf, Trits, Utrit};

mod constants;
pub use constants::{
    BTRIT_NEG_ONE, BTRIT_ONE, BTRIT_ZERO, UTRIT_ONE, UTRIT_TWO, UTRIT_U384_MAX, UTRIT_U384_MAX_HALF, UTRIT_ZERO,
};

def_and_impl_ternary!(T243, 243);

impl<T: Trit> T243<T> {
    pub fn into_t242(self) -> T242<T> {
        let mut trit_buf = self.into_inner();
        trit_buf.pop();
        T242::from_trit_buf(trit_buf)
    }
}

impl T243<Utrit> {
    pub fn from_u384(value: U384<BigEndian, U32Repr>) -> Self {
        let mut u384_value = value;
        let mut u384_inner_slice = &mut u384_value.inner[..];

        let mut trit_buf = T243::<Utrit>::zero().into_inner();
        unsafe {
            for trit in trit_buf.as_i8_slice_mut() {
                let mut rem = 0;

                // Skip the most significant digits if they are 0.
                let mut first_digit = 0;
                for (i, digit) in u384_inner_slice.iter().enumerate() {
                    first_digit = i;
                    if *digit != 0 {
                        break;
                    }
                }
                u384_inner_slice = &mut u384_inner_slice[first_digit..];

                // Iterate over the digits of the bigint, starting from the most significant one.
                for digit in u384_inner_slice.iter_mut() {
                    let digit_with_rem = ((rem as u64) << 32) | *digit as u64;
                    *digit = (digit_with_rem / 3u64) as u32;
                    rem = (digit_with_rem % 3u64) as u32;
                }

                *trit = rem as i8;
            }
        }

        Self(trit_buf)
    }
}

impl<T: Trit> From<T242<T>> for T243<T> {
    fn from(value: T242<T>) -> Self {
        value.into_t243()
    }
}

impl From<I384<BigEndian, U8Repr>> for T243<Btrit> {
    fn from(value: I384<BigEndian, U8Repr>) -> Self {
        let be_u32 = Into::<I384<BigEndian, U32Repr>>::into(value);
        let le_u32 = Into::<I384<LittleEndian, U32Repr>>::into(be_u32);
        le_u32.into()
    }
}

impl From<I384<BigEndian, U32Repr>> for T243<Btrit> {
    fn from(value: I384<BigEndian, U32Repr>) -> Self {
        let value_little_endian: I384<LittleEndian, U32Repr> = value.into();
        value_little_endian.into()
    }
}

impl From<I384<LittleEndian, U32Repr>> for T243<Btrit> {
    fn from(value: I384<LittleEndian, U32Repr>) -> Self {
        let u384_value = value.shift_into_u384();
        let t243_unbalanced = T243::<Utrit>::from(u384_value);
        t243_unbalanced.into_shifted()
    }
}

impl From<U384<BigEndian, U32Repr>> for T243<Utrit> {
    fn from(value: U384<BigEndian, U32Repr>) -> Self {
        Self::from_u384(value)
    }
}

impl From<U384<LittleEndian, U32Repr>> for T243<Utrit> {
    fn from(value: U384<LittleEndian, U32Repr>) -> Self {
        let value: U384<BigEndian, U32Repr> = value.into();
        value.into()
    }
}
