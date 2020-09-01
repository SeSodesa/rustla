# ruSTLa - rSTLa in Rust

ruSTLa is an implementation of the rSTLa
(reStructuredText to $`\LaTeX`$) transpiler,
written in the Rust programming language.
rSTLa itself is an inverse transpiler to the LarST ($`\LaTeX`$ to reStructuredText) transpiler written by [Tomi Janhunen](https://www.tuni.fi/fi/tomi-janhunen).

## Project structure

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
│   ├── node_categories.rs
│   ├── section_data.rs
│   ├── tests.rs
│   ├── tree_node.rs
│   ├── tree_node_types.rs
│   └── tree_zipper.rs
├── main.rs
└── parser
    ├── converters.rs
    ├── directive_parsers.rs
    ├── line_cursor.rs
    ├── mod.rs
    ├── state_machine
    │   ├── block_quote.rs
    │   ├── body.rs
    │   ├── bullet_list.rs
    │   ├── common.rs
    │   ├── definition_list.rs
    │   ├── enumerated_list.rs
    │   ├── field_list.rs
    │   ├── footnote.rs
    │   ├── inline.rs
    │   ├── mod.rs
    │   └── transitions.rs
    ├── test_admonitions.rs
    ├── test_block_quotes.rs
    ├── test_block_reading.rs
    ├── test_bullet_lists.rs
    ├── test_comments.rs
    ├── test_converters.rs
    ├── test_definition_lists.rs
    ├── test_enumerated_lists.rs
    ├── test_field_lists.rs
    ├── test_hyperlink_targets.rs
    ├── test_images.rs
    ├── test_inline_parsing.rs
    ├── test_literal_blocks.rs
    ├── test_mixed_structures.rs
    ├── test_sections_and_transitions.rs
    └── types_and_aliases.rs

4 directories, 42 files
```

## Build instructions

If you wish to build the project yourself, the easiest way to do it is to install [`rustup`](https://rustup.rs/), reboot your computer so the necessary `PATH` modifications come into effect, navigate to the project folder and run (`cargo build`|`cargo run`). To run the unit tests, type `cargo test`. Running a specific test includes typing `cargo test path::to::test::function`. Type `cargo test path::to::test::function -- --nocapture` if you wish to view test output.