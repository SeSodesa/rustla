/// ## inline
/// A submodule related to parsing blocks of text for inline elements.
/// 
/// ### Inline markup recognition rules
/// 
/// Inline markup start-strings and end-strings are only recognized if the following conditions are met:
/// 
/// 1. Inline markup start-strings must be immediately followed by non-whitespace.
/// 2. Inline markup end-strings must be immediately preceded by non-whitespace.
/// 3. The inline markup end-string must be separated by at least one character from the start-string.
/// 4. Both, inline markup start-string and end-string must not be preceded by an unescaped backslash
///    (except for the end-string of inline literals). See Escaping Mechanism above for details.
/// 5. If an inline markup start-string is immediately preceded by one of the ASCII characters ' " < ( [ { or a similar non-ASCII character,
///    it must not be followed by the corresponding closing character from ' " ) ] } > or a similar non-ASCII character.
///    (For quotes, matching characters can be any of the quotation marks in international usage.)
/// 
/// If the configuration setting simple-inline-markup is False (default),
/// additional conditions apply to the characters "around" the inline markup:
/// 
/// 6. Inline markup start-strings must start a text block or be immediately preceded by
///   * whitespace,
///   * one of the ASCII characters - : / ' " < ( [ {
///   * or a similar non-ASCII punctuation character.
/// 
/// 7. Inline markup end-strings must end a text block or be immediately followed by
///   * whitespace,
///   * one of the ASCII characters - . , : ; ! ? \ / ' " ) ] } >
///   * or a similar non-ASCII punctuation character.
///
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### paired_delimiter
/// Parses inline text elements that have simple opening
/// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
pub fn paired_delimiter (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {
  
  // Destructuring the regex parts...

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, lookahead_str) {

    eprintln!("{}...{}\n", lookbehind_str, lookahead_str);

    let start_quote_string = lookbehind_str.to_string();
    let match_len = start_quote_string.chars().count();
    let text_node = TreeNodeType::Text { text: start_quote_string };
    return (text_node, match_len)

  } else if quotation_matches(lookbehind_str, content) {

    eprintln!("Quoted start!\n");

    let lookbehind_string = lookbehind_str.to_string();
    let lookbehind_char_count = lookbehind_string.chars().count();

    let quoted_start_char_count = (lookbehind_str.to_string() + markup_start_str).chars().count() + 1;

    let quoted_start_string: String = lookbehind_string
      + markup_start_str
      + content
        .chars()
        .take(lookbehind_char_count)
        .collect::<String>()
        .as_str();

    eprintln!("quoted start: {:#?}\n", quoted_start_string);
    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)
  }

  let content_string = String::from(content);

  let node_data = match pattern_name {
    PatternName::StrongEmphasis => TreeNodeType::StrongEmphasis{text: content_string},
    PatternName::Emphasis => TreeNodeType::Emphasis{text: content_string},
    PatternName::Literal => TreeNodeType::Literal{text: content_string},
    PatternName::InlineTarget => TreeNodeType::InlineTarget{target_label: content_string},
    _ => panic!("No such paired delimiter type!")
  };

  let match_len = (lookbehind_str.to_string() + markup_start_str + content + markup_end_str).chars().count();

  (node_data, match_len)
}


/// ### whitespace
/// Parses inline whitespace
pub fn whitespace(opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let content = captures.get(0).unwrap();
  let node_data = TreeNodeType::WhiteSpace{text: String::from(content.as_str())};
  let match_len = content.as_str().chars().count();

  (node_data, match_len)
}


pub fn interpreted_text (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  todo!()
}

