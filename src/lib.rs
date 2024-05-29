// #![no_std]

mod errors;
mod executable;
mod interpreter;
mod models;
mod platform;

pub use executable::*;
pub use interpreter::*;
pub use models::*;
pub use platform::*;

// pub fn add(left: usize, right: usize) -> usize {
//   left + right
// }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn it_works() {
//     let result = add(2, 2);
//     assert_eq!(result, 4);
//   }
// }
