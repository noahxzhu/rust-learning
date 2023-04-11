//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// //! # Art
// //!
// //! A library for modeling artistic concepts.
//
// pub mod kinds {
//     /// The primary colors according to the RYB color model.
//     pub enum PrimaryColor {
//         Red,
//         Yellow,
//         Blue,
//     }
//
//     /// The secondary colors according to the RYB color model.
//     pub enum SecondaryColor {
//         Orange,
//         Green,
//         Purple,
//     }
// }
//
// // pub mod utils {
// //     use crate::kinds::*;
// //
// //     pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
// //         //
// //     }
// // }
