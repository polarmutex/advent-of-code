# Adding New Years and Days - Fully Automatic Registration

This repository now uses **completely automatic year and day registration** - zero manual code changes required!

## ✅ Adding a New Day (100% Automatic)

Simply use the framework CLI:

```bash
cargo run -- init <day> [year]
```

**Example:**
```bash
cargo run -- init 5 2024
```

**What happens automatically:**
1. ✅ Creates solution file: `aoc_2024/src/day_05.rs`
2. ✅ Adds module import: `mod day_05;` to `aoc_2024/src/lib.rs`
3. ✅ **Automatically discovers solution** via `solution!` macro registration
4. ✅ Downloads input file and creates scaffold from template

**Zero manual changes needed! No SOLUTIONS array modifications!**

## ✅ Adding a New Year (Simple Process)

### Step 1: Create Year Crate
```bash
# Copy existing year structure
cp -r aoc_2024 aoc_2025
```

### Step 2: Update Year Crate
```toml
# aoc_2025/Cargo.toml
[package]
name = "aoc_2025"
# ... rest unchanged
```

```rust
// aoc_2025/src/lib.rs - no more SOLUTIONS array needed!
use common::register_year;

// Import day modules to trigger solution registration
// [import_marker] - keep this marker for new days!

// Register this year with the dynamic registry
// Solutions are automatically discovered from day modules
register_year!(2025);
```

### Step 3: Add to Workspace
```toml
# Cargo.toml - add to workspace dependencies
[workspace.dependencies]
aoc_2025 = { path = "./aoc_2025" }
```

### Step 4: Enable Year in Framework
```toml
# framework/Cargo.toml
[dependencies]
aoc_2025 = { workspace = true, optional = true }

[features]
year_2025 = ["aoc_2025"]
```

```rust
// framework/src/years.rs
#[cfg(feature = "year_2025")]
extern crate aoc_2025;

// In init_years() function:
#[cfg(feature = "year_2025")]
{
    let _ = &aoc_2025::SOLUTIONS;
}
```

### Step 5: Enable by Default (Optional)
```toml
# framework/Cargo.toml
[features]
default = ["year_2024", "year_2025"]  # Add year_2025
```

**That's it!** New days in 2025 will now register automatically with `cargo run -- init`.

## 🎯 Benefits of Dynamic System

### Before (Manual Registration)
- ❌ Edit `framework/src/main.rs` to add year case
- ❌ Edit `framework/Cargo.toml` to add dependency  
- ❌ Edit `aoc_YYYY/src/lib.rs` SOLUTIONS array for each new day
- ❌ Risk of forgetting steps or making mistakes

### After (Fully Automatic Registration)  
- ✅ **New Days**: **100% automatic** - `cargo run -- init` handles everything
- ✅ **New Years**: Simple 5-step process, then 100% automatic days
- ✅ **Type Safety**: Compile-time verification of registrations
- ✅ **Feature Flags**: Enable/disable years as needed
- ✅ **No Runtime Overhead**: Registration happens at program startup
- ✅ **Zero Array Management**: No more SOLUTIONS array maintenance

## 🔧 How It Works

1. **Dynamic Solution Discovery**: Each `solution!` macro automatically registers via `#[ctor::ctor]`
2. **Lazy Year Registration**: Years are registered with lazy functions that discover solutions on-demand
3. **Feature-Based Loading**: Years are optional dependencies controlled by Cargo features
4. **Module-Only Insertion**: Scaffolding only adds module imports - no array management needed

## 🚀 Migration Notes

**Existing years** can be migrated by:
1. Removing the `pub const SOLUTIONS` array from `lib.rs`
2. Adding `register_year!(YEAR);` to their `lib.rs`
3. Adding `ctor` dependency to their `Cargo.toml`  
4. Following the "Enable Year in Framework" steps above

The old manual SOLUTIONS array system has been completely replaced with automatic discovery.