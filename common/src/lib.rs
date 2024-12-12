mod answer;
mod misc;
mod part;
#[macro_use]
pub mod solution;

pub use answer::Answer;
pub use misc::{human_time, load, load_raw};
pub use part::Part;
pub use solution::Solution;
