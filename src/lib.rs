//! # Expression Simplifier
//! 
//! The program is a Rust library crate in the use of simplifying math expression, 
//! its function is based on the first Unit of HW1, Object Oriented course of SCSE, BUAA.
//! 
//! #Example
//! ```
//! use expression_simplify::read_input;
//! 
//! fn main() {
//!     read_input();
//! }
//! 
//! 
//! ```

pub mod simplifier;

pub use simplifier::inputhandler::input_handler::read_input as read_input;