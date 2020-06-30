/// This submodule contains the states and related transitions of
/// the parser state machine.

//pub mod body;

use super::*;
use crate::doctree::{self, TreeNode, TreeNodeType, structural_nodes, body_nodes, inline_nodes};

/// ### Body
/// A state for detecting and parsing the first lines
/// of different types of rST text blocks. Transitions to
/// other states for handling the following lines
/// of the block are handled by the `TransitionMethod`s
/// in this state.
pub struct Body  {
  pub transitions: &'static Vec<Transition>
}


impl Body  {

  /// ### new
  /// A `Body` state constructor.
  pub fn new() -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("Body").unwrap()
    }
  }

  /// ### bullet
  /// The transition method for matching bullets in `Body` state.
  /// Causes the parser to push a new machine in the state
  /// `BulletList` on top of its machine stack.
  pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop), &'static str> {

    let mut tree_container = doctree.unwrap();

    let bullet = captures.get(1).unwrap().as_str().chars().next().unwrap();
    let indent = captures.get(0).unwrap().end();
    let nesting_level: usize = 0;

    let bullet_list_data = TreeNodeType::BulletList(doctree::body_nodes::BulletList::new(bullet, indent));

    let list_node = TreeNode::new(bullet_list_data);

    tree_container.tree.node.push_child(list_node);

    tree_container.tree = match tree_container.tree.focus_on_last_child() {
      Ok(child_zipper) => child_zipper,
      Err(e) => {
        eprintln!("{}", e);
        return Err("An error occurred when adding a child to the current node.\n");
      }
    };

    let next_state = StateMachine::new(pattern_name);

    Ok( ( Some(tree_container), Some(next_state), PushOrPop::Push ) )

  }

  pub fn enumerator (doctree: Option<DocTree>, captures: regex::Captures) -> Result<(Option<DocTree>, Option<StateMachine>), &'static str> {
    todo!();
  }

}

impl From<MachineWithState<Body>> for MachineWithState<BulletList> {
  fn from(machine: MachineWithState<Body>) -> Self {
    MachineWithState {
      state: BulletList::new()
    }
  }
}

/// ### BulletList
/// A transition to this state is made if a `BulletList`
/// is detected in state `Body`. Handles subsequent
/// `BulletList` items.
pub struct BulletList {
  pub transitions: &'static Vec<Transition>
}

impl BulletList {

