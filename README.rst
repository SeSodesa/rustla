ruSTLa - rSTLa in Rust
======================

ruSTLa is an implementation of the rSTLa (reStructuredText → LaTeX) transpiler,
written in the Rust programming language. rSTLa itself is an inverse transpiler to the
LarST (LaTeX → reStructuredText) transpiler written by `Tomi Janhunen`_.

.. _`Tomi Janhunen`: https://www.tuni.fi/fi/tomi-janhunen

ruSTLa was originally written as the "practical part"
of Santtu Söderholm's Master's Thesis. In other words:

    Copyright © 2020 Santtu Söderholm

Build instructions
------------------

If you wish to build the project yourself, the easiest way to do it is to install `rustup`_,
reboot your computer so the necessary `PATH` modifications come into effect,
navigate to the project folder and run ::

    cargo build [--release] | cargo run path/to/rst/file.rst

To run the unit tests, type `cargo test`. Running a specific test includes typing ::

    cargo test path::to::test::function

Type ::

    cargo test path::to::test::function -- --nocapture

if you wish to view test output. See `Cargo documentation`_ for more options.

.. _`rustup`: https://rustup.rs/
.. _`Cargo documentation`: https://doc.rust-lang.org/cargo/commands/cargo-build.html

Usage on a machine without Cargo
--------------------------------

The program can be run in the terminal without any options by navigating to the folder with the `rustla` binary and typing::

    $ ./rustla path/to/rst/file.rst

Note the required source file suffix `.rst`:
rusTLa is opinionated in this way to protect the user from accidentally overwriting the source file with the object file.

Alternatively, one might move the binary to one of the folders listed in the `PATH` environment variable
and restarting the terminal or logging out, if your system requires this in order for the changes to `PATH`
to become effective. This allows it to be run from anywhere by simply typing::

    $ rustla path/to/rst/file.rst


Options can be given to `rustla` via different flags, specified *before* the reStructuredText source file path::

    $ ./rustla --flag1 --flag2 ... --flagN path/to/rst/file.rst


The recognized flags are given in the following listing::

    Option              Known values and explanation
    ===========         ============================

    --to-stdout         The option "stdout" is self-explanatory:
                        it directs the program output to the standard output of rustla.
                        This is the default functionality if "output-stream" is not specified.

    --to-file           The option "file" creates a new reStructuredText file next to the source file,
                        with the same name except for the suffix ".rst", which is replaced with ".tex".
                        There is currently no way to prevent this object file from being overwritten,
                        so care should be taken when running the program with this flag set.

    --full-doc          If this is set, the resulting output will be surrounded by the string::

                            \documentclass{aplus}
                            \begin{document}
                            <output>
                            \end{document}

    --aplus-cls         This option generates an aplus.cls file next to the file generated,
                        when the flag --to-file is set.


Project structure
-----------------

The current structure of the project is given below::

    src/
    ├── common.rs
    ├── doctree
    │   ├── class_data.rs
    │   ├── directives.rs
    │   ├── hyperref_data.rs
    │   ├── larst_writer.rs
    │   ├── mod.rs
    │   ├── node_categories.rs
    │   ├── restructuredtext_transforms.rs
    │   ├── section_data.rs
    │   ├── tests
    │   │   ├── mod.rs
    │   │   ├── test_constructor.rs
    │   │   └── test_walkers.rs
    │   ├── tree_node.rs
    │   ├── tree_node_types.rs
    │   ├── tree_zipper.rs
    │   └── walkers.rs
    ├── main.rs
    ├── parser
    │   ├── automata.rs
    │   ├── converters.rs
    │   ├── directive_parsers.rs
    │   ├── line_cursor.rs
    │   ├── mod.rs
    │   ├── regex_patterns.rs
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
    │   │   ├── inline.rs
    │   │   ├── literal_block.rs
    │   │   ├── mod.rs
    │   │   ├── transitions.rs
    │   │   └── unknown_transitions.rs
    │   ├── table_parsers.rs
    │   ├── tests
    │   │   ├── mod.rs
    │   │   ├── test_admonitions.rs
    │   │   ├── test_aplus_point_of_interest.rs
    │   │   ├── test_aplus_questionnaire.rs
    │   │   ├── test_block_quotes.rs
    │   │   ├── test_block_reading.rs
    │   │   ├── test_bullet_lists.rs
    │   │   ├── test_class.rs
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
    │   │   ├── test_regexes.rs
    │   │   ├── test_sections_and_transitions.rs
    │   │   ├── test_sphinx_only.rs
    │   │   └── test_unknown_directives.rs
    │   └── types_and_aliases.rs
    ├── rustla_options.rs
    └── utf8_to_latex.rs

    6 directories, 65 files


This is subject to change as the project advances further.
