use crate::Answer;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug)]
pub struct Solution {
    pub name: &'static str,
    pub day: u8,

    pub part_1: fn(&str) -> miette::Result<Answer>,
    pub part_2: fn(&str) -> miette::Result<Answer>,
}

// Registry for automatic solution discovery within a year
static SOLUTION_REGISTRY: Lazy<Mutex<Vec<&'static Solution>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});

/// Register a solution for automatic discovery
pub fn register_solution(solution: &'static Solution) {
    if let Ok(mut registry) = SOLUTION_REGISTRY.lock() {
        registry.push(solution);
    }
}

/// Get all registered solutions for this year, sorted by day
/// Returns a Vec since we can't return a static slice of dynamic content
pub fn get_registered_solutions() -> Vec<Solution> {
    if let Ok(registry) = SOLUTION_REGISTRY.lock() {
        let mut solutions: Vec<Solution> = registry.iter().map(|&s| *s).collect();
        solutions.sort_by_key(|s| s.day);
        solutions
    } else {
        Vec::new()
    }
}

/// Convert Vec<Solution> to a static slice by leaking memory
/// This is called once per year crate initialization
pub fn solutions_as_static_slice() -> &'static [Solution] {
    let solutions = get_registered_solutions();
    let boxed = solutions.into_boxed_slice();
    Box::leak(boxed)
}

#[macro_export]
macro_rules! solution {
    ($name:expr, $day:expr) => {
        pub const SOLUTION: $crate::Solution = $crate::Solution {
            name: $name,
            day: $day,

            part_1,
            part_2,
        };

        // Automatically register this solution
        #[ctor::ctor]
        fn register() {
            $crate::solution::register_solution(&SOLUTION);
        }
    };
}
