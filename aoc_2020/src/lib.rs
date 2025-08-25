use common::register_year;

// Import all day modules to trigger solution registration
mod day01;

// Register this year with the dynamic registry
// Solutions are automatically discovered from day modules
register_year!(2020);