  /// ### new
  /// A `BulletList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("Bullet").unwrap()
    }
  }


  /// ### bullet
  /// A `BulletList` version of the bullet list related
  /// transition method. Differs from the `Body` state version
  /// in that this detects whether a list of a different type has started
  /// and acts accordingly.
  pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop), &'static str> {

    let mut tree_wrapper = doctree.unwrap();

    let list_item_bullet = captures.get(1).unwrap().as_str().chars().next().unwrap();
    let list_item_indent = captures.get(0).unwrap().end();

    let (list_bullet, list_indent) = match &tree_wrapper.tree.node.data {
      doctree::TreeNodeType::BulletList(bullet_list_node) => (bullet_list_node.bullet, bullet_list_node.indent),
      _ => return Err("Only bullet list nodes contain bullets\nCannot compare detected bullet with parent...\n")
    };

    // If bullet and indentation match with current list node, continue with current list.
    // Else check for possible sublist or need to break out of current list and act accordingly.
    match (list_item_bullet, list_item_indent) {
      (bullet, indent) if bullet == list_bullet && indent == list_indent => {

        // Still within same list based on indentation an bullet.
        // Create new ListItem node, read in the next block of text with known
        // indent with Parser::read_indented_block and parse it for inline elements,
        // feeding those to the ListItem node.

        let item_node = doctree::TreeNode::new(TreeNodeType::ListItem(body_nodes::ListItem{}));

        tree_wrapper.tree.push_child(item_node);
        tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
          Ok(tree_zipper) => tree_zipper,
          Err(e) => {
            eprintln!("{}", e);
            return Err("No child of type ListItem to be focused on.\n")
          }
        };

        // Read indented block here
        let block = match Parser::read_indented_block(src_lines, Some(*current_line), None, None, Some(indent), Some(indent)) {
          Ok((lines, min_indent, line_offset, blank_finish)) => {

            if min_indent != indent {
              return Err("Indent of list item block was less than given.")
            }

            *current_line += line_offset; // update current line after reading block

            lines.join("\n")

          }

          Err(e) => {
            eprintln!("{}", e);
            return Err("Error when reading list item block.\n")
          }

        };


        // Pass text to inline parser as a string

        let inline_parser = MachineWithState::<Inline>::from(MachineWithState::new());

        let mut inline_nodes = if let Some(children) = inline_parser.parse(block, current_line) {
          children
        } else {
          Vec::new()
        };

        // Add inline nodes to list item node
        tree_wrapper.tree.append_children(&mut inline_nodes);
        
        // Move focus back to parent list so new list items might be appended
        tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
          Ok(parent) => parent,
          Err(e) => {
            eprintln!("{}", e);
            return Err("Cannot focus on parent bullet list\n...")
          }
        };

        return Ok((Some(tree_wrapper), None, PushOrPop::Neither))

      },

      (bullet, indent) if bullet != list_bullet && indent == list_indent => {

        // If bullet doesn't match but indent is the same, we have another list on the same level
        //   => simply move focus back to parent so the new list might be appended to it

        tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
          Ok(parent) => parent,
          Err(e) => {
            eprintln!("{}", e);
            return Err("Encountered list on same level but couldn't focus on list parent.\n")
          }
        };

        return Ok((Some(tree_wrapper), None, PushOrPop::Neither))

      },

      (bullet, indent) if indent < list_indent => {

        // Less indent after discovering a bullet means a sublist has ended,
        // regardless of bullet type.
        // Move focus back to parent and pop from machine stack.

        tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
          Ok(parent) => parent,
          Err(e) => {
            eprintln!("{}", e);
            return Err("Encountered a list item with less indent but couldn't focus on list parent.\n")
          }
        };

        return Ok((Some(tree_wrapper), None, PushOrPop::Pop))

      },

      (bullet, indent) if indent > list_indent => {

        // More indent after discovering a bullet means a sublist has started,
        // regardless of bullet type.
        // Create an entirely new list nodem add it to the children of the current list item
        // and push a new bullet machine on top of the
        // parser stack to signify an increase in nesting level.

        let bullet_list_data = TreeNodeType::BulletList(body_nodes::BulletList::new(bullet, indent));

        let list_node = TreeNode::new(bullet_list_data);

        let new_machine = StateMachine::BulletList(MachineWithState::<BulletList>::from(MachineWithState::new()));



        todo!();

      }

      _ => {
        return Err("No action for this type of bullet--indent combination")
      }
    }

  }

}

/// ### Definition
/// A state for handling the second line of a possible
/// `DefinitionList` items.
pub struct Definition {
  pub transitions: &'static Vec<Transition>
}

impl Definition {

  /// ### new
  /// A `Definition` state constructor
  pub fn new () -> Self {
    Self{
      transitions: transitions::TRANSITION_MAP.get("Definition").unwrap()
    }
  }

}


/// ### DefinitionList
/// This state is transitioned to if a first line of `DefinitionList`
/// is detected. Handles the subsequent lines.
pub struct DefinitionList {
  pub transitions: &'static Vec<Transition>
}

impl DefinitionList {

  /// ### new
  /// A `DefinitionList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("DefinitionList").unwrap()
    }
  }
}

/// ### EnumeratedList
/// A state that parses the lines followed by the detection of
/// the first line of a possibly detected `EnumeratedList`.
pub struct EnumeratedList {
  pub transitions: &'static Vec<Transition>
}

impl EnumeratedList {

