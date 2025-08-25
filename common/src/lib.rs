mod answer;
mod misc;
mod part;
pub mod registry;
#[macro_use]
pub mod solution;

pub use answer::Answer;
pub use misc::{human_time, load, load_raw, pixel_vector_to_char_strings, ocr};
pub use part::Part;
pub use registry::{get_available_years, get_year_solutions, has_year, register_lazy_year, YearProvider, YearRegistry};
pub use solution::{get_registered_solutions, register_solution, solutions_as_static_slice, Solution};
