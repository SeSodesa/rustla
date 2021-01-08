/*!
A submodule for testing A+ questionnaire directrives.

Copyright © 2020 Santtu Söderholm
*/


use super::*;

#[cfg(test)]
#[test]
fn aplus_questionnaire_01() {
    let src =
r#"
.. questionnaire:: 1 A
  :submissions: 4
  :points-to-pass: 0

  This is a questionnaire with the key `1` that grants at maximum 70 points
  of difficulty A. Students can make at most 4 submissions.
  This exercise is marked passed when 0 points are reached (the default).

  .. pick-one:: 10
    :required:

    What is 1+1?

    a. 1
    *b. 2
    c. 3

    !b § Count again!
    b § That is correct!
    c § Too much

  (Hints can be included or omitted in any question.)

  .. pick-one:: 10
    :required:
    :dropdown:

    What is 1+2?

    +0. 0
    1. 1
    2. 2
    *3. 3

  .. pick-any:: 10
    :partial-points:

    Pick the two **first**. Since the 'partial-points' option is set,
    some points are awarded with a partially correct answer. If either one of the
    correct options is not chosen or one of the wrong fields is chosen, 5 points are
    still awarded. Selecting the last neutral option does not affect the points.

    +*a. this is the **first**
    *b. this is the **second**
    c. this is the **third**
    d. this is the **fourth**
    ?e. choosing this does not affect the granted points

  .. freetext:: 30 string-ignorews-ignorequotes-requirecase
    :length: 10

    A textual input can be compared with the model solution as integer, float or string.
    Here the correct answer is "test". Surrounding quotes are ignored in the solution
    as well as whitespace everywhere (modifiers ignorequotes and ignorews).

    test
    !test § Follow the instruction.
    regexp:Test|TEST § Use the lower case!

  .. freetext:: 10 regexp

    This question accepts either "red" or "blue" as the correct answer.
    The model solution is a regular expression.

    red|blue
"#
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::AplusQuestionnaire { .. } = doctree
        .shared_child(0).unwrap().shared_data() {
    } else {
        panic!()
    }
    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusPickOne { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap().shared_data() {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusPickOne { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(3).unwrap().shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusPickAny { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap().shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusFreeText { .. } =
        doctree
        .shared_child(0).unwrap()
        .shared_child(5).unwrap().shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusFreeText { .. } =
        doctree
        .shared_child(0).unwrap()
        .shared_child(6).unwrap().shared_data()
    {
    } else {
        panic!()
    }
}

#[test]
fn aplus_questionnaire_02() {
    let src =
        r#"
(K) Suostumus anonyymin kurssidatan tutkimuskäyttöön
====================================================

Kurssin aikana kurssin tietojärjestelmiin (Plussa, GitLab) syntyy
opiskelijoiden tuottamaa dataa.
Yliopiston yksi tehtävistä on tutkimus, ja siten tätä dataa halutaan
hyödyntää opetuksen ja ohjelmistokehityksen tutkimustyössä.
Tämä parantaa opetuksen laatua ja luo uutta tietoa.
Kaikki data käsitellään tutkimuksissa anonyymisti, eikä yksittäistä
opiskelijaa voi tunnistaa.

Kurssi pyytää siten jokaiselta opiskelijalta suostumuksen datan käyttöön.

.. questionnaire:: gdpr 10
  :category: N
  :submissions: 2
  :points-to-pass: 0


  .. pick-one:: 10

    Annan luvan käyttää kurssidataa anonymisoituna tutkimustarkoituksiin

    *a. Kyllä
    *b. En

"#
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::AplusQuestionnaire { .. } =
        doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap().shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusPickOne { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusPickChoices { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusPickChoice { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusPickChoice { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
    } else {
        panic!()
    }
}
