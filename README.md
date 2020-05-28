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
│   ├── body_actions
│   │   ├── block_tests.rs
│   │   ├── comment_tests.rs
│   │   ├── directive_tests.rs
│   │   ├── list_tests.rs
│   │   ├── mod.rs
│   │   ├── ref_target_tests.rs
│   │   └── title_tests.rs
│   ├── error
│   │   ├── mod.rs
│   │   └── tests
│   │       └── mod.rs
│   ├── inline_actions
│   │   └── mod.rs
│   ├── mod.rs
│   ├── state
│   │   └── mod.rs
│   ├── tests
│   │   └── mod.rs
│   ├── token
│   │   ├── mod.rs
│   │   └── tests
│   │       └── mod.rs
│   └── token_mappings
│       ├── body
│       │   ├── block_tests.rs
│       │   ├── comment_tests.rs
│       │   ├── directive_tests.rs
│       │   ├── list_tests.rs
│       │   ├── mod.rs
│       │   ├── ref_target_tests.rs
│       │   └── title_tests.rs
│       ├── inline
│       │   └── mod.rs
│       └── mod.rs
└── main.rs

13 directories, 25 files
```
If you wish to build the project yourself, the easiest way to do it is to install [rustup](https://rustup.rs/), reboot your computer so the necessary `PATH` modifications come into effect, navigate to the project folder and run (`cargo build`|`cargo run`). To run the unit tests, type `cargo run`.