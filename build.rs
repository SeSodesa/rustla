/*!
This build script is compiled before
the actual main program and its dependencies.

Text sent to standard output by this script
is handled in specific ways by Cargo,
depending on how each output line starts.

For full details, see https://doc.rust-lang.org/cargo/reference/build-scripts.html

Copyright © 2020 Santtu Söderholm
*/
fn main() {
    println!("cargo:rustc-env=AUTHOR_NAME=Santtu Söderholm");
    println!("cargo:rustc-env=AUTHOR_EMAIL=santtu.soderholm@tuni.fi");
    println!("cargo:rustc-env=AUTHOR_YEAR=2020");
}
