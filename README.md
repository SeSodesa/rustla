# ruSTLa - rSTLa in Rust

ruSTLa is an implementation of the rSTLa
(reStructuredText to $`\LaTeX`$) transpiler,
written in the Rust programming language.
rSTLa itself is an inverse transpiler to the LarST ($`\LaTeX`$ to reStructuredText) transpiler written by [Tomi Janhunen](https://www.tuni.fi/fi/tomi-janhunen).

The current structure of the project is given below.
This is subject to change as the project advances further.
```bash
src/
├── bin
├── lexer
│   ├── error
│   │   ├── mod.rs
│   │   └── tests
│   │       └── mod.rs
│   ├── mod.rs
│   ├── regex
│   │   ├── mod.rs
│   │   ├── test
│   │   └── titles
│   │       ├── mod.rs
│   │       └── test
│   │           └── mod.rs
│   ├── tests
│   │   └── mod.rs
│   └── token
│       ├── mod.rs
│       └── tests
│           └── mod.rs
└── main.rs

11 directories, 10 files
```
If you wish to build the project yourself, the easiest way to do it is to install [rustup](https://rustup.rs/), reboot your computer so the necessary `PATH` modifications come into effect, navigate to the project folder and run (`cargo build`|`cargo run`).