/// ### reference
/// Parses reference type inline elements based on their pattern name.
pub fn reference(opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, lookahead_str) {

    // The entire markup is quoted so turn beginning quote into text and return

    let lookbehind_as_text = lookbehind_str.to_string();
    let match_len = lookbehind_as_text.chars().count();
    let text_node = TreeNodeType::Text { text: lookbehind_as_text.to_string() };
    return (text_node, match_len)

  } else if quotation_matches(lookbehind_str, content) {

    let quoted_start_char_count = lookbehind_str.chars().count() + markup_start_str.chars().count() + 1;

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect();
    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)
  }

  let whole_match = captures.get(0).unwrap();
  let displayed_text = captures.name("content").unwrap().as_str();

  let target_label = if let Some(type_match) = captures.name("ref_type") {
    match type_match.as_str() {
      "_"   => displayed_text.to_string(),
      "__"  => {
        if let Some(doctree_ref) = opt_doctree_ref {
          doctree_ref.next_anon_reference_label()
        } else {
          panic!("No doctree reference where one was expected while parsing an inline reference...\n")
        }
      },
      _ => panic!("No matching reference type when parsing an inline reference...\n")
    }
  } else {
    panic!("No reference type suffix (\"_\" or \"__\") when parsing an inline reference...\n")
  };

  let data = match pattern_name {
    PatternName::SimpleRef | PatternName::PhraseRef => {

      TreeNodeType::Reference{
        displayed_text: displayed_text.to_string(),
        target_label: target_label
      }
    },
    PatternName::FootNoteRef => {
      TreeNodeType::FootnoteReference{
        displayed_text: displayed_text.to_string(),
        target_label: target_label
      }
    },
    // PatternName::SubstitutionRef => {
    //   TreeNodeType::SubstitutionReference{
    //     displayed_text: displayed_text.to_string(),
    //     target_label: target_label
    //   }
    // },
    _ => panic!("No such reference pattern.\n")
  };

  let match_len = captures.get(0).unwrap().as_str().chars().count();

  (data, match_len)
}


pub fn simple_ref (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let content = captures.name("content").unwrap().as_str();
  let ref_type = captures.name("ref_type").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, lookahead_str) {

    let start_quote_string = lookbehind_str.to_string();
    let match_len = start_quote_string.chars().count();
    let text_node = TreeNodeType::Text { text: start_quote_string };
    return (text_node, match_len)
  }

  use crate::common::normalize_refname;

  let target_label: String = match ref_type {
    "__" => { // Automatic reference label => ask doctree for label, if present. Else use the manual label

      if let Some(doctree) = opt_doctree_ref {
        doctree.next_anon_reference_label()
      } else {
        eprintln!("Warning: detected an automatic reference name but no doctree available to generate one...");
        normalize_refname(content)
      }
    }
    "_" => { // Manual reference label
      normalize_refname(content)
    }
    _ => unreachable!("Only automatic or manual reference types are recognized. Computer says no...")
  };

  let ref_node = TreeNodeType::Reference {
    displayed_text: content.to_string(),
    target_label: target_label
  };

  let match_len = (lookbehind_str.to_string() + content + ref_type).chars().count();
  
  (ref_node, match_len)
}

pub fn phrase_ref (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let ref_type = captures.name("ref_type").unwrap().as_str();
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, lookahead_str) {

    // The entire markup is quoted so turn beginning quote into text and return

    let lookbehind_as_text = lookbehind_str.to_string();
    let match_len = lookbehind_as_text.chars().count();
    let text_node = TreeNodeType::Text { text: lookbehind_as_text.to_string() };
    return (text_node, match_len)

  } else if quotation_matches(lookbehind_str, content) {

    let quoted_start_char_count = lookbehind_str.chars().count() + markup_start_str.chars().count() + 1;

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect();
    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)
  }

  use crate::common::normalize_refname;

  let target_label: String = match ref_type {
    "__" => { // Automatic reference label => ask doctree for label, if present. Else use the manual label

      if let Some(doctree) = opt_doctree_ref {
        doctree.next_anon_reference_label()
      } else {
        eprintln!("Warning: detected an automatic reference name but no doctree available to generate one...");
        normalize_refname(content)
      }
    }
    "_" => { // Manual reference label
      normalize_refname(content)
    }
    _ => unreachable!("Only automatic or manual reference types are recognized. Computer says no...")
  };

  let ref_node = TreeNodeType::Reference {
    displayed_text: content.to_string(),
    target_label: target_label
  };

  let match_len = (lookbehind_str.to_string() + markup_start_str + content + markup_end_str + ref_type).chars().count();

  (ref_node, match_len)
}


