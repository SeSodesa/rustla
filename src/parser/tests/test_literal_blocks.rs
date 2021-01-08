/*!
A submodule for testing literal blocks.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn literal_block_01() {
    let src = String::from(
        "
::

> This is a literal block of text,
> indicated by the \"::\" at the end of last paragraph.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::LiteralBlock { text } => {
            assert_eq!(text.as_str(), "This is a literal block of text,\nindicated by the \"::\" at the end of last paragraph.")
        }
        _ => panic!(),
    }
}

#[test]
fn literal_block_02() {
    let src = String::from(
        r#"
::

  An indented literal block with
  multiple lines

    Even more indent here.

      And even more...
  Return to original level of indentation

This line ends the literal block, as its indentation is on the same level
as that of the literal block indicator "::".

  "#,
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));
    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::LiteralBlock { text } => {
            assert_eq!(text.as_str(), "An indented literal block with\nmultiple lines\n\n  Even more indent here.\n\n    And even more...\nReturn to original level of indentation\n")
        }
        _ => panic!(),
    }
}

#[test]
fn code_01() {
    let src = String::from(
        r#"
.. code:: python
  :number lines: 3
  :name: reference-name
  :class: some-class and-another-one

  def shout(text):
    print(text + "!")

  def main():
    text = "abcde"
    shout(text)

This paragraph ends the literal block.
"#,
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Code {
            text,
            language,
            number_lines,
            name,
            class,
        } => {
            assert_eq!(text.as_str(), "def shout(text):\n  print(text + \"!\")\n\ndef main():\n  text = \"abcde\"\n  shout(text)\n");
            assert_eq!(language.as_ref().unwrap().as_str(), "python");
            assert_eq!(name.as_ref().unwrap().as_str(), "reference-name");
            assert_eq!(
                class.as_ref().unwrap().as_str(),
                "some-class and-another-one"
            );
        }
        _ => panic!(),
    }
}
