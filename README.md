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
├── common.rs
├── doctree
│   ├── directives.rs
│   ├── hyperref_data.rs
│   ├── mod.rs
│   ├── tests.rs
│   └── tree_zipper.rs
├── main.rs
└── parser
    ├── converters.rs
    ├── line_cursor.rs
    ├── mod.rs
    ├── state_machine
    │   ├── body.rs
    │   ├── bullet_list.rs
    │   ├── common.rs
    │   ├── enumerated_list.rs
    │   ├── field_list.rs
    │   ├── footnote.rs
    │   ├── inline.rs
    │   ├── mod.rs
    │   └── transitions.rs
    ├── test_block_reading.rs
    ├── test_bullet_lists.rs
    ├── test_converters.rs
    ├── test_enumerated_lists.rs
    ├── test_field_lists.rs
    ├── test_hyperlink_targets.rs
    ├── test_inline_parsing.rs
    ├── test_mixed_structures.rs
    └── types_and_aliases.rs

4 directories, 28 files
```
If you wish to build the project yourself, the easiest way to do it is to install [rustup](https://rustup.rs/), reboot your computer so the necessary `PATH` modifications come into effect, navigate to the project folder and run (`cargo build`|`cargo run`). To run the unit tests, type `cargo test`.