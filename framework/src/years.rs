//! Year loading and registration
//! 
//! This module handles dynamic loading of year crates.
//! Add year dependencies here to enable automatic registration.

// Import year crates to trigger their registration constructors
#[cfg(feature = "year_2024")]
extern crate aoc_2024;

#[cfg(feature = "year_2023")]
extern crate aoc_2023;

#[cfg(feature = "year_2022")]
extern crate aoc_2022;

#[cfg(feature = "year_2021")]
extern crate aoc_2021;

#[cfg(feature = "year_2020")]
extern crate aoc_2020;

/// Initialize all available years
pub fn init_years() {
    // Year crates are initialized automatically via ctor constructors
    // This function exists to ensure year crate linking
    
    #[cfg(feature = "year_2024")]
    {
        // Force linking of aoc_2024 by referencing any public item
        // The year registration happens automatically via ctor
        let _ = stringify!(aoc_2024);
    }
    
    #[cfg(feature = "year_2023")]
    {
        // Force linking of aoc_2023
        // let _ = &aoc_2023::SOLUTIONS;
    }
    
    #[cfg(feature = "year_2022")]
    {
        // Force linking of aoc_2022
        // let _ = &aoc_2022::SOLUTIONS;
    }
    
    #[cfg(feature = "year_2021")]
    {
        // Force linking of aoc_2021
        // let _ = &aoc_2021::SOLUTIONS;
    }
    
    #[cfg(feature = "year_2020")]
    {
        // Force linking of aoc_2020
        // let _ = &aoc_2020::SOLUTIONS;
    }
}