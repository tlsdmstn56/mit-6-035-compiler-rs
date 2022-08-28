# MIT 6-035 Compiler in Rust

Rust implementation of compiler project in [mit 6-035 computer language engineering (Spring 2010)](https://ocw.mit.edu/courses/6-035-computer-language-engineering-spring-2010/)

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Design

* Scanner/ Parser: use [lalrpop](https://github.com/lalrpop/lalrpop)
* Semantic Analyzer: TBD
* Code Generation: TBD
* Dataflow Optimizer: TBD
