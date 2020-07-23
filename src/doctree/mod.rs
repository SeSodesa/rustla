/// This module defines the document tree and its nodes

use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

mod tests;

mod tree_zipper;
use tree_zipper::TreeZipper;

use crate::utils::{EnumDelims, EnumKind};

/// ### DocTree
/// A container for the document tree.
/// In addition to holding ownership of the
/// tree (stored in a zipper), also contains
/// metadata about the tree.
pub struct DocTree {

  /// #### tree
  /// Holds the tree focused on a specific node.
  pub tree: TreeZipper,

  /// #### node_counter
  /// Keeps track of how many nodes have been added to the tree thus far
  /// besides the root node, that gets an ID of `0`. Some nodes might differ
  /// in their behaviour depending on their insertion order into the tree.
  /// For example, a field list will be transformed into bibliographic data,
  /// if it is the first non-(whitespace|comment) node in the tree.
  node_counter: u32,

}


/// ### DocTree
/// Document tree container methods
impl DocTree {

  /// ### new
  /// A `DocTree` constructor.
  pub fn new(doc_name: String) -> Self {

    let root_id: u32 = 0;

    let root_data = TreeNodeType::Root{doc_name};

    let root_node = TreeNode::new(root_data);

    let zipper = TreeZipper::new(root_node, None, None);

    DocTree {
      tree: zipper,
      node_counter: root_id,
    }

  }
}


/// ### TreeNode
/// A tree node that contains a struct of `TreeNodeType`
/// plus the information needed to traverse the tree.
#[derive(Debug)]
pub struct TreeNode {
  pub data : TreeNodeType,
  pub children: Children,

}

impl TreeNode {

  /// ### new
  /// A `TreeNode` constructor.
  pub fn new(data: TreeNodeType) -> Self {
    
    TreeNode {
      children: Vec::new(),
      data: data
    }

  }

  /// ### push_child
  /// Pushes a given child node the the end of `self.children`.
  pub fn push_child (&mut self, node : TreeNode) -> Result<(), String> {

    if self.child_is_allowed(&node.data) {
      self.children.push(node);
    } else {
      return Err(format!("Child of type {:#?} not allowed inside a {:#?}", node.data, self.data))
    }
    
    Ok(())
  }


  /// ### append_children
  /// Appends multiple children to `self.children`.
  pub fn append_children(&mut self, children: &mut Vec<TreeNode>) {
    self.children.append(children);
  }

  /// ### traverse
  /// Traverses `TreeNode`s recursively.
  fn traverse(&mut self) {

    eprintln!("Entering {:?}", self.get_data_type());

    let children = &mut self.children;

    for child in children {
      child.traverse();
    }

  }


  /// ### child_is_allowed
  /// Checks whether a node is allowed to be inserted into another node.
  pub fn child_is_allowed (&self, node_data: &TreeNodeType) -> bool {

    match self.data {

      TreeNodeType::Root { .. }           | TreeNodeType::BulletListItem { .. } | TreeNodeType::EnumeratedListItem { .. }
      | TreeNodeType::DefinitionListItem  | TreeNodeType::FieldListItem { .. }  | TreeNodeType::OptionListItem
      | TreeNodeType::BlockQuote          | TreeNodeType::Footnote              | TreeNodeType::Citation  => {
        match node_data {
          TreeNodeType::Paragraph             | TreeNodeType::BulletList { .. } | TreeNodeType::EnumeratedList { .. }
          | TreeNodeType::DefinitionList      | TreeNodeType::FieldList { .. }  | TreeNodeType::OptionList
          | TreeNodeType::LiteralBlock { .. } | TreeNodeType::LineBlock         | TreeNodeType::BlockQuote
          | TreeNodeType::DoctestBlock        | TreeNodeType::Footnote          | TreeNodeType::Citation
          | TreeNodeType::HyperlinkTarget     | TreeNodeType::Directive         | TreeNodeType::SubstitutionDefinition
          | TreeNodeType::Comment             | TreeNodeType::EmptyLine         => true,
          _ => false
        }
      },

      TreeNodeType::BulletList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::BulletListItem { .. } => true,
          _ => false
        }
      }