pub fn footnote_ref (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let ref_type = captures.name("ref_type").unwrap().as_str();
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, lookahead_str) {

    // The entire markup is quoted so turn beginning quote into text and return

    let lookbehind_as_text = lookbehind_str.to_string();
    let match_len = lookbehind_as_text.chars().count();
    let text_node = TreeNodeType::Text { text: lookbehind_as_text.to_string() };
    return (text_node, match_len)

  } else if quotation_matches(lookbehind_str, content) {

    let quoted_start_char_count = lookbehind_str.chars().count() + markup_start_str.chars().count() + 1;

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect();
    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)
  }

  todo!()
}


pub fn citation_ref (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let ref_type = captures.name("ref_type").unwrap().as_str();
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, lookahead_str) {

    // The entire markup is quoted so turn beginning quote into text and return

    let lookbehind_as_text = lookbehind_str.to_string();
    let match_len = lookbehind_as_text.chars().count();
    let text_node = TreeNodeType::Text { text: lookbehind_as_text.to_string() };
    return (text_node, match_len)

  } else if quotation_matches(lookbehind_str, content) {

    let quoted_start_char_count = lookbehind_str.chars().count() + markup_start_str.chars().count() + 1;

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect();
    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)
  }

  todo!()
}


pub fn substitution_ref (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let ref_type = if let Some(ref_type_str) = captures.name("ref_type") { ref_type_str.as_str() } else { "" };
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, lookahead_str) {

    // The entire markup is quoted so turn beginning quote into text and return

    let lookbehind_as_text = lookbehind_str.to_string();
    let match_len = lookbehind_as_text.chars().count();
    let text_node = TreeNodeType::Text { text: lookbehind_as_text.to_string() };
    return (text_node, match_len)

  } else if quotation_matches(lookbehind_str, content) {

    let quoted_start_char_count = lookbehind_str.chars().count() + markup_start_str.chars().count() + 1;

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect();
    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)
  }

  use crate::common::normalize_refname;

  let target_label = if !ref_type.is_empty() {

    let target_label: String = match ref_type {
      "__" => { // Automatic reference label => ask doctree for label, if present. Else use the manual label
  
        if let Some(doctree) = opt_doctree_ref {
          doctree.next_anon_reference_label()
        } else {
          eprintln!("Warning: detected an automatic reference name but no doctree available to generate one...");
          normalize_refname(content)
        }
      }
      "_" => { // Manual reference label
        normalize_refname(content)
      }
      _ => unreachable!("Only automatic or manual reference types are recognized. Computer says no...")
    };

    Some(target_label)

  } else { None };

  let substitution_ref_node = TreeNodeType::SubstitutionReference {
    substitution_label: normalize_refname(content),
    target_label: target_label
  };

  let match_len = (lookbehind_str.to_string() + markup_start_str + content + markup_end_str + ref_type).chars().count();

  (substitution_ref_node, match_len)
}


pub fn uri (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let whole_match = captures.get(0).unwrap();

  let mut is_valid = true;

  const MISSING: &str = "!!!MISSING!!!";

  // Retrieving the relevant parts of the URI as &str
  let scheme = if let Some(scheme) = captures.name("scheme") {
    scheme.as_str()
  } else {
    MISSING
  };

  eprintln!("Scheme: {:#?}", scheme);

  let data = match scheme {
    MISSING => {
      let email = if let Some(email) = captures.name("email") {
        email.as_str()
      } else {
        MISSING
      };

      eprintln!("Email: {:#?}", email);

      // If no email when missing a scheme, simply return match as string
      if email == MISSING {
        let match_str = whole_match.as_str();
        let data = TreeNodeType::Text{text: String::from(whole_match.as_str())};
        return (data, match_str.chars().count())
      }

      let match_str = whole_match.as_str();

      // If a successful email recognition, prepend a mailto scheme to email.
      TreeNodeType::StandaloneEmail{text: format!("{}", match_str)}

    }

    _ => {

      let authority = if let Some(authority) = captures.name("authority") {
        authority.as_str()
      } else {
        MISSING
      };
      let userinfo = if let Some(userinfo) = captures.name("userinfo") {
        userinfo.as_str()
      } else {
        MISSING
      };
      let host = if let Some(host) = captures.name("host") {
        host.as_str()
      } else {
        MISSING
      };
      let port = if let Some(port) = captures.name("port") {
        port.as_str()
      } else {
        MISSING
      };

      eprintln!("Authority: {:#?}", authority);
      eprintln!("  userinfo: {:#?}", userinfo);
      eprintln!("  host: {:#?}", host);
      eprintln!("  port: {:#?}", port);

      let path = if let Some(path) = captures.name("path")  {
        path.as_str()
      } else {
        MISSING
      };

      eprintln!("path: {:#?}", path);

      let query = if let Some(query) = captures.name("query") {
        query.as_str()
      } else {
        MISSING
      };

      eprintln!("query: {:#?}", query);

      let fragment = if let Some(fragment) = captures.name("fragment") {
        fragment.as_str()
      } else {
        MISSING
      };

      eprintln!("fragment: {:#?}", fragment);

      // Validity checks

      if authority != MISSING  {
        let has_slash = if let Some(c) = path.chars().next() {
          eprintln!("First char of path is {}\n", c);

          let mut has_slash: bool = false;
          if c == '/' {
            has_slash = true;
          }
          has_slash

        } else {
          false
        };

        if !has_slash {
          eprintln!("URI {}\nhas an autority field and a path that doesn't start with a '/'...\n  => URI invalid\n", whole_match.as_str());
          is_valid = false;
        }
      }

      // If URI is valid, return it as URI, else as text
      if is_valid {
        TreeNodeType::AbsoluteURI{text: String::from(whole_match.as_str())}
      } else {
        TreeNodeType::Text{text: String::from(whole_match.as_str())}
      }

    }
  };

  let match_len = whole_match.as_str().chars().count();
  (data, match_len)
}


