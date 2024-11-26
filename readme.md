# Advent of Code 2024

A collection of scripts for [Advent of Code](https://adventofcode.com/)

Mixture of Typescript and Rust, depending on what computer I'm on at the time and/or how hard I find it to solve the problem in Rust and want to reach for something comfy instead...

Individual puzzle inputs need to be dropped into `inputs/` with names like `dayN.txt`, where `N` is the day number, eg `day1.txt`. I've gitignore'd mine just so the repo is neat and tidy

## Running rust files

`cd rust`
`cargo run --bin <name of module>`

for example, to run the day 1 solution (which I intend to call `day1.rs`): `cargo run --bin day1`

### Tests

To run the test for a day, run `cargo test --bin <name of module>` from within the rust directory

## Running typescript files

You'll need [pnpm](https://pnpm.io/) installed, then it's as simple as:

`cd typescript`
`pnpm install`
`pnpm run dayN` where `N` is the day number, eg `pnpm run day2` to print the solution for day 2.
