# loop_rerolling_egg
Repository for research on hardware decompilation (loop rerolling) through rewrite rules.

## Building
### Preliminary Rewrites using `Egg` Rust Library
* `cd` to root of the repo
* `cargo install --path .`
* `cargo build` - build main

### Egglog
* `cd ./egglog` - make sure you are in the `egglog` directory from the root of the repo
* `cargo build`

## Running
### Preliminary Rewrites using `Egg` Rust Library
* `cd` to root of the repo
* `cargo run` - run sanity check
* `cargo test` - run test cases

### Using Egglog on .egg DSL files
* `cd ./egglog`
* `cargo run ../tool/tests/[filename]` - files that test the decomp tool's functionality
* `cargo run ../playground/egglog_examples/[filename]` - test files from the official Egglog repository that I have annotated as I learned how to use the framework

## Repository Info
Currently, `src/main.rs` contains `egg` DSLs used in the rewrites and corresponding unit tests. The adder detection logic here is incomplete as I have since moved to `Egglog.`
The `egglog` subrepository is the official `egglog` repository, included for convenience.  
Keep it up to date.

Files in `playground` test the Egglog framework. Files in `tool` test logic to identify n-bit adders, and consist of the test suite mentioned in my report.
* `/tool/maki_decomp.egg` - Egglog program I developed as the main component of my master's project
* `/tool/maki_build.egg` - incomplete exploration of using Egglog to convert a synthesized Netlist to Maki intermediate language used in `maki_decomp.egg`, may update in the future
