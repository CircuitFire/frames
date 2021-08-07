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

mod shared;
pub use shared::*;

pub mod frame_types;

mod object;
pub use object::*;

mod manager;
pub use manager::*;

pub mod mask_rules;

pub use crossterm;