      TreeNodeType::EnumeratedList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::EnumeratedListItem { .. } => true,
          _ => false
        }
      }

      TreeNodeType::FieldList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::FieldListItem { .. } => true,
          _ => false
        }
      }

      TreeNodeType::OptionList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::OptionListItem { .. } => true,
          _ => false
        }
      }

      TreeNodeType::Paragraph => {
        match node_data {
          TreeNodeType::Emphasis { .. }             | TreeNodeType::StrongEmphasis { .. }         | TreeNodeType::InterpretedText
          | TreeNodeType::Literal { .. }            | TreeNodeType::InlineTarget { .. }           | TreeNodeType::FootnoteReference { .. }
          | TreeNodeType::CitationReference { .. }  | TreeNodeType::SubstitutionReference { .. }  | TreeNodeType::AbsoluteURI { .. }
          | TreeNodeType::StandaloneEmail { .. }    => true,
          _=> false
        }
      },
      _ => false
    }
  }

  /// ### get_data_type
  /// For retrieving an immutable reference to the data type of a node.
  /// Mainly for printing purposes.
  fn get_data_type (&self) -> &TreeNodeType {
    &self.data
  }

}


/// ### TreeNodeType
/// An enumaration of the different possible document
/// node types.
#[derive(Debug)]
pub enum TreeNodeType {

  /// #### Root
  /// The root node of an reStructuredText document tree.
  /// Contains the (name|absolute path) of the document
  /// as its only field.
  Root{
    doc_name: String
  },

  /// #### EmptyLine
  /// A simple empty line, that contains no actual data.
  /// These can be contained in pretty much any container
  /// node, such as lists or list items, in addition to
  /// existing between body level elements.
  EmptyLine,

  /// #### Section
  /// A section title node, that contains the title text,
  /// in addition to its marker type and (sub)section level.
  Section {

  },

  /// #### Transition
  /// A node corresponding to LaTeX's `\hrulefill` command.
  Transition {

  },

  // Body level elements

  /// #### Paragraph
  /// A node constructed of a left-aligned block of text
  /// with no special starter markers.
  Paragraph,

  /// #### BulletList
  /// An unnumbered list node. These may only contain `BulletListItem` nodes
  /// or `EmptyLine`s as their direct children.
  BulletList {
    bullet: char,
    bullet_indent: usize,
    text_indent: usize
  },

  /// #### BulletListItem
  /// An unnumbered list item. Cna contain any `Body` level elements
  /// as its direct children.
  BulletListItem{
    bullet: char,
    bullet_indent: usize,
    text_indent: usize
  },

  /// #### EnumeratedList
  /// An enumerated list node. Cna only contain `EnumeratedListItem` and `EmptyLine`
  /// nodes as its direct children.
  EnumeratedList {
    delims: EnumDelims,
    kind: EnumKind,
    start_index: usize,
    n_of_items: usize,
    enumerator_indent: usize,
    latest_text_indent: usize
  },

  /// #### EnumeratedListItem
  /// Child node type of `EnumeratedList`. Can contain any `Body`elements
  /// as its children.
  EnumeratedListItem {
    delims: EnumDelims,
    kind: EnumKind,
    index_in_list: usize,
    enumerator_indent: usize,
    text_indent: usize
  },

  /// #### DefinitionList
  /// A list of definitions. Contains `DefinitionListItems` or `EmptyLine` nodes
  /// as its direct children.
  DefinitionList,

  /// #### DefinitionListItem
  /// A child node type of `DefinitionList`.
  /// Contains a map of `DefinitionTerm`s and the corresponding
  /// `TermDefinitions`, in addition to optional term classifiers.
  DefinitionListItem,

