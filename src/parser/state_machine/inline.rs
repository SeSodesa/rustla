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
/// 
/// Parses inline text elements that have identical opening
/// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
pub fn paired_delimiter (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {
  
  // Destructuring the regex parts...

  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if quotation_matches(lookbehind_str, content) {

    let quoted_start_char_count = 2 * lookbehind_str.chars().count() + markup_start_str.chars().count();

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect::<String>();

    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)

  } else if quotation_matches(lookbehind_str, lookahead_str) {

    let start_quote_string = lookbehind_str.to_string();
    let match_len = start_quote_string.chars().count();
    let text_node = TreeNodeType::Text { text: start_quote_string };
    return (text_node, match_len)
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
/// 
/// Parses inline whitespace
pub fn whitespace(opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let content = captures.get(0).unwrap();
  let node_data = TreeNodeType::WhiteSpace{text: String::from(content.as_str())};
  let match_len = content.as_str().chars().count();

  (node_data, match_len)
}


pub fn interpreted_text (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let whole_match = captures.get(0).unwrap().as_str();
  let lookbehind_str = if let Some(lookbehind) = captures.name("lookbehind") { lookbehind.as_str() } else { "" };
  let front_role_marker = if let Some(marker) = captures.name("front_role_marker") { marker.as_str() } else { "" };
  let front_role = if let Some(role) = captures.name("front_role") { role.as_str() } else { "" };
  let markup_start_str = captures.name("markup_start").unwrap().as_str();
  let content = captures.name("content").unwrap().as_str();
  let markup_end_str = captures.name("markup_end").unwrap().as_str();
  let back_role_marker = if let Some(marker) = captures.name("back_role_marker") { marker.as_str() } else { "" };
  let back_role = if let Some(role) = captures.name("back_role") { role.as_str() } else { "" };
  let lookahead_str = if let Some(lookahead) = captures.name("lookahead") { lookahead.as_str() } else { "" };

  if !front_role_marker.is_empty() && quotation_matches(lookbehind_str, front_role) {

    let lookbehind_char_count = lookbehind_str.chars().count();
    let quoted_start_char_count = 2 * lookbehind_char_count + ":".chars().count();

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect();
    return (TreeNodeType::Text { text: quoted_start_string }, quoted_start_char_count)

  } else if quotation_matches(lookbehind_str, content) {

    let lookbehind_char_count = lookbehind_str.chars().count();
    let quoted_start_char_count = 2 * lookbehind_char_count + markup_start_str.chars().count();

    let quoted_start_string: String = captures
      .get(0)
      .unwrap()
      .as_str()
      .chars()
      .take(quoted_start_char_count)
      .collect();

    return (TreeNodeType::Text { text: quoted_start_string}, quoted_start_char_count)

  } else if quotation_matches(lookbehind_str, lookahead_str) {

    // The entire markup is quoted so turn beginning quote into text and return

    let lookbehind_as_text = lookbehind_str.to_string();
    let match_len = lookbehind_as_text.chars().count();
    let text_node = TreeNodeType::Text { text: lookbehind_as_text.to_string() };
    return (text_node, match_len)
  }

  if !front_role_marker.is_empty() && !back_role_marker.is_empty() {
    println!("Warning: found both pre- and suffix roles for interpreted text. Returning whole match as inline literal...");
    let match_len = (lookbehind_str.to_string() + front_role_marker + markup_start_str + content + markup_end_str + back_role_marker).chars().count();
    let match_string: String = whole_match
    .chars()
    .take(match_len)
    .collect();
    return (TreeNodeType::Literal { text: match_string }, match_len)
  }

  let match_len = (lookbehind_str.to_string() + front_role_marker + markup_start_str + content + markup_end_str + back_role_marker).chars().count();
  let role = if !front_role.is_empty() {
    front_role
  } else if !back_role.is_empty() {
    back_role
  } else {

    /// ### DEFAULT_DEFAULT_ROLE
    /// 
    /// This is used as the interpreted text role, if no role was specified.
    /// This is in accordance with the
    /// [reStructuredText Markup Specification](https://docutils.sourceforge.io/docs/ref/rst/roles.html).
    const DEFAULT_DEFAULT_ROLE: &str = "title-reference";

    println!("Warning: no role found for interpreted text. Using {}...", DEFAULT_DEFAULT_ROLE);
    use crate::common::normalize_refname;
    return (TreeNodeType::TitleReference { displayed_text: content.to_string(), target_label: normalize_refname(content) }, match_len)
  };

  match role {
    "emphasis" => {
      (TreeNodeType::Emphasis { text: content.to_string() }, match_len)
    }
    "literal" => {
      (TreeNodeType::Literal { text: content.to_string() }, match_len)
    }
    "code" => {
      (TreeNodeType::Literal { text: content.to_string() }, match_len)
    }
    "math" => {

      use utf8_to_latex::str_to_latex;
      // TODO: add conversions from utf8-characters such as greek letters
      //  to LaTeX macros to this, maybe via a "utf8_to_latex" function.
      let content_string = str_to_latex(content);
      (TreeNodeType::Math { text: content_string, class: None, name: None }, match_len)
    }
    "pep-reference" | "PEP" => {
      // PEP reference strings are 4 digits long
      let content_len = content.chars().count();
      let zeroes = "0".repeat(4 - content_len);
      let pep_ref = format!("https://www.python.org/peps/pep-{pep_num}.html", pep_num = zeroes + content);
      let displayed_text = "PEP ".to_string() + content;
      (TreeNodeType::Reference { displayed_text: displayed_text, target_label: pep_ref}, match_len)
    }
    "rfc-reference" | "RFC" => {
      let rfc_ref = format!("https://www.faqs.org/rfcs/rfc{rfc_num}.html", rfc_num = content);
      let displayed_text = "RFC ".to_string() + content;
      (TreeNodeType::Reference { displayed_text: displayed_text, target_label: rfc_ref }, match_len)
    }
    "strong" => {
      (TreeNodeType::StrongEmphasis { text: content.to_string() }, match_len)
    }
    "subscript" => {
      (TreeNodeType::Subscript { text: content.to_string() }, match_len)
    }
    "superscript" => {
      (TreeNodeType::Superscript { text: content.to_string() }, match_len)
    }
    "title-reference" => {
      use crate::common::normalize_refname;
      (TreeNodeType::TitleReference { displayed_text: content.to_string(), target_label: normalize_refname(content) }, match_len)
    }
    _ => { // Unknown role into literal
      let match_len = (lookbehind_str.to_string() + front_role_marker + markup_start_str + content + markup_end_str + back_role_marker).chars().count();
      let match_string: String = whole_match
      .chars()
      .take(match_len)
      .collect();
      return (TreeNodeType::Literal { text: match_string }, match_len)
    }
  }
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


/// ### simple_ref
/// 
/// Parses simple hyperlink references.
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


/// ### phrase_ref
/// 
/// Parses phrase references.
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


/// ### footnote_ref
/// 
/// Parses footnote references.
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


/// ### citation_ref
/// 
/// Parses citation references.
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


/// ### substitution_ref
/// 
/// Parses inline subsitution references. Also adds hyperlink information to the reference,
/// if the matched string ended with a `__?`.
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


/// ### uri
/// 
/// Parses inline URIs. These are split into general URIs and standalone email addresses.
/// These two are differentiate by whether the URI starts with a protocol scheme,
/// such as `https://`.
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
/// 
/// This function is invoked when no other inline pattern matched.
/// Eats up any consequent non-whitespace characters as a single
/// word of "text".
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

  for (i, c) in OPENERS.iter().enumerate() {
    if start.ends_with(*c) && end.starts_with(*CLOSERS.get(i).unwrap()) { return true }
  }

  for c in DELIMITERS.iter() {
    if start.ends_with(*c) && end.starts_with(*c) { return true }
  }

  false
}


/// ### OPENERS
/// 
/// A long string of "quotation openers".
/// 
/// source: https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils/docutils/utils/punctuation_chars.py#l46
const OPENERS: &[char] = &[
  '"', '\'', '(', '<', '\\', '[', '{', '\u{0f3a}', '\u{0f3c}', '\u{169b}', '\u{2045}', '\u{207d}', '\u{208d}', '\u{2329}', '\u{2768}',
  '\u{276a}', '\u{276c}', '\u{276e}', '\u{2770}', '\u{2772}', '\u{2774}', '\u{27c5}', '\u{27e6}', '\u{27e8}', '\u{27ea}',
  '\u{27ec}', '\u{27ee}', '\u{2983}', '\u{2985}', '\u{2987}', '\u{2989}', '\u{298b}', '\u{298d}', '\u{298f}', '\u{2991}',
  '\u{2993}', '\u{2995}', '\u{2997}', '\u{29d8}', '\u{29da}', '\u{29fc}', '\u{2e22}', '\u{2e24}', '\u{2e26}', '\u{2e28}',
  '\u{3008}', '\u{300a}', '\u{300c}', '\u{300e}', '\u{3010}', '\u{3014}', '\u{3016}', '\u{3018}', '\u{301a}', '\u{301d}',
  '\u{301d}', '\u{fd3e}', '\u{fe17}', '\u{fe35}', '\u{fe37}', '\u{fe39}', '\u{fe3b}', '\u{fe3d}', '\u{fe3f}', '\u{fe41}',
  '\u{fe43}', '\u{fe47}', '\u{fe59}', '\u{fe5b}', '\u{fe5d}', '\u{ff08}', '\u{ff3b}', '\u{ff5b}', '\u{ff5f}', '\u{ff62}',
  '\u{00ab}', '\u{2018}', '\u{201c}', '\u{2039}', '\u{2e02}', '\u{2e04}', '\u{2e09}', '\u{2e0c}', '\u{2e1c}', '\u{2e20}',
  '\u{201a}', '\u{201e}', '\u{00bb}', '\u{2019}', '\u{201d}', '\u{203a}', '\u{2e03}', '\u{2e05}', '\u{2e0a}', '\u{2e0d}',
  '\u{2e1d}', '\u{2e21}', '\u{201b}', '\u{201f}',
  
  // Additional (weird like the Swedish quotes that the Swedish don't even use) quotes

  '\u{00bb}', '\u{2018}', '\u{2019}', '\u{201a}', '\u{201a}',
  '\u{201c}', '\u{201e}', '\u{201e}', '\u{201d}', '\u{203a}',

];

/// ### CLOSERS
/// 
/// A long list of "quotation" closers.
/// 
/// source: https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils/docutils/utils/punctuation_chars.py#l56
const CLOSERS: &[char] = &[
  '"', '\'', ')', '>', '\\', ']', '}', '\u{0f3b}', '\u{0f3d}', '\u{169c}', '\u{2046}', '\u{207e}', '\u{208e}', '\u{232a}', '\u{2769}',
  '\u{276b}', '\u{276d}', '\u{276f}', '\u{2771}', '\u{2773}', '\u{2775}', '\u{27c6}', '\u{27e7}', '\u{27e9}', '\u{27eb}',
  '\u{27ed}', '\u{27ef}', '\u{2984}', '\u{2986}', '\u{2988}', '\u{298a}', '\u{298c}', '\u{298e}', '\u{2990}', '\u{2992}',
  '\u{2994}', '\u{2996}', '\u{2998}', '\u{29d9}', '\u{29db}', '\u{29fd}', '\u{2e23}', '\u{2e25}', '\u{2e27}', '\u{2e29}',
  '\u{3009}', '\u{300b}', '\u{300d}', '\u{300f}', '\u{3011}', '\u{3015}', '\u{3017}', '\u{3019}', '\u{301b}', '\u{301e}',
  '\u{301f}', '\u{fd3f}', '\u{fe18}', '\u{fe36}', '\u{fe38}', '\u{fe3a}', '\u{fe3c}', '\u{fe3e}', '\u{fe40}', '\u{fe42}',
  '\u{fe44}', '\u{fe48}', '\u{fe5a}', '\u{fe5c}', '\u{fe5e}', '\u{ff09}', '\u{ff3d}', '\u{ff5d}', '\u{ff60}', '\u{ff63}',
  '\u{00bb}', '\u{2019}', '\u{201d}', '\u{203a}', '\u{2e03}', '\u{2e05}', '\u{2e0a}', '\u{2e0d}', '\u{2e1d}', '\u{2e21}',
  '\u{201b}', '\u{201f}', '\u{00ab}', '\u{2018}', '\u{201c}', '\u{2039}', '\u{2e02}', '\u{2e04}', '\u{2e09}', '\u{2e0c}',
  '\u{2e1c}', '\u{2e20}', '\u{201a}', '\u{201e}',

  // Swedish, Albanian, etc. closers

  '\u{00bb}','\u{201a}','\u{2019}','\u{2018}','\u{2019}',
  '\u{201e}','\u{201c}','\u{201d}','\u{201d}','\u{203a}',
];


/// ### DELIMITERS
/// 
/// A long string of general delimiters in the unicode range smaller than `\u{FFFFFF}`.
/// Wider code points have been exluded because of Rust limitations on unicode digits,
/// for now. The docutils parser supports even those, so a solution might have to be invented.
/// 
/// source: https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils/docutils/utils/punctuation_chars.py#l66
const DELIMITERS: &[char] = &[
  '"', '\\', '-', '/', ':', '}', '\u{058a}', '\u{00a1}', '\u{00b7}', '\u{00bf}', '\u{037e}', '\u{0387}', '\u{055a}', '-', '\u{055f}', '\u{0589}',
  '\u{05be}', '\u{05c0}', '\u{05c3}', '\u{05c6}', '\u{05f3}', '\u{05f4}', '\u{0609}', '\u{060a}', '\u{060c}',
  '\u{060d}', '\u{061b}', '\u{061e}', '\u{061f}', '\u{066a}', '-', '\u{066d}', '\u{06d4}', '\u{0700}', '-', '\u{070d}',
  '\u{07f7}', '-', '\u{07f9}', '\u{0830}', '-', '\u{083e}', '\u{0964}', '\u{0965}', '\u{0970}', '\u{0df4}', '\u{0e4f}',
  '\u{0e5a}', '\u{0e5b}', '\u{0f04}', '-', '\u{0f12}', '\u{0f85}', '\u{0fd0}', '-', '\u{0fd4}', '\u{104a}', '-', '\u{104f}',
  '\u{10fb}', '\u{1361}', '-', '\u{1368}', '\u{1400}', '\u{166d}', '\u{166e}', '\u{16eb}', '-', '\u{16ed}', '\u{1735}',
  '\u{1736}', '\u{17d4}', '-', '\u{17d6}', '\u{17d8}', '-', '\u{17da}', '\u{1800}', '-', '\u{180a}', '\u{1944}', '\u{1945}',
  '\u{19de}', '\u{19df}', '\u{1a1e}', '\u{1a1f}', '\u{1aa0}', '-', '\u{1aa6}', '\u{1aa8}', '-', '\u{1aad}', '\u{1b5a}', '-',
  '\u{1b60}', '\u{1c3b}', '-', '\u{1c3f}', '\u{1c7e}', '\u{1c7f}', '\u{1cd3}', '\u{2010}', '-', '\u{2017}', '\u{2020}', '-',
  '\u{2027}', '\u{2030}', '-', '\u{2038}', '\u{203b}', '-', '\u{203e}', '\u{2041}', '-', '\u{2043}', '\u{2047}', '-',
  '\u{2051}', '\u{2053}', '\u{2055}', '-', '\u{205e}', '\u{2cf9}', '-', '\u{2cfc}', '\u{2cfe}', '\u{2cff}', '\u{2e00}',
  '\u{2e01}', '\u{2e06}', '-', '\u{2e08}', '\u{2e0b}', '\u{2e0e}', '-', '\u{2e1b}', '\u{2e1e}', '\u{2e1f}', '\u{2e2a}', '-',
  '\u{2e2e}', '\u{2e30}', '\u{2e31}', '\u{3001}', '-', '\u{3003}', '\u{301c}', '\u{3030}', '\u{303d}', '\u{30a0}',
  '\u{30fb}', '\u{a4fe}', '\u{a4ff}', '\u{a60d}', '-', '\u{a60f}', '\u{a673}', '\u{a67e}', '\u{a6f2}', '-', '\u{a6f7}',
  '\u{a874}', '-', '\u{a877}', '\u{a8ce}', '\u{a8cf}', '\u{a8f8}', '-', '\u{a8fa}', '\u{a92e}', '\u{a92f}', '\u{a95f}',
  '\u{a9c1}', '-', '\u{a9cd}', '\u{a9de}', '\u{a9df}', '\u{aa5c}', '-', '\u{aa5f}', '\u{aade}', '\u{aadf}', '\u{abeb}',
  '\u{fe10}', '-', '\u{fe16}', '\u{fe19}', '\u{fe30}', '-', '\u{fe32}', '\u{fe45}', '\u{fe46}', '\u{fe49}', '-', '\u{fe4c}',
  '\u{fe50}', '-', '\u{fe52}', '\u{fe54}', '-', '\u{fe58}', '\u{fe5f}', '-', '\u{fe61}', '\u{fe63}', '\u{fe68}', '\u{fe6a}',
  '\u{fe6b}', '\u{ff01}', '-', '\u{ff03}', '\u{ff05}', '-', '\u{ff07}', '\u{ff0a}', '\u{ff0c}', '-', '\u{ff0f}', '\u{ff1a}',
  '\u{ff1b}', '\u{ff1f}', '\u{ff20}', '\u{ff3c}', '\u{ff61}', '\u{ff64}', '\u{ff65}'
];
