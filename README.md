# ruSTLa - rSTLa in Rust

ruSTLa is an implementation of the rSTLa (reStructuredText ⟶ $`\LaTeX`$) transpiler,
written in the Rust programming language.
rSTLa itself is an inverse transpiler to the LarST ($`\LaTeX`$ ⟶ reStructuredText)
transpiler written by [Tomi Janhunen](https://www.tuni.fi/fi/tomi-janhunen).

## Build instructions

If you wish to build the project yourself, the easiest way to do it is to install [`rustup`](https://rustup.rs/), reboot your computer so the necessary `PATH` modifications come into effect, navigate to the project folder and run (`cargo build`|`cargo run path/to/rst/file.rst`). To run the unit tests, type `cargo test`. Running a specific test includes typing `cargo test path::to::test::function`. Type `cargo test path::to::test::function -- --nocapture` if you wish to view test output.

## Usage on a machine without Cargo

The program can be run in the terminal without any options by navigating to the folder with the `rustla` binary and typing
```
$ ./rustla path/to/rst/file.rst
```
Note the required source file suffix `.rst`:
rusTLa is opinionated in this way to protect the user from accidentally overwriting the source file with the object file.

Alternatively, one might move the binary to one of the folders listed in the `PATH` environment variable
and restarting the terminal or logging out, if your system requires this in order for the changes to `PATH`
to become effective. This allows it to be run from anywhere by simply typing
```
$ rustla path/to/rst/file.rst
```

Options can be given to `rustla` by typing the respective `key`--`value` pairs *before* the reStructuredText source file path:
```
$ ./rustla key1 value1 key2 value2 ... keyN valueN path/to/rst/file.rst
```

The recognized options are given in the following listing:
```
Option              Known values and explanation
======              ============================

output-stream       stdout|file

                    The option "stdout" is self-explanatory:
                    it directs the program output to the standard output of rustla.
                    This is the default functionality if "output-stream" is not specified.

                    The option "file" creates a new reStructuredText file next to the source file,
                    with the same name except for the suffix ".rst", which is replaced with ".tex".
                    There is currently no way to prevent this object file from being overwritten,
                    so care should be taken when running the program.
```

## Project structure

The current structure of the project is given below.
This is subject to change as the project advances further.
```bash
src
├── bin
├── common.rs
├── doctree
│   ├── directives.rs
│   ├── hyperref_data.rs
│   ├── larst_writer.rs
│   ├── mod.rs
│   ├── node_categories.rs
│   ├── restructuredtext_transforms.rs
│   ├── section_data.rs
│   ├── tests.rs
│   ├── test_walkers.rs
│   ├── tree_node.rs
│   ├── tree_node_types.rs
│   ├── tree_zipper.rs
│   └── walkers.rs
├── main.rs
├── parser
│   ├── converters.rs
│   ├── directive_parsers.rs
│   ├── line_cursor.rs
│   ├── mod.rs
│   ├── state_machine
│   │   ├── aplus_questionnaire.rs
│   │   ├── aplus.rs
│   │   ├── block_quote.rs
│   │   ├── body.rs
│   │   ├── bullet_list.rs
│   │   ├── common.rs
│   │   ├── definition_list.rs
│   │   ├── enumerated_list.rs
│   │   ├── field_list.rs
│   │   ├── footnote.rs
│   │   ├── inline.rs
│   │   ├── mod.rs
│   │   ├── transitions.rs
│   │   └── unknown_transitions.rs
│   ├── tests
│   │   ├── mod.rs
│   │   ├── test_admonitions.rs
│   │   ├── test_aplus_point_of_interest.rs
│   │   ├── test_aplus_questionnaire.rs
│   │   ├── test_block_quotes.rs
│   │   ├── test_block_reading.rs
│   │   ├── test_bullet_lists.rs
│   │   ├── test_comments.rs
│   │   ├── test_converters.rs
│   │   ├── test_definition_lists.rs
│   │   ├── test_enumerated_lists.rs
│   │   ├── test_field_lists.rs
│   │   ├── test_hyperlink_targets.rs
│   │   ├── test_images.rs
│   │   ├── test_inline_parsing.rs
│   │   ├── test_list_tables.rs
│   │   ├── test_literal_blocks.rs
│   │   ├── test_math_blocks.rs
│   │   ├── test_mixed_structures.rs
│   │   ├── test_sections_and_transitions.rs
│   │   ├── test_sphinx_only.rs
│   │   └── test_unknown_directives.rs
│   └── types_and_aliases.rs
└── utf8_to_latex.rs

5 directories, 57 files
```
