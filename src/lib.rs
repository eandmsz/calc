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

//! Library root for the GUI crate. The calculation core lives in the
//! separate `cosmic-calc-core` package and is re-exported here, so UI
//! modules keep referring to `crate::engine`, `crate::config`, and so
//! on, and embedders get one import for the whole calculator.
//!
//! The split exists so the core can be tested without libcosmic: run
//! `cargo test -p cosmic-calc-core` for the engine, formatter, config
//! and clipboard suites, and `cargo test` for the UI on top.

pub use cosmic_calc_core::{
    clipboard, color, config, engine, history, layout, locale, memory, props, rng, theme,
};

pub mod ui;

#[cfg(test)]
mod tests;