  /// ### new
  /// An `EnumeratedList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("EnumeratedList").unwrap()
    }
  }

}

/// ### ExplicitMarkup
/// A state for parsing explicit markup followed by the detection
/// of a first such item.
pub struct ExplicitMarkup {
  pub transitions: &'static Vec<Transition>
}

impl ExplicitMarkup {

  /// ### new
  /// An `ExplicitMarkup` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("ExplicitMarkup").unwrap()
    }
  }

}

/// ### ExtensionOptions
/// A state for parsing directive option fields.
pub struct ExtensionOptions {
  pub transitions: &'static Vec<Transition>
}

impl ExtensionOptions {

  /// ### new
  /// An `ExtenstionOptions` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("ExtenstionOptions").unwrap()
    }
  }

}

/// ### FieldList
/// A state for parsing subsequent fields in a field list.
pub struct FieldList {
  pub transitions: &'static Vec<Transition>
}

impl FieldList {

  /// ### new
  /// An `FieldList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("FieldList").unwrap()
    }
  }

}



/// ### Line
/// A state for parsing a detected `Line` (section titles and such).
pub struct Line {
  pub transitions: &'static Vec<Transition>
}

impl Line {

  /// ### new
  /// An `Line` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("Line").unwrap()
    }
  }

}

/// ### LineBlock
/// A state for parsing subsequent lines of a line block.
pub struct LineBlock {
  pub transitions: &'static Vec<Transition>
}

impl LineBlock{

  /// ### new
  /// An `LineBlock` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("LineBlock").unwrap()
    }
  }

}


/// ### A state for  parsing subsequent option list items.
pub struct OptionList {
  pub transitions: &'static Vec<Transition>
}

impl OptionList {

  /// ### new
  /// An `OptionList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("OptionList").unwrap()
    }
  }

}

/// ### RFC2822Body
/// A state for parsing body items that follow the RFC2822 specification.
pub struct RFC2822Body {
  pub transitions: &'static Vec<Transition>
}

/// ### RFC2822List
/// A state for parsing list items that follow the RFC2822 specification.
pub struct RFC2822List {
  pub transitions: &'static Vec<Transition>
}

/// ### SubstitutionDef
/// A state for parsing substitution definitions
pub struct SubstitutionDef {
  pub transitions: &'static Vec<Transition>
}

impl SubstitutionDef {

  /// ### new
  /// A `SubstitutioDef` state constructor
  pub fn new() -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("SubstitutionDef").unwrap()
    }
  }

}

/// ### Text
/// A state for parsing generic text.
pub struct Text {
  pub transitions: &'static Vec<Transition>
}

impl Text {

  /// ### new
  /// A `Text` state constructor
  pub fn new() -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("Text").unwrap()
    }
  }

}


/// ### Inline
/// A state for inline parsing. This state is different from the other ones,
/// as it is not used at the `Parser` level. Instead,
/// individual transition methods may initiate a
/// `MachineWithState<Inline>` for parsing an inline block of text.
pub struct Inline {
  pub transitions: &'static Vec<(PatternName, regex::Regex, InlineParsingMethod)>
}

impl Inline {

  /// ### new
  /// An `Inline` state constructor
  pub fn new() -> Self{
    Self {
      transitions: &COMPILED_INLINE_TRANSITIONS
    }
  }


