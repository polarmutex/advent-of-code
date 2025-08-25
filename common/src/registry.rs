//! Dynamic year and solution registry system
//! 
//! Eliminates manual year registration by using constructor-based discovery.
//! Years and solutions are automatically registered via macros.

use crate::Solution;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

/// Registry of all available years and their solutions
pub struct YearRegistry {
    years: HashMap<u16, &'static [Solution]>,
    lazy_years: HashMap<u16, fn() -> &'static [Solution]>,
}

impl YearRegistry {
    pub fn new() -> Self {
        Self {
            years: HashMap::new(),
            lazy_years: HashMap::new(),
        }
    }

    /// Register a year with its solutions
    pub fn register_year(&mut self, year: u16, solutions: &'static [Solution]) {
        self.years.insert(year, solutions);
    }
    
    /// Register a year with a lazy function
    pub fn register_lazy_year(&mut self, year: u16, solutions_fn: fn() -> &'static [Solution]) {
        self.lazy_years.insert(year, solutions_fn);
    }

    /// Get solutions for a specific year
    pub fn get_year(&mut self, year: u16) -> &'static [Solution] {
        // First check if we have pre-cached solutions
        if let Some(solutions) = self.years.get(&year) {
            return *solutions;
        }
        
        // Then check if we have a lazy function for this year
        if let Some(solutions_fn) = self.lazy_years.get(&year) {
            let solutions = solutions_fn();
            self.years.insert(year, solutions);
            return solutions;
        }
        
        &[]
    }

    /// Get all available years
    pub fn available_years(&self) -> Vec<u16> {
        let mut years: Vec<u16> = self.years.keys().copied()
            .chain(self.lazy_years.keys().copied())
            .collect();
        years.sort();
        years.dedup();
        years
    }

    /// Check if a year is available
    pub fn has_year(&self, year: u16) -> bool {
        self.years.contains_key(&year) || self.lazy_years.contains_key(&year)
    }
}

/// Global registry instance
static YEAR_REGISTRY: Lazy<Mutex<YearRegistry>> = Lazy::new(|| {
    Mutex::new(YearRegistry::new())
});

/// Registration trait for automatic year discovery
pub trait YearProvider {
    fn year() -> u16;
    fn solutions() -> &'static [Solution];
}

/// Register a year's solutions
pub fn register_year(year: u16, solutions: &'static [Solution]) {
    if let Ok(mut registry) = YEAR_REGISTRY.lock() {
        registry.register_year(year, solutions);
    }
}

/// Register a year using a lazy function
pub fn register_lazy_year(year: u16, solutions_fn: fn() -> &'static [Solution]) {
    if let Ok(mut registry) = YEAR_REGISTRY.lock() {
        registry.register_lazy_year(year, solutions_fn);
    }
}

/// Macro for registering a year module with dynamic solution discovery
/// 
/// Usage: `register_year!(2024);`
#[macro_export]
macro_rules! register_year {
    ($year:literal) => {
        use $crate::solution::solutions_as_static_slice;
        
        #[ctor::ctor]
        fn register() {
            // Register year with a lazy function that gets solutions on-demand
            $crate::registry::register_lazy_year($year, solutions_as_static_slice);
        }
    };
}

/// Helper function to access the global registry
pub fn get_year_solutions(year: u16) -> &'static [Solution] {
    if let Ok(mut registry) = YEAR_REGISTRY.lock() {
        registry.get_year(year)
    } else {
        &[]
    }
}

/// Get all available years
pub fn get_available_years() -> Vec<u16> {
    if let Ok(registry) = YEAR_REGISTRY.lock() {
        registry.available_years()
    } else {
        Vec::new()
    }
}

/// Check if year is available
pub fn has_year(year: u16) -> bool {
    if let Ok(registry) = YEAR_REGISTRY.lock() {
        registry.has_year(year)
    } else {
        false
    }
}