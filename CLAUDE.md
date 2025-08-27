# Advent of Code Project Instructions

## Project Overview
This is a Rust-based Advent of Code solution repository with a custom AOC runner framework. The project contains solutions for multiple years (2021-2024) and includes automated tooling for problem solving and benchmarking.

## Advent of Code Problem Lookup

When working on AOC problems, always reference the official problem descriptions at:
- **Base URL**: `https://adventofcode.com/{year}/day/{day}`
- **Examples**:
  - Day 1, 2023: https://adventofcode.com/2023/day/1
  - Day 15, 2022: https://adventofcode.com/2022/day/15
  - Day 25, 2024: https://adventofcode.com/2024/day/25

### Problem Structure
Each AOC problem consists of:
- **Part 1**: Initial problem with example input/output
- **Part 2**: Extended problem that usually scales up or adds complexity
- **Input file**: Unique personalized input data (requires login)

### Key Problem Categories
Based on the project's comprehensive categorization system:

**Core Categories:**
- `Math`: Number theory, modular arithmetic, combinatorics
- `Grid`: 2D/3D grid traversal, pathfinding on grids
- `Graph`: Tree/graph traversal, topological sorting
- `BFS/DFS`: Search algorithms, connected components
- `DynProg`: Dynamic programming, memoization
- `Strings`: String processing, pattern matching, parsing
- `Grammar`: Parsing, regex, symbolic manipulation
- `Logic`: Constraint satisfaction, logic puzzles
- `Sim`: Process simulation, game implementations
- `Optimize`: Min/max problems, linear programming

## Project Structure

### Workspace Layout
```
├── aoc-runner/          # Custom AOC framework
├── aoc-runner-macros/   # Procedural macros for solutions
├── aoc_2021/           # 2021 solutions (Python + Rust)
├── aoc_2022/           # 2022 solutions (Rust)
├── aoc_2023/           # 2023 solutions (Rust) 
├── aoc_2024/           # 2024 solutions (Rust)
├── aoc_lib/            # Shared utilities
└── assets/             # Visual assets and GIFs
```

### Solution Files
- Pattern: `aoc_{year}/src/day_{NN}.rs` (e.g., `day_01.rs`, `day_15.rs`)
- Each file contains both Part 1 and Part 2 solutions
- Uses custom `solution!` macro for problem setup

## Development Workflow

### Adding New Solutions
1. **Check existing pattern**: Look at recent solutions for current coding style
2. **Reference problem**: Always visit `https://adventofcode.com/{year}/day/{day}` first
3. **Understand input format**: Parse the example carefully before coding
4. **Implement incrementally**: Start with Part 1, validate with examples
5. **Scale for Part 2**: Most Part 2 solutions require algorithmic improvements

### Testing and Running
- **Build**: `cargo build`
- **Test specific day**: `cargo test day_{NN}`
- **Run solution**: `cargo run --bin aoc_{year} -- day {N}`
- **Benchmarks**: Available via criterion integration

### Code Style
- Follow existing Rust idioms in the codebase
- Use workspace dependencies (itertools, regex, petgraph, etc.)
- Leverage shared utilities in `aoc_lib/`
- Comment complex algorithms but keep code clean

## Common AOC Patterns & Libraries

### Frequently Used Crates
- **itertools**: Iterator combinations, grouping, cartesian products
- **regex**: Pattern matching and text parsing
- **petgraph**: Graph algorithms and data structures  
- **pathfinding**: A*, Dijkstra, BFS implementations
- **glam**: Vector math and 3D operations
- **nom**: Parser combinators for complex input formats

### Typical Solution Structure
```rust
use aoc_lib::*; // Common utilities

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    // Parse input
    // Implement algorithm
    // Return result
}

#[aoc(day1, part2)]  
fn part2(input: &str) -> i32 {
    // Often builds on part1 logic
    // Usually requires optimization/scaling
    // Return result
}
```

## Problem-Solving Strategy

### Initial Analysis
1. **Read problem carefully**: Understanding is 80% of the solution
2. **Categorize**: Identify the problem type (Grid, Graph, Math, etc.)
3. **Check examples**: Verify understanding with provided test cases
4. **Plan algorithm**: Consider time/space complexity for Part 2 scaling

### Implementation Tips
- **Start simple**: Brute force Part 1, optimize for Part 2
- **Debug with examples**: Use provided test cases extensively
- **Consider edge cases**: Empty inputs, boundary conditions
- **Profile if needed**: Use benchmarking for performance analysis

### Common Gotchas
- **Off-by-one errors**: Especially in grid/array indexing
- **Input parsing**: AOC inputs often have tricky formats
- **Integer overflow**: Use appropriate numeric types
- **Part 2 scaling**: Linear solutions rarely work, need O(log n) or better

## Helper Commands

### Useful Build Commands
```bash
# Build entire workspace
cargo build --release

# Test specific year
cargo test --package aoc_2023

# Run with optimizations
cargo run --release --bin aoc_2024

# Check all code
cargo check --workspace
```

### Framework Usage
The custom AOC runner provides:
- Automatic input parsing
- Timing measurements  
- Multiple output formats
- Integrated benchmarking

## Resources

### External References
- **AOC Website**: https://adventofcode.com
- **Rust AOC Guide**: https://gendignoux.com/blog/2019/08/25/rust-advent-of-code.html
- **Algorithm Reference**: Use README.md tables for problem categorization
- **Parsing Library**: Chumsky for complex grammar problems

### Performance Considerations
- Most problems designed to run in <1 second on modern hardware
- Part 2 usually requires algorithmic insight, not just faster code
- Consider mathematical properties before implementing brute force
- Profile only after algorithmic optimization

## Notes for Claude

- Always check the specific year/day URL before implementing solutions
- Reference existing solutions in the same year for coding patterns  
- Use the categorization system from README.md to understand problem types
- Consider both Part 1 and Part 2 requirements during initial design
- Test with provided examples before submitting solutions