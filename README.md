# ruSTLa - rSTLa in Rust

ruSTLa is an implementation of the rSTLa
(reStructuredText to $`\LaTeX`$) transpiler,
written in the Rust programming language.
rSTLa itself is an inverse transpiler to the LarST ($`\LaTeX`$ to reStructuredText) transpiler written by [Tomi Janhunen](https://www.tuni.fi/fi/tomi-janhunen).

The current structure of the project is given below.
This is subject to change as the project advances further.
```bash
src
├── bin
├── doctree
│   ├── body_nodes.rs
│   ├── mod.rs
│   ├── tests.rs
│   └── tree_zipper.rs
├── main.rs
├── parser
│   ├── converters.rs
│   ├── mod.rs
│   ├── state_machine
│   │   ├── body.rs
│   │   ├── bullet_list.rs
│   │   ├── common.rs
│   │   ├── enumerated_list.rs
│   │   ├── inline.rs
│   │   ├── list_item.rs
│   │   ├── mod.rs
│   │   ├── tests.rs
│   │   └── transitions.rs
│   ├── tests.rs
│   └── types_and_aliases.rs
└── utils.rs

4 directories, 19 files
```
If you wish to build the project yourself, the easiest way to do it is to install [rustup](https://rustup.rs/), reboot your computer so the necessary `PATH` modifications come into effect, navigate to the project folder and run (`cargo build`|`cargo run`). To run the unit tests, type `cargo test`.