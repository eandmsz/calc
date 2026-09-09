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

//! UI module root: the libcosmic application, the keypad grid, the
//! expression display, the side panels and the keyboard bindings.
//!
//! The calculation itself lives in `cosmic-calc-core`, which this crate
//! re-exports; nothing under `ui` should hold state the core could own
//! instead.

pub mod app;
pub mod button_style;
pub mod buttons;
pub mod cosmic_bridge;
pub mod display;
pub mod display_metrics;
pub mod font;
pub mod font_metrics;
pub mod keymap;
pub mod keypad;
pub mod keys;
pub mod panels;

pub use app::AppModel;
pub use buttons::{
    apply_button, apply_resolved_button, resolve_for_keyboard, Button, ButtonEffect, ClearMode,
    MemoryOp, UiState,
};
pub use cosmic_bridge::override_from_cosmic;