  /// #### DefinitionTerm
  /// The term that is to be defined in a `DefinitionList`.
  DefinitionTerm,

  /// #### Classifier
  /// A classifier for a `DefinitionTerm` in a `DefinitionList`.
  /// Could be the type of a varible in a function decraration, or something similar.
  Classifier,

  /// #### TermDefinition
  /// A definition of a term in a `DefinitionList`.
  TermDefinition,

  /// #### FieldList
  /// A list of fields, that are used as a part of the
  /// reStructuredText extension syntax, such as directives.
  /// Bibliographies are a special case of these types of lists.
  FieldList {
    marker_indent: usize,
  },

  /// #### FieldListItem
  /// A field item of a `FieldList`. Consists of a marker with a field name and a
  /// field body consisting of arbitrary body elements.
  /// ```text
  /// +--------------------+----------------------+
  /// | ":" field name ":" | field body           |
  /// +-------+------------+                      |
  ///         | (body elements)+                  |
  ///         +-----------------------------------+
  /// ```
  FieldListItem {
    raw_marker_name: String,
    marker_name_as_inline_nodes: Vec<TreeNode>,
    marker_indent: usize,
    body_indent: usize,
  },

  /// #### FieldBody
  /// The parameter that `FieldName` refers to. May contain arbitrary body elements,
  /// just like bulleted and enumerated list items. The first line after the marker specifies
  /// the indentation used as a reference for parsing the rest of the block.
  FieldBody {
    indentation: usize
  },
  
  /// #### OptionList
  /// A two-column list of command line options, such as the ones typically seen on unix `man` pages.
  /// Four types of options are supported:
  ///
  /// 1. short POSIX options with one '-' and an opion letter,
  /// 2. Long POSIX options with "--", followed by an option word.
  ///    Some systems might use a single dash.
  /// 3. Old GNU-style options starting with a '+', followed by an option letter (!!!deprecated!!!)
  /// 4. DOS/VMS options starting with a '/', followed by an option letter or a word.
  /// 
  /// The recognized syntax is based on Python's `getopt.py` module.
  OptionList,

  /// #### OptionListItem
  /// A single option in an `OptionList`. Consists of an option,
  /// folllowed by and optional argument and a description.
  /// May contain arbitrary indented body elements after these:
  /// ```text
  /// +----------------------------+-------------+
  /// | option [" " argument] "  " | description |
  /// +-------+--------------------+             |
  ///         | (body elements)+                 |
  ///         +----------------------------------+
  /// ```
  OptionListItem,

  /// #### LiteralBlock
  /// Paragraph (possibly empty) ending in a "::" signifies the start of a literal block of text.
  /// Text contained in a literal block is not interpreted in any way,
  /// but simply stored in this node as is.
  LiteralBlock,

  /// #### DoctestBlock
  /// These are interactive Python sessions contained in Python docstrings.
  /// Based on the Python standard library [doctest](http://www.python.org/doc/current/lib/module-doctest.html) module.
  /// 
  /// Doctest blocks begin with ">>>", the python REPL main prompt and end with a blank line.
  /// They are a special case of the literal block and if both are present,
  /// the literal block takes precedence.
  DoctestBlock,
  
  /// #### MathBlock
  /// A node for display-style mathematics (LaTeX).
  MathBlock,

  /// #### LineBlock
  /// A block of text where each new line begins with an unindented '|',
  /// followed be text with specific left-alignment, used as a reference
  /// for the rest of the block.
  /// Allows writing blocks of text, where the struture of the lines
  /// is meaningful, such as poetry.
  /// 
  /// The symbols '|' may be omitted, as they signify the start of a new
  /// line in the rendered output.
  /// ```txt
  /// +------+-----------------------+
  /// | "| " | line                  |
  /// +------| continuation line     |
  ///        +-----------------------+
  /// ```
  LineBlock,

