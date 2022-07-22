//! # Frames
//! Allows the user to display sprite like structures in the terminal more easily.
//! 
//! ## Structs
//! - Manager
//! - Object
//! - frame_types
//!   - Basic
//!   - Fill
//!   - Text
//!   - Bundle
//!   - Mask
//! - mask-rules
//!   - Circle
//! - Pixel
//! - PixelData
//! - Coord
//! - Rec
//! - Task
//! ## Traits
//! - SizeUpdate
//! - MaskRule
//! - MaskLogic

pub mod prelude;

pub mod frame_types;

pub mod modifiers;

mod manager;
pub use manager::*;

pub use crossterm;

pub mod test_helpers;