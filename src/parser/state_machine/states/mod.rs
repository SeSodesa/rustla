/// This submodule contains the states and related transitions of
/// the parser state machine.

pub mod body;

use super::*;
use crate::doctree;

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
  pub fn bullet (doctree: Option<DocTree>, captures: regex::Captures) -> Result<(Option<DocTree>, Option<StateMachine>), &'static str> {

    let mut tree_container = doctree.unwrap();

    let bullet = captures.get(1).unwrap().as_str().chars().next().unwrap();
    let indent = captures.get(0).unwrap().end();
    let nesting_level: usize = 0;

    let bullet_list_data = doctree::TreeNodeType::BulletList(doctree::body::BulletList::new(bullet, indent, nesting_level));

    let list_node = doctree::TreeNode::new(bullet_list_data);

    tree_container.tree.node.push_child(list_node);

    tree_container.tree = match tree_container.tree.focus_on_last_child() {
      Ok(child_zipper) => child_zipper,
      Err(e) => {
        eprintln!("{}", e);
        return Err("An error occurred when adding a child to the current node.\n");
      }
    };

    // Create logic for building the next BulletList state.

    todo!();

  }

  pub fn enumerator (doctree: Option<DocTree>, captures: regex::Captures) -> Result<(Option<DocTree>, Option<StateMachine>), &'static str> {
    todo!();
  }

}


impl From<MachineWithState<Body>> for MachineWithState<BulletList> {

  fn from(machine: MachineWithState<Body>) -> MachineWithState<BulletList> {

    // parsing before transition

    MachineWithState {
      state: BulletList { transitions: TRANSITION_MAP.get("Bullet").unwrap() },
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
  fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get("Bullet").unwrap()
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

  fn new () -> Self {
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


/// ### EnumeratedList
/// A state that parses the lines followed by the detection of
/// the first line of a possibly detected `EnumeratedList`.
pub struct EnumeratedList {
  pub transitions: &'static Vec<Transition>
}

/// ### ExplicitMarkup
/// A state for parsing explicit markup followed by the detection
/// of a first such item.
pub struct ExplicitMarkup {
  pub transitions: &'static Vec<Transition>
}


/// ### ExtensionOptions
/// A state for parsing directive option fields.
pub struct ExtensionOptions {
  pub transitions: &'static Vec<Transition>
}


/// ### FieldList
/// A state for parsing subsequent fields in a field list.
pub struct FieldList {
  pub transitions: &'static Vec<Transition>
}


/// ### Line
/// A state for parsing a detected `Line` (section titles and such).
pub struct Line {
  pub transitions: &'static Vec<Transition>
}



/// ### LineBlock
/// A state for parsing subsequent lines of a line block.
pub struct LineBlock {
  pub transitions: &'static Vec<Transition>
}


/// ### A state for  parsing subsequent option list items.
pub struct OptionList {
  pub transitions: &'static Vec<Transition>
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

/// ### Text
/// A state for parsing generic text.
pub struct Text {
  pub transitions: &'static Vec<Transition>
}
