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

    let bullet_list_data = TreeNodeType::BulletList(doctree::body_nodes::BulletList::new(bullet, indent, nesting_level));

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

            let src_block = lines.join("\n");



          }

          Err(e) => {
            eprintln!("{}", e);
            return Err("Error when reading list item block.\n")
          }

        };


        // Pass text to inline parser as a string

        let inline_parser = MachineWithState::<Inline>::from(MachineWithState::new());

        todo!();
      },
      _ => {
        return Err("No action for this  type of bullet--indent combination")
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


  /// ### simple_reference
  /// Parses a simple reference.
  pub fn reference(pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {

    let whole_match = captures.get(0).unwrap();

    let target_label = captures.get(1).unwrap();

    let data = TreeNodeType::Reference(inline_nodes::Reference{target_label: String::from(target_label.as_str())});


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
