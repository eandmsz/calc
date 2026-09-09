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
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Calculation core: everything the calculator does that does not need
//! a window. The tokenizer/parser/evaluator pipeline, the display
//! formatter, persisted configuration, themes, locale handling,
//! clipboard sanitising, history and memory all live here.
//!
//! Deliberately free of any GUI dependency, so `cargo test -p
//! cosmic-calc-core` compiles in seconds rather than pulling in
//! libcosmic and wgpu. The `cosmic-calc` binary crate re-exports every
//! module below, so UI code keeps referring to them as
//! `crate::engine`, `crate::config`, and so on.

pub mod clipboard;
pub mod color;
pub mod config;
pub mod engine;
pub mod history;
pub mod layout;
mod lenient;
pub mod locale;
pub mod memory;
pub mod props;
pub mod rng;
pub mod theme;

#[cfg(test)]
mod tests;