  /// #### Line
  /// A general line node. Might signify the start of a transtition or a section title.
  Line,
  
  /// #### BlockQuote
  /// Text indented relative to previous text,
  /// without markup indicating the start of
  /// a list or other container nodes.
  /// A block quote may end with an `Attribution`,
  /// which allows placing multiple block quotes
  /// in a sequence.
  BlockQuote,

  /// #### Attribution
  /// An optional attribution of a `BlockQuote`.
  /// If a `BlockQuote` contains an attribution,
  /// the following node may be a `BlockQuote as well,
  /// but not otherwise.
  Attribution,

  /// #### SubstitutionDefinition
  /// Explicit markup node, as in begins with ".. " followed by a vertical bar '|',
  /// substitution text and another '|'. The text may not begin or end with whitespace.
  /// Substitution definition blocks may contain a nested, *inline compatible* directive
  /// *without* the leading ".. ", such as `image` or `replace`.
  /// ```text
  /// +-------+-----------------------------------------------------+
  /// | ".. " | "|" substitution text "| " directive type "::" data |
  /// +-------+ directive block                                     |
  ///         |                                                     |
  ///         +-----------------------------------------------------+
  /// ```
  SubstitutionDefinition,

  /// #### Footnote
  /// A foonote citation target. Contaisn a label and the foornote text itself.
  Footnote,

  /// #### Citation
  /// A generic citation target.
  Citation,

  /// #### HyperlinkTarget
  /// A target for a hyperlink.
  HyperlinkTarget,

  /// #### Directive
  /// One of many differents kinds of directives.
  Directive,

  /// #### Comment
  /// An rST comment, that might get removed by the writer of the object code.
  Comment,


  // Inline elements
  // ---------------

  /// #### Text
  /// A plain text node, that contains no special markup.
  Text {
    text:String
  },

  /// #### Emphasis
  /// Emphasised or italicized text.
  Emphasis {
    text: String
  },

  /// #### StrongEmphasis
  /// Strongly emphasised text, usually rendered in bold.
  StrongEmphasis {
    text:String
  },

  /// #### InterpretedText
  /// Text, whose meaning depends entirely on the given `role`:
  /// (:role:`content`|`content`:role:). There are predefined roles
  /// such as `math` or `emphasis`, but others may be defined by applications.
  InterpretedText,

  /// #### Literal
  /// Literal text, usually reserved for code.
  Literal {
    text: String
  },

  /// #### InlineTarget
  /// An inline reference target.
  InlineTarget {
    target_label: String
  },

  /// #### Reference
  /// A general reference to a reference target.
  Reference {
    target_label: String
  },

  /// #### FootnoteReference
  /// A reference to a foot note.
  FootnoteReference {
    target_label: String
  },

  /// #### CitationReference
  /// A reference to a bibliographic citation.
  CitationReference {
    target_label: String
  },

  /// #### SubstitutionReference
  /// A reference that is to be substituted with the reference target directive output.
  SubstitutionReference {
    target_label: String
  },

  /// #### TitleReference
  /// A reference to a title.
  TitleReference {
    target_label: String
  },

  /// ### AbsoluteURI
  /// A reference to a web address.
  AbsoluteURI{
    text: String
  },

  /// #### StandaloneEmail
  /// A reference to an email address.
  StandaloneEmail{
    text: String
  },

  /// #### Math
  /// An inline math node.
  Math {
    text: String
  },

  /// #### Whitespace
  /// Generic whitespace that covers everything from spaces to newlines.
  WhiteSpace{
    text: String
  },

}


/// ### Parent
/// A shorthand for an optional (parent might not exist)
/// weak reference to a parent node.
type Parent = Option< Weak<RefCell<TreeNode>>>;

/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children = Vec<TreeNode>;


/// ### NodeRefVec
/// A vector of weak pointers to internally mutable nodes.
type NodeRefVec = Vec<Weak<RefCell<TreeNode>>>;