/// ### text
/// Parses inline text elements that have simple opening
/// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
pub fn text (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let content = captures.get(1).unwrap();
  let match_len = content.as_str().chars().count();
  let node_data = TreeNodeType::Text { text: String::from(content.as_str()) };
  (node_data, match_len)
}


// =======================
//  Constants and helpers
// =======================

/// ### quotation matches
/// 
/// Checks the two given string slices for matching reStructuredText quotation characters.
fn quotation_matches (start: &str, end: &str) -> bool {

  for pair in QUOTATION_PAIRS.iter() {
    if start.ends_with(pair.0) && end.starts_with(pair.1) { return true }
  };

  false
}


/// ### QUOTATION_PAIRS
/// 
/// A listing of pairs of "quotation" characters that, among other things,
/// must not immediately surround reStructuredText inline markup for it
/// to be interpreted as such. Contains all known international paris of
/// quotation strings.
/// 
/// #### Note
/// 
/// Some languages like French use multiple characters (quillemets) to denote quotations.
/// Hence the type of the pairs.
/// 
/// #### TODO
/// 
/// Reduce the size of this list or alternatively come up with a string- and iterator-based approach.
/// 
/// source: https://en.wikipedia.org/wiki/Quotation_mark#Summary_table
/// 
const QUOTATION_PAIRS: &[(&str, &str)] = &[
  
  // Parentheses and other "quotation" pairs
  (r#"("#, r#")"#),
  (r#"["#, r#"]"#),
  (r#"{"#, r#"}"#),
  (r#"<"#, r#">"#),
  (r#"-"#, r#"-"#),
  
  // Language-specific quotation pairs
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"”"#),
  (r#"‚"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»‎"#),
  (r#"”"#, r#"“‎"#),
  (r#"«"#, r#"»"#),
  (r#"«"#, r#"»"#),
  (r#"„"#, r#"“"#),
  (r#"„"#, r#"“"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"”"#, r#"”"#),
  (r#"„"#, r#"”"#),
  (r#"’"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"»"#, r#"«"#),
  (r#"”"#, r#"”"#),
  (r#"’"#, r#"’"#),
  (r#"»"#, r#"«"#),
  (r#"„"#, r#"“"#),
  (r#"’"#, r#"’"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"’"#, r#"’"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"""#, r#"""#),
  (r#"“"#, r#"”"#),
  (r#"’"#, r#"’"#),
  (r#"‘"#, r#"’"#),
  (r#"'"#, r#"'"#),
  (r#"‘"#, r#"’"#),
  (r#"‛"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"『"#, r#"』"#),
  (r#"「"#, r#"」"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"「"#, r#"」"#),
  (r#"『"#, r#"』"#),
  (r#"「"#, r#"」"#),
  (r#"『"#, r#"』"#),
  (r#"„"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"»"#, r#"«"#),
  (r#"„"#, r#"”"#),
  (r#"»"#, r#"«"#),
  (r#"‘"#, r#"’"#),
  (r#"»"#, r#"«"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"»"#, r#"«"#),
  (r#"›"#, r#"‹"#),
  (r#"»"#, r#"«"#),
  (r#"„"#, r#"“"#),
  (r#"›"#, r#"‹"#),
  (r#"‚"#, r#"‘"#),
  (r#"”"#, r#"”"#),
  (r#"’"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"”"#),
  (r#","#, r#"’"#),
  (r#"‘"#, r#"’"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"‹"#, r#"›"#),
  (r#"‚"#, r#"‘"#),
  (r#"„"#, r#"“"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"”"#, r#"”"#),
  (r#"’"#, r#"’"#),
  (r#"»"#, r#"»"#),
  (r#"’"#, r#"’"#),
  (r#"« "#, r#" »"#),
  (r#"« "#, r#" »"#),
  (r#"‹ "#, r#" ›"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"“"#, r#"”"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"»"#, r#"«"#),
  (r#"›"#, r#"‹"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"”"#, r#"”‎"#),
  (r#"’"#, r#"’‎"#),
  (r#"„"#, r#"”‎"#),
  (r#"‚"#, r#"’‎"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"”"#),
  (r#"»"#, r#"«"#),
  (r#"„"#, r#"”"#),
  (r#"»"#, r#"«"#),
  (r#"’"#, r#"’"#),
  (r#"""#, r#"""#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"« "#, r#" »"#),
  (r#"‘ "#, r#" ’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"「"#, r#"」"#),
  (r#"『"#, r#"』"#),
  (r#"「"#, r#"」"#),
  (r#"『"#, r#"』"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"《"#, r#"》"#),
  (r#"〈"#, r#"〉"#),
  (r#"〈"#, r#"〉"#),
  (r#"《"#, r#"》"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"「"#, r#"」"#),
  (r#"『"#, r#"』"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"„"#, r#"”"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"“"#, r#"”"#),
  (r#"„"#, r#"“"#),
  (r#"’"#, r#"‘"#),
  (r#"„"#, r#"“"#),
  (r#"’"#, r#"‘"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"„"#, r#"“"#),
  (r#"„"#, r#"“"#),
  (r#"《"#, r#"》"#),
  (r#"〈"#, r#"〉"#),
  (r#"《"#, r#"》"#),
  (r#"〈"#, r#"〉"#),
  (r#"«"#, r#"»"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#","#, r#"‘"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»‎"#),
  (r#"«"#, r#"»‎"#),
  (r#"„"#, r#"”"#),
  (r#"«"#, r#"»"#),
  (r#"»"#, r#"«"#),
  (r#"«"#, r#"»"#),
  (r#"»"#, r#"«"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"«"#, r#"»"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"”"#),
  (r#"«"#, r#"»"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"«"#, r#"»"#),
  (r#"„"#, r#"“"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"„"#, r#"“"#),
  (r#"„"#, r#"”"#),
  (r#"’"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"«"#, r#"»"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"»"#, r#"«"#),
  (r#"›"#, r#"‹"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"»"#, r#"«"#),
  (r#"›"#, r#"‹"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"”"#, r#"”"#),
  (r#"’"#, r#"’"#),
  (r#"»"#, r#"»"#),
  (r#"»"#, r#"«"#),
  (r#"’"#, r#"’"#),
  (r#"《"#, r#"》"#),
  (r#"〈"#, r#"〉"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"《"#, r#"》"#),
  (r#"〈"#, r#"〉"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»"#),
  (r#"‹"#, r#"›"#),
  (r#"«"#, r#"»"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"„"#, r#"“"#),
  (r#"„"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
  (r#"«"#, r#"»‎"#),
  (r#"‹"#, r#"›‎"#),
  (r#"«"#, r#"»"#),
  (r#"„"#, r#"“"#),
  (r#"„"#, r#"“"#),
  (r#"‚"#, r#"‘"#),
  (r#"“"#, r#"”"#),
  (r#"« "#, r#" »"#),
  (r#"‘"#, r#"’"#),
  (r#"“"#, r#"”"#),
  (r#"“"#, r#"”"#),
  (r#"‘"#, r#"’"#),
];
