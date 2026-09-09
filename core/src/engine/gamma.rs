//! Calc - An easy-to-use stateful scientific calculator focusing on intuitive user experience
//! Copyright (C) 2027  Andras Vinter
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.

//! Gamma function wrapper used by factorial on non-integer operands.
//! Delegates to libm::tgamma which provides high-accuracy gamma across
//! the full f64 range, including the reflection formula for negative
//! non-integer arguments.

use crate::engine::errors::CalcError;
use crate::engine::eval::is_integer;

/// Compute x! = Γ(x + 1). Returns Undefined for negative integers
/// (where Γ has poles) and for overflow inputs.
pub fn factorial(x: f64) -> Result<f64, CalcError> {
    if x.is_nan() {
        return Err(CalcError::Indeterminate);
    }
    if x < 0.0 && is_integer(x) {
        return Err(CalcError::Undefined);
    }
    let y = libm::tgamma(x + 1.0);
    if y.is_nan() {
        return Err(CalcError::Undefined);
    }
    if y.is_infinite() {
        return Err(CalcError::Overflow);
    }
    Ok(y)
}
