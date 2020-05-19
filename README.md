# ruSTLa - rSTLa in Rust

ruSTLa is an implementation of the rSTLa
(reStructuredText to $`\LaTeX`$) transpiler,
written in the Rust programming language.
rSTLa itself is an inverse transpiler to the LarST ($`\LaTeX`$ to reStructuredText) transpiler written by [Tomi Janhunen](https://www.tuni.fi/fi/tomi-janhunen).

The current structure of the project is given below.
This is subject to change as the project advances furher.
```bash
src
├── lexer
│   ├── error
│   │   ├── mod.rs
│   │   └── tests
│   │       └── mod.rs
│   ├── mod.rs
│   ├── tests
│   │   └── mod.rs
│   └── token
│       ├── mod.rs
│       └── tests
│           └── mod.rs
└── main.rs

6 directories, 7 files
```
