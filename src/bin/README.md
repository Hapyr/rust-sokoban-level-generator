# Sokoban Trace Generation Scripts

This directory contains three Rust scripts for generating different types of Sokoban game traces using A* search algorithm.

## Scripts Overview

### 1. `suboptimal_trace_generation_multi.rs`
**Purpose**: Generates multiple suboptimal traces for multiple levels in parallel.

**Features**:
- Multi-threaded processing using Rayon
- Command-line argument support via Clap
- Saves traces to files in `game_states/` directory
- Processes multiple levels with multiple iterations per level

**Usage**:
```bash
cargo run --bin suboptimal_trace_generation_multi -- [OPTIONS]

Options:
  -t, --threads <THREADS>      Number of threads to use [default: system cores]
  -l, --levels <LEVELS>        Number of levels to generate [default: 10]
  -i, --iterations <ITERATIONS> Number of A* search iterations per level [default: 3]
```

**Output Format**:
For each level, 1 or more suboptimal traces get generated. All traces for one level are stored in the same file. The file is named `game_{level_id}.txt`. A state is encoded in a one line string. Each trace gets a heading:
- If the heading is formatted like `--{}--`, it is an already solved instance
- If the heading is formatted like `__{}__`, it is a suboptimal trace
- If the heading is formatted like `++{}++`, it is a failed attempt

The number in the heading is the iteration number.

### 2. `suboptimal_trace_generation_single.rs`
**Purpose**: Generates suboptimal traces for a single level and displays them on the console.

**Features**:
- Generates one level
- Runs some iterations of A* search with suboptimal parameters
- Displays results in a human-readable format on the console
- Shows both actions and pretty-printed board states

**Usage**:
```bash
cargo run --bin suboptimal_trace_generation_single
```
single
- Search depth: 100

### 3. `optimal_trace.rs`
**Purpose**: Generates an optimal trace for a single level using A* search.

**Features**:
- Generates one level with 2x2 grid and 1 box
- Runs a single A* search with optimal parameters
- Displays the optimal solution path on the console
- Shows both actions and pretty-printed board states

**Usage**:
```bash
cargo run --bin optimal_trace
```

**Parameters Used**:
- Uses optimal A* parameters: `get_successor(false, 0.0, true, false, 0.0)`
- Search depth: 40

## A* Search Parameters

The `get_successor` method takes parameters that control the search behavior:
- **Suboptimal mode**: Uses randomization and exploration to find varied solutions
- **Optimal mode**: Focuses on finding the shortest possible solution

This allows generating diverse training data for machine learning models that need to learn from both optimal and suboptimal gameplay examples.