  /// ### paired_delimiter
  /// Parses inline text elements that have simple opening
  /// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
  pub fn paired_delimiter (pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {
    
    let content = captures.get(1).unwrap();

    let data = String::from(content.as_str());

    let node = match pattern_name {
      PatternName::StrongEmphasis => TreeNode::new(TreeNodeType::StrongEmphasis(inline_nodes::StrongEmphasis{text: data})),
      PatternName::Emphasis => TreeNode::new(TreeNodeType::Emphasis(inline_nodes::Emphasis{text: data})),
      PatternName::Literal => TreeNode::new(TreeNodeType::Literal(inline_nodes::Literal{text: data})),
      PatternName::InlineTarget => TreeNode::new(TreeNodeType::InlineTarget(inline_nodes::InlineTarget{target_label: data})),
      _ => panic!("No such paired delimiter type!")
    };

    assert!(node.children.is_empty());

    let match_len = captures.get(0).unwrap().as_str().chars().count();

    (node, match_len)

  }


  /// ### whitespace
  /// Parses inline whitespace
  pub fn whitespace(pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {

    let content = captures.get(0).unwrap();

    let data = TreeNodeType::WhiteSpace(inline_nodes::WhiteSpace{text: String::from(content.as_str())});

    let node = TreeNode::new(data);

    let match_len = content.as_str().chars().count();

    (node, match_len)

  }


  /// ### reference
  /// Parses reference type inline elements based on their pattern name.
  pub fn reference(pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {

    let whole_match = captures.get(0).unwrap();


    let data = match pattern_name {
      PatternName::SimpleRef | PatternName::PhraseRef => {
        let target_label = captures.get(1).unwrap();
        TreeNodeType::Reference(inline_nodes::Reference{target_label: String::from(target_label.as_str())})
      },
      PatternName::FootNoteRef => {
        let target_label = captures.get(1).unwrap();
        TreeNodeType::FootnoteReference(inline_nodes::FootnoteReference{target_label: String::from(target_label.as_str())})
      },
      PatternName::SubstitutionRef => {
        let target_label = captures.get(1).unwrap();
        TreeNodeType::SubstitutionReference(inline_nodes::SubstitutionReference{text: String::from(target_label.as_str())})
      },
      PatternName::StandaloneHyperlink => {

        let mut is_valid = true;

        const MISSING: &str = "!!!MISSING!!!";

        // Retrieving the relevant parts of the URI as &str
        let scheme = if let Some(scheme) = captures.name("scheme") {
          scheme.as_str()
        } else {
          MISSING
        };

        eprintln!("Scheme: {:#?}", scheme);

        match scheme {
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
              let data = TreeNodeType::Text(inline_nodes::Text{text: String::from(whole_match.as_str())});
              let text_node = TreeNode::new(data);
              return (text_node, match_str.chars().count())
            }

            let match_str = whole_match.as_str();

            // If a successful email recognition, prepend a mailto scheme to email.
            TreeNodeType::StandaloneEmail(inline_nodes::StandaloneEmail{text: format!("{}{}", "mailto:", match_str)})

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
              TreeNodeType::AbsoluteURI(inline_nodes::AbsoluteURI{text: String::from(whole_match.as_str())})
            } else {
              TreeNodeType::Text(inline_nodes::Text{text: String::from(whole_match.as_str())})
            }

          }
        }
      }
      _ => panic!("No such reference pattern.\n")
    };

    let node = TreeNode::new(data);

    let match_len = whole_match.as_str().chars().count();

    (node, match_len)
  }


  /// ### text
  /// Parses inline text elements that have simple opening
  /// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
  pub fn text (pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {

    let content = captures.get(1).unwrap();

    let match_len = content.as_str().chars().count();

    let data = String::from(content.as_str());

    let node = TreeNode::new(TreeNodeType::Text(inline_nodes::Text{text: data}));

    assert!(node.children.is_empty());

    (node, match_len)

  }

}


/// ### Failure
/// A failure state, which is entered if no match in current state is found.
pub struct Failure;


impl From<MachineWithState<Body>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Body>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<BulletList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<BulletList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<Definition>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Definition>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<DefinitionList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<DefinitionList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<EnumeratedList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<EnumeratedList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<ExplicitMarkup>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<ExplicitMarkup>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<ExtensionOptions>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<ExtensionOptions>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<FieldList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<FieldList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<Line>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Line>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<LineBlock>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<LineBlock>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<OptionList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<OptionList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<SubstitutionDef>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<SubstitutionDef>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<Text>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Text>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}
