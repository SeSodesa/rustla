/*!
A submodule for testing the regex patterns in `crate::parser::regex_patterns`.

Copyright © 2020 Santtu Söderholm
*/

#[cfg(test)]
#[test]
fn enumerator_01() {
    let test_str = "    (1)    ";
    if let Some(capts) = crate::parser::automata::ENUMERATOR_AUTOMATON.captures(test_str) {
        eprintln!("Yay!?!")
    } else {
        panic!("Nay...")
    }
}
