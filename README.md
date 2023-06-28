# README

This repo contains the source code for producing benchmarks of running queries on a mysql database with different libraries.

This includes implementations for:-

* Haskell
  - Beam + Beam-Mysql + Mysql-Haskell Juspay forks - [link](https://github.com/juspay/beam), [link](https://github.com/juspay/beam-mysql) and [link](https://github.com/juspay/mysql-haskell)
  - Mysql-Simple - [link](https://hackage.haskell.org/package/mysql-simple)
  
* Rust
  - Diesel - [link](https://diesel.rs/)

This uses the popular sample database - Sakila DB - a database containing films, actors and other related information. It can be found [here](https://dev.mysql.com/doc/sakila/en/). Be sure to follow the installation instructions at the link along with a functional mysql installation on your system to run these repos.

The Benchmarks make use of the `Criterion` library, both on Haskell and Rust sides.

Haskell project can be found at the `beam-times` folder, while the `db-times-disel` folder contains the Rust project.
you may find more specific information and instructions inside the projects' folders own readme files.
