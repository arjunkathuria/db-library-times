# db-times-diesel

This project runs benchmarks on querying the popular Sakila DB in a mysql database using Rust.

It currently uses the libraries:-

1. Diesel

There maybe additions to these in the future.

There are implementations using these in the `src` folder.

## What it does currently
Current  implementaion queries the `film` table in `sakila DB`, which contains a total of 1000 rows and 5 columns (as implemented, it uses 5 columns from the film table, the film table itself has more columns, which are not used here).

The benchmarks measure the time it takes to progressively query a single row, then 1000 rows (all rows) from this 5-column table, using the Diesel library.

## Requirements

### OS
This was build and tested on linux.

### Tools

The project uses:-
- The rust toolchain (rustc, cargo, rustfmt etc.), be sure to get a recent version, Diesel version used required rustc version 1.65+.
- `cargo` build tool
- `rust-fmt` for formatting
- `mysql` database

### Databases
Currently, It expects a database named `sakila`. Please follow the link at the top level readme to install that.

## Instructions to build, run and benchmark

### Benchmark
The project can run the required benchmarks by simply running `cargo bench` in this folder

### Main app
The main app doesn't do much and serves as a playground to first run and check queries, verfying their correctness, before using them in benchmarks. A sample of debug query has been left in the main app for reference.
The main app can be ran with the command `cargo run`

## Benchmark results

The following benchmark results were observed:- 

**Rust - Diesel**

results from running `cargo bench`

```
get 1 Film              time:   [14.605 µs 14.683 µs 14.763 µs]
                        change: [-10.260% -9.4196% -8.5453%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

get 1000 Films          time:   [542.74 µs 543.13 µs 543.62 µs]
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) high mild
  14 (14.00%) high severe

```
