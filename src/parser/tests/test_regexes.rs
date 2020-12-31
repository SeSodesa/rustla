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

#[test]
fn footnote_01 () {
    let test_strs = [
        ".. [1] ",
        ".. [#] ",
        ".. [#asd] ",
        ".. [*] ",
    ];
    for label in test_strs.iter() {
        if let None = crate::parser::automata::FOOTNOTE_AUTOMATON.captures(label) {
            panic!("Did not recognize \"{}\" as a footnote label. Computer says no...", label)
        }
    }
}
