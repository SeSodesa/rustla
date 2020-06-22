/// This submodule contains the states and related transitions of
/// the parser state machine.

//pub mod body;

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
  /// Causes the parser to push a new machine in the state
  /// `BulletList` on top of its machine stack.
  pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>), &'static str> {

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

    let next_state = StateMachine::new(pattern_name);

    Ok( ( Some(tree_container), Some(next_state) ) )

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
  pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>), &'static str> {

    let bullet = captures.get(1).unwrap().as_str().chars().next().unwrap();

    todo!();

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
