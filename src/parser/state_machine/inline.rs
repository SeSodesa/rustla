/// ## inline
/// A submodule related to parsing blocks of text for inline elements.

use super::*;


/// ### paired_delimiter
/// Parses inline text elements that have simple opening
/// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
pub fn paired_delimiter (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {
  
  let content = captures.get(1).unwrap();

  let data = String::from(content.as_str());

  let node_data = match pattern_name {
    PatternName::StrongEmphasis => TreeNodeType::StrongEmphasis{text: data},
    PatternName::Emphasis => TreeNodeType::Emphasis{text: data},
    PatternName::Literal => TreeNodeType::Literal{text: data},
    PatternName::InlineTarget => TreeNodeType::InlineTarget{target_label: data},
    _ => panic!("No such paired delimiter type!")
  };

  let match_len = captures.get(0).unwrap().as_str().chars().count();

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


/// ### reference
/// Parses reference type inline elements based on their pattern name.
pub fn reference(opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize) {

  let whole_match = captures.get(0).unwrap();
  let displayed_text = captures.get(1).unwrap().as_str();
  let target_label = if  let Some(type_match) = captures.get(2) {
    match type_match.as_str() {
      "_"   => displayed_text.to_string(),
      "__"  => {
        if let Some(doctree_ref) = opt_doctree_ref {
          doctree_ref.next_anon_reference_label()
        } else {
          panic!("No doctree reference where one was expected while parsing an inline reference...\n")
        }
      },
      _     => panic!("No matching reference type when parsing an inline reference...\n")
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
    PatternName::SubstitutionRef => {
      TreeNodeType::SubstitutionReference{
        displayed_text: displayed_text.to_string(),
        target_label: target_label
      }
    },
    _ => panic!("No such reference pattern.\n")
  };

  let match_len = whole_match.as_str().chars().count();

  (data, match_len)
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
      TreeNodeType::StandaloneEmail{text: format!("{}{}", "mailto:", match_str)}

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
