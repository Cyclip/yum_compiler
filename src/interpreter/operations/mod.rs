//! Operations in the Symbol enum are split up here.
//! This includes binary and unary operations.
//! All modules are public and should be called through
//! `mod operations`.

// Binary operations
pub mod add; // Addition
pub mod sub; // Subtraction
pub mod mul; // Multiplication
pub mod div; // Division
pub mod power; // Power
pub mod lt; // Less than
pub mod le; // Less than or equal to
pub mod gt; // Greater than
pub mod ge; // Greater than or equal to
pub mod eq; // Equal to
pub mod ne; // Not equal to
pub mod and; // And
pub mod or; // Or

// Unary operations
pub mod not; // Not
pub mod neg; // Negation


pub fn bool_to_int(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}