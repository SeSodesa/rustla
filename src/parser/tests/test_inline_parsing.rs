/*!
A submodule for testing inline parsing functions.

Copyright © 2020 Santtu Söderholm
*/

use super::*;
use crate::common::Reference;

#[cfg(test)]
#[test]
fn literal_and_strong_emphasis_01() {
    let src =
        String::from("This is a string with\n a ``literal``, **strong emphasis** and normal text");
    let mut lc = LineCursor::new(0, 0);

    let nodes = match Parser::inline_parse(src, None, &mut lc) {
        InlineParsingResult::Nodes(nodes) => nodes,
        _ => panic!("No nodes to be found!"),
    };

    eprintln!("{:#?}", nodes);

    assert_eq!(
        if let TreeNodeType::Literal { text } = &nodes[12] {
            text.as_str()
        } else {
            panic!()
        },
        "literal"
    );

    assert_eq!(
        if let TreeNodeType::StrongEmphasis { text } = &nodes[15] {
            text.as_str()
        } else {
            panic!()
        },
        "strong emphasis"
    );
}

#[test]
fn references_01() {
    let src = String::from(
        "This is a string with a simple-reference+with:punctuation_\nand a `phrase reference`_",
    );

    let mut lc = LineCursor::new(0, 0);

    let nodes = match Parser::inline_parse(src, None, &mut lc) {
        InlineParsingResult::Nodes(nodes) => nodes,
        _ => panic!("No nodes to be found!"),
    };

    eprintln!("{:#?}", nodes);

    if let TreeNodeType::Reference { reference, .. } = &nodes[12] {
        if let Reference::Internal(ref_str) = reference {
            assert_eq!(ref_str, "simple-reference+with:punctuation");
        } else {
            panic!()
        }
    } else {
        panic!()
    }
    if let TreeNodeType::Reference { reference, .. } = &nodes[18] {
        if let Reference::Internal(ref_str) = reference {
            assert_eq!(ref_str, "phrase reference");
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

#[test]
fn references_02() {
    let src = String::from("Here is a simple-reference_ and a `not so simple refereNce`_.");
    let mut lc = LineCursor::new(0, 0);

    let nodes = match Parser::inline_parse(src, None, &mut lc) {
        InlineParsingResult::Nodes(nodes) => nodes,
        _ => panic!("No nodes to be found!"),
    };

    eprintln!("{:#?}", nodes);

    if let TreeNodeType::Reference { reference, .. } = &nodes[6] {
        if let Reference::Internal(ref_str) = reference {
            assert_eq!(ref_str, "simple-reference");
        } else {
            panic!()
        }
    } else {
        panic!()
    }

    if let TreeNodeType::Reference {
        reference,
        displayed_text,
    } = &nodes[12]
    {
        if let Reference::Internal(ref_str) = reference {
            assert_eq!(ref_str, "not so simple reference");
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

#[test]
fn embedded_uri_01() {
    let src = String::from(
        r#"
`embedded uri <https://docs.rs/regex/1.3.9/regex/>`_

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

    if let TreeNodeType::Reference {
        displayed_text,
        reference,
    } = doctree.shared_child(0).shared_child(0).shared_data()
    {
        if let Reference::URI(ref_str) = reference {
            assert_eq!(ref_str, "https://docs.rs/regex/1.3.9/regex/");
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

#[test]
fn substitution_ref_01() {
    let src = String::from(
        r#"
This is a simple |substitution reference|.  It will be replaced by
the processing system.

This is a combination |substitution and hyperlink reference|_.  In
addition to being replaced, the replacement text or element will
refer to the "substitution and hyperlink reference" target.
"#,
    );
    let mut lc = LineCursor::new(0, 0);

    let nodes = match Parser::inline_parse(src, None, &mut lc) {
        InlineParsingResult::Nodes(nodes) => nodes,
        _ => panic!("No nodes to be found!"),
    };

    eprintln!("{:#?}", nodes);

    if let TreeNodeType::SubstitutionReference {
        substitution_label,
        target_label,
    } = &nodes[9]
    {
        assert_eq!(substitution_label.as_str(), "substitution reference");
        assert_eq!(target_label.as_deref(), None);
    } else {
        panic!()
    };

    if let TreeNodeType::SubstitutionReference {
        substitution_label,
        target_label,
    } = &nodes[36]
    {
        assert_eq!(
            substitution_label.as_str(),
            "substitution and hyperlink reference"
        );
        assert_eq!(
            target_label.as_deref(),
            Some("substitution and hyperlink reference")
        );
    } else {
        panic!()
    };
}

#[test]
fn inline_parse_05() {
    let src = String::from("This is an absolute URI: https://john.harry.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top");
    let mut lc = LineCursor::new(0, 0);

    let nodes = match Parser::inline_parse(src, None, &mut lc) {
        InlineParsingResult::Nodes(nodes) => nodes,
        _ => panic!("No nodes to be found!"),
    };

    eprintln!("{:#?}", nodes);

    if let TreeNodeType::Reference {
        displayed_text,
        reference,
    } = &nodes[10]
    {
        if let Reference::URI(ref_str) = reference {
            assert_eq!(ref_str, "https://john.harry.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top")
        } else {
            panic!()
        }
    } else {
        panic!("Absolute URI not found!")
    }
}

#[test]
fn inline_parse_06() {
    let src = String::from("This is an email address: john.harry.doe@www.example.com");
    let mut lc = LineCursor::new(0, 0);

    let nodes = match Parser::inline_parse(src, None, &mut lc) {
        InlineParsingResult::Nodes(nodes) => nodes,
        _ => panic!("No nodes to be found!"),
    };

    eprintln!("{:#?}", nodes);

    if let TreeNodeType::Reference {
        displayed_text,
        reference,
    } = &nodes[10]
    {
        if let Reference::EMail(ref_str) = reference {
            assert_eq!(ref_str, "john.harry.doe@www.example.com")
        } else {
            panic!()
        }
    } else {
        panic!("Absolute URI not found!")
    }
}

use std::path::PathBuf;

#[test]
fn quoted_markup_01() {
    let src = String::from(
        r#"
Paragraph with quoted markup: "**strong emphasis**",
<*text in italics*>, (`a phrase reference with automatic labeling`__),
maybe a -simple-reference__- as well.

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

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(8).shared_data() {
        assert_eq!(text, "\"");
    } else {
        panic!()
    }

    if let TreeNodeType::StrongEmphasis { text } =
        doctree.shared_child(0).shared_child(9).shared_data()
    {
        assert_eq!(text, "strong emphasis");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(10).shared_data() {
        assert_eq!(text, "\",");
    } else {
        panic!()
    }

    if let TreeNodeType::WhiteSpace { text } =
        doctree.shared_child(0).shared_child(11).shared_data()
    {
        assert_eq!(text, "\n");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(12).shared_data() {
        assert_eq!(text, "<");
    } else {
        panic!()
    }

    if let TreeNodeType::Emphasis { text } = doctree.shared_child(0).shared_child(13).shared_data()
    {
        assert_eq!(text, "text in italics");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(14).shared_data() {
        assert_eq!(text, ">,");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(16).shared_data() {
        assert_eq!(text, "(");
    } else {
        panic!()
    }

    if let TreeNodeType::Reference {
        displayed_text,
        reference,
    } = doctree.shared_child(0).shared_child(17).shared_data()
    {
        if let Reference::Internal(ref_str) = reference {
            assert_eq!(
                displayed_text.as_ref(),
                Some(&"a phrase reference with automatic labeling".to_string())
            );
            assert_eq!(ref_str, "[[-ANON-LABEL-1-]]");
        } else {
            panic!()
        }
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(18).shared_data() {
        assert_eq!(text, "),");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(24).shared_data() {
        assert_eq!(text, "-");
    } else {
        panic!()
    }

    if let TreeNodeType::Reference {
        displayed_text,
        reference,
    } = doctree.shared_child(0).shared_child(25).shared_data()
    {
        assert_eq!(displayed_text, &None);
        if let Reference::Internal(ref_str) = reference {
            assert_eq!(ref_str, "[[-ANON-LABEL-2-]]");
        } else {
            panic!()
        }
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(26).shared_data() {
        assert_eq!(text, "-");
    } else {
        panic!()
    }
}

#[test]
fn quoted_markup_02() {
    let src = String::from(
        r#"
Test for "*"quoted* (**)start** '`'strings <https://www.absolute.uri.fi>`__.

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

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(4).shared_data() {
        assert_eq!(text, "\"*\"quoted*");
    } else {
        panic!()
    }

    if let TreeNodeType::WhiteSpace { text } = doctree.shared_child(0).shared_child(5).shared_data()
    {
        assert_eq!(text, " ");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(6).shared_data() {
        assert_eq!(text, "(**)start**");
    } else {
        panic!()
    }

    if let TreeNodeType::WhiteSpace { text } = doctree.shared_child(0).shared_child(7).shared_data()
    {
        assert_eq!(text, " ");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(8).shared_data() {
        assert_eq!(text, "\'`\'strings");
    } else {
        panic!()
    }

    if let TreeNodeType::WhiteSpace { text } = doctree.shared_child(0).shared_child(9).shared_data()
    {
        assert_eq!(text, " "); // <- Plain text is LaTeX-escaped
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(10).shared_data() {
        assert_eq!(text, "<"); // <- Plain text is LaTeX-escaped
    } else {
        panic!()
    }

    if let TreeNodeType::Reference {
        displayed_text,
        reference,
    } = doctree.shared_child(0).shared_child(11).shared_data()
    {
        if let Reference::URI(ref_str) = reference {
            assert_eq!(ref_str, "https://www.absolute.uri.fi"); // <- Plain text is LaTeX-escaped
        } else {
            panic!()
        }
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(12).shared_data() {
        assert_eq!(text, ">`\\_\\_."); // <- Plain text is LaTeX-escaped
    } else {
        panic!()
    }
}

#[test]
fn quoted_markup_03() {
    let src = String::from(
        r#"
Test for ats: @``@literal``.

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

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(6).shared_data() {
        assert_eq!(text, "@");
    } else {
        panic!()
    }

    if let TreeNodeType::Literal { text } = doctree.shared_child(0).shared_child(7).shared_data() {
        assert_eq!(text, "@literal");
    } else {
        panic!()
    }
}

#[test]
fn interpreted_text_01() {
    let src = String::from(
        r#"
Test for :emphasis:`interpreted text`.

Here is some math with a suffix role: `α_t(i) = P(O_1, O_2, … O_t, q_t = S_i λ)`:math:.

Let's add a title reference too: :title-reference:`Söderholm2020`.

The following should produce a title reference: `Söderholm2020`.

`Strong emphasis`:strong:.

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

    if let TreeNodeType::Emphasis { text } = doctree.shared_child(0).shared_child(4).shared_data() {
        assert_eq!(text, "interpreted text");
    } else {
        panic!()
    }

    if let TreeNodeType::Math { text, name, class } =
        doctree.shared_child(1).shared_child(16).shared_data()
    {
        assert_eq!(
            text,
            r"\alpha_t(i) = P(O_1, O_2, \ldots O_t, q_t = S_i \lambda)"
        );
    } else {
        panic!()
    }

    if let TreeNodeType::TitleReference {
        displayed_text,
        target_label,
    } = doctree.shared_child(2).shared_child(12).shared_data()
    {
        assert_eq!(displayed_text, "Söderholm2020");
        assert_eq!(target_label, "söderholm2020");
    } else {
        panic!()
    }

    if let TreeNodeType::TitleReference {
        displayed_text,
        target_label,
    } = doctree.shared_child(3).shared_child(14).shared_data()
    {
        assert_eq!(displayed_text, "Söderholm2020");
        assert_eq!(target_label, "söderholm2020");
    } else {
        panic!()
    }

    if let TreeNodeType::StrongEmphasis { text } =
        doctree.shared_child(4).shared_child(0).shared_data()
    {
        assert_eq!(text, "Strong emphasis");
    } else {
        panic!()
    }
}

#[test]
fn interpreted_text_02() {
    let src = String::from(
        r#"
Interpreted text with two roles produces a literal: :emphasis:`text here`:strong:.

Unknown roles also trigger literal text: :role1:`this is a literal`,
`as is this`:role2:.

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

    if let TreeNodeType::Literal { text } = doctree.shared_child(0).shared_child(16).shared_data() {
        assert_eq!(text, ":emphasis:`text here`:strong:");
    } else {
        panic!()
    }

    if let TreeNodeType::Literal { text } = doctree.shared_child(1).shared_child(12).shared_data() {
        assert_eq!(text, ":role1:`this is a literal`");
    } else {
        panic!()
    }

    if let TreeNodeType::Literal { text } = doctree.shared_child(1).shared_child(15).shared_data() {
        assert_eq!(text, "`as is this`:role2:");
    } else {
        panic!()
    }
}

#[test]
fn quoted_interpreted_text_01() {
    let src = String::from(
        r#"
(`a "quoted" title reference`)

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

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(0).shared_data() {
        assert_eq!(text, "(");
    } else {
        panic!()
    }

    if let TreeNodeType::TitleReference {
        displayed_text,
        target_label,
    } = doctree.shared_child(0).shared_child(1).shared_data()
    {
        assert_eq!(displayed_text, r#"a "quoted" title reference"#);
        assert_eq!(target_label, r#"a "quoted" title reference"#);
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(2).shared_data() {
        assert_eq!(text, ")");
    } else {
        panic!()
    }
}

#[test]
fn quoted_interpreted_text_02() {
    let src = String::from(
        r#"
":emphasis:`quoted emphasis`"

<`quoted strong emphasis`:strong:>

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

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(0).shared_data() {
        assert_eq!(text, "\"");
    } else {
        panic!()
    }

    if let TreeNodeType::Emphasis { text } = doctree.shared_child(0).shared_child(1).shared_data() {
        assert_eq!(text, r#"quoted emphasis"#);
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(2).shared_data() {
        assert_eq!(text, "\"");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(1).shared_child(0).shared_data() {
        assert_eq!(text, "<");
    } else {
        panic!()
    }

    if let TreeNodeType::StrongEmphasis { text } =
        doctree.shared_child(1).shared_child(1).shared_data()
    {
        assert_eq!(text, r#"quoted strong emphasis"#);
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(1).shared_child(2).shared_data() {
        assert_eq!(text, ">");
    } else {
        panic!()
    }
}

#[test]
fn quoted_interpreted_text_03() {
    let src = String::from(
        r#"
":"emphasis:`quoted emphasis`"

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

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(0).shared_data() {
        assert_eq!(text, "\":\"");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(1).shared_data() {
        assert_eq!(text, "emphasis:`quoted");
    } else {
        panic!()
    }

    if let TreeNodeType::WhiteSpace { text } = doctree.shared_child(0).shared_child(2).shared_data()
    {
        assert_eq!(text, " ");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(3).shared_data() {
        assert_eq!(text, "emphasis`\"");
    } else {
        panic!()
    }
}

#[test]
fn uri_01() {
    let src = String::from(
        r#"
<https://quoted.uri.fi>.

<quoted@email.com>

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

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(0).shared_data() {
        assert_eq!(text, "<");
    } else {
        panic!()
    }

    if let TreeNodeType::Reference { reference, .. } =
        doctree.shared_child(0).shared_child(1).shared_data()
    {
        if let Reference::URI(ref_str) = reference {
            assert_eq!(ref_str, "https://quoted.uri.fi");
        } else {
            panic!()
        }
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(0).shared_child(2).shared_data() {
        assert_eq!(text, ">.");
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(1).shared_child(0).shared_data() {
        assert_eq!(text, "<");
    } else {
        panic!()
    }

    if let TreeNodeType::Reference { reference, .. } =
        doctree.shared_child(1).shared_child(1).shared_data()
    {
        if let Reference::EMail(ref_str) = reference {
            assert_eq!(ref_str, "quoted@email.com");
        } else {
            panic!()
        }
    } else {
        panic!()
    }

    if let TreeNodeType::Text { text } = doctree.shared_child(1).shared_child(2).shared_data() {
        assert_eq!(text, ">");
    } else {
        panic!()
    }
}

#[test]
fn inline_target_01() {
    let src = String::from(
        r#"
_`inline target` **This emphasis is a reference target**.
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

    use crate::doctree::tree_node::TreeNode;

    if let TreeNodeType::StrongEmphasis { text } =
        doctree.shared_child(0).shared_child(1).shared_data()
    {
        if let Some(names) = doctree
            .shared_child(0)
            .shared_child(1)
            .shared_target_labels()
        {
            assert_eq!(names, &["inline target"]);
        };
    } else {
        panic!()
    }
}

#[test]
fn footnote_ref_01 () {
    let src = String::from(
        r#"[1]_ [#]_ [#Label]_ [*]_"#,
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::FootnoteReference { displayed_text, target_label } = doctree
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        assert_eq!(displayed_text, "1");
        assert_eq!(target_label, "1");
    } else {
        panic!()
    }
    if let TreeNodeType::FootnoteReference { displayed_text, target_label } = doctree
        .shared_child(0)
        .shared_child(2)
        .shared_data()
    {
        assert_eq!(displayed_text, "2");
        assert_eq!(target_label, "2");
    } else {
        panic!()
    }
    if let TreeNodeType::FootnoteReference { displayed_text, target_label } = doctree
        .shared_child(0)
        .shared_child(4)
        .shared_data()
    {
        assert_eq!(displayed_text, "Label");
        assert_eq!(target_label, "3");
    } else {
        panic!()
    }
    if let TreeNodeType::FootnoteReference { displayed_text, target_label } = doctree
        .shared_child(0)
        .shared_child(6)
        .shared_data()
    {
        assert_eq!(displayed_text, "*");
        assert_eq!(target_label, "*");
    } else {
        panic!()
    }
    todo!()
}