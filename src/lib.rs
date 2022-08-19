//! # Frames
//! Allows the user to display sprite like structures in the terminal more easily.
//! 
//! ## Structs
//! - frame_types
//!   - Basic
//!   - Bundle
//!   - Fill
//!   - Layout
//!   - Text
//!   - With Modifier
//! - manager
//! - modifiers
//!   - Position
//!   - Circle Mask
//! - prelude
//!   - Coord
//!   - Frame <IFrame>
//!   - Modifier <IModifier>
//!   - Pixel
//!   - PixelData
//!   - Input
//!   - ScreenBuf

pub mod prelude;

pub mod frame_types;

pub mod modifiers;

mod manager;
pub use manager::*;

pub use crossterm;

pub mod test_helpers;

mod color_string;
pub use color_string::{ColorString, ColorSlice};