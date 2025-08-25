use common::register_year;

// Import all day modules to trigger solution registration
mod day_01;
mod day_02;
mod day_03;
mod day_04;
// [import_marker]

// Register this year with the dynamic registry
// Solutions are automatically discovered from day modules
register_year!(2024);
