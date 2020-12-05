/// ## Test regexes
///
/// A submodule for testing the automata located in `crate::parser::automata`.
///
/// (c) 2020 Santtu Söderholm <santtu.soderholm@tuni.fi>

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
