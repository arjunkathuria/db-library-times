# beam-times

This project runs benchmarks on querying the popular Sakila DB in a mysql database using Haskell.

It currently uses the libraries:-

1. Beam + Mysql-haskell (juspay forks)
2. Mysql-Simple (upstream)

There maybe additions to these in the future.

There are implementations using these in the `src` folders, marked with the library's name.

## What it does currently
Currently, both implementaions query the `film` table in `sakila DB`, which contains a total of 1000 rows and 5 columns (as implemented, it uses 5 columns from the film table, the film table itself has more columns, which are not used here).

The benchmarks measure the time it takes to progressively query a single row, then 10 rows, the a 100 and the finally a thousand (all rows) from this 5-column table, using both the libraries.

## Requirements

### OS
This was build and tested on linux. Since this uses GHC 8.8.4, it might need some mods to run on aarch64 MacOS. If you're on aarch64 MacOS and don't want to modify this, you may try running it under Rosetta.

### Tools

The project uses:-
- the `stack` build tool and lts-16.31 (GHC 8.8.4) snapshot, for building and benchmarking the project.
- `ormolu` for formatting
- `mysql` database

### Databases
Currently, It expects a database named `sakila`. Please follow the link at the top level readme to install that.

## Instructions to build, run and benchmark

### Benchmark
The project can run the required benchmarks by simply running `stack bench` in this folder

### Main app
The main app doesn't do much and serves as a playground to first run and check queries, verfying their correctness, before using them in benchmarks. A sample of debug queries has been left in the main app for reference.
The main app can be ran with the command `stack run`

## Benchmark results

The following benchmark results were observed:- 

**Beam + mysql-haskell**

results from running `stack bench`

```
Benchmark query-bench: RUNNING...
benchmarking BEAM///get one Film - 5 cols
time                 19.33 μs   (18.79 μs .. 20.03 μs)
                     0.995 R²   (0.989 R² .. 1.000 R²)
mean                 19.09 μs   (18.93 μs .. 19.46 μs)
std dev              812.3 ns   (439.3 ns .. 1.463 μs)
variance introduced by outliers: 50% (severely inflated)
                        
benchmarking BEAM///get ten Films - 5 cols
time                 40.32 μs   (39.11 μs .. 41.26 μs)
                     0.996 R²   (0.994 R² .. 0.998 R²)
mean                 38.90 μs   (38.35 μs .. 39.62 μs)
std dev              2.081 μs   (1.585 μs .. 2.584 μs)
variance introduced by outliers: 60% (severely inflated)
                        
benchmarking BEAM///get 100 Films after the first 100 - 5 cols
time                 257.7 μs   (252.9 μs .. 264.8 μs)
                     0.996 R²   (0.992 R² .. 0.999 R²)
mean                 263.7 μs   (260.2 μs .. 269.2 μs)
std dev              15.12 μs   (10.58 μs .. 22.79 μs)
variance introduced by outliers: 54% (severely inflated)
                        
benchmarking BEAM///get one thousand Films - 5 cols
time                 2.238 ms   (2.219 ms .. 2.254 ms)
                     0.998 R²   (0.993 R² .. 1.000 R²)
mean                 2.275 ms   (2.249 ms .. 2.347 ms)
std dev              132.4 μs   (59.10 μs .. 269.8 μs)
variance introduced by outliers: 42% (moderately inflated)

```

And

**Mysql-Simple**

results from running `stack bench`

```
benchmarking MySQL Simple///get one Film - 5 cols
time                 19.33 μs   (19.22 μs .. 19.45 μs)
                     1.000 R²   (1.000 R² .. 1.000 R²)
mean                 19.40 μs   (19.32 μs .. 19.52 μs)
std dev              318.1 ns   (248.5 ns .. 415.4 ns)
variance introduced by outliers: 13% (moderately inflated)
                        
benchmarking MySQL Simple///get Ten Films - 5 cols
time                 66.82 μs   (66.67 μs .. 67.13 μs)
                     1.000 R²   (0.999 R² .. 1.000 R²)
mean                 68.24 μs   (67.72 μs .. 68.79 μs)
std dev              1.708 μs   (1.610 μs .. 1.802 μs)
variance introduced by outliers: 22% (moderately inflated)
                        
benchmarking MySQL Simple///get Hundred Films - 5 cols
time                 598.8 μs   (580.1 μs .. 614.3 μs)
                     0.996 R²   (0.995 R² .. 0.998 R²)
mean                 573.9 μs   (568.6 μs .. 582.5 μs)
std dev              21.14 μs   (14.48 μs .. 28.44 μs)
variance introduced by outliers: 28% (moderately inflated)
                        
benchmarking MySQL Simple///get one thousand Films - 5 cols
time                 5.738 ms   (5.650 ms .. 5.832 ms)
                     0.999 R²   (0.998 R² .. 1.000 R²)
mean                 5.519 ms   (5.466 ms .. 5.570 ms)
std dev              163.4 μs   (137.9 μs .. 202.2 μs)
variance introduced by outliers: 12% (moderately inflated)
                        
Benchmark query-bench: FINISH
```


