Rust always confuses me in how you're writing code to both solve for the problem
you want to solve, but also fit it into Rust's memory access constraints.

This will be an implementation of Gale-Shapley's stable marriage algorithm,
since it involves a lot of checking between two composite data structures of
variable length, and involves a lot of moving things around back and forth.

```
cargo test
```
