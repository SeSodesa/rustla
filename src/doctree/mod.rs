/// This module defines the document tree and its nodes

use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

mod tests;

mod tree_zipper;
  use tree_zipper::TreeZipper;

mod node_types;
pub mod structural_nodes;
pub mod body_nodes;
pub mod inline_nodes;
use self::node_types::BranchNodeType;

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

  /// #### src_line
  /// The row currently under inspection by the parser.
  src_line: usize,

  /// #### indirect_target_nodes
  /// A vector of indirect target nodes.
  indirect_target_nodes: NodeRefVec,

  /// #### substitutiton_defs
  /// A map of substitution names to nodes containing substitution definitions.
  substitution_defs: HashMap<String, TreeNodeType>,

  /// #### substitution_names
  /// A mapping of case-normalized substitution names to the original names.
  substitution_names: HashMap<String, String>,

  /// #### refs_to_nodes
  /// A mapping of reference names to reference nodes.
  refs_to_nodes: HashMap<String, NodeRefVec>,

  /// #### ids_to_nodes
  /// A mapping of ids to vectors of reference nodes.
  ids_to_nodes: HashMap<usize, NodeRefVec>,

  /// #### names_to_ids
  /// A mapping of node names to their unique ids.
  names_to_ids: HashMap<String, usize>,

}


/// ### DocTree
/// Document tree container methods
impl DocTree {

  /// ### new
  /// A `DocTree` constructor.
  pub fn new(doc_name: String) -> Self {

    let root_data = TreeNodeType::Root{doc_name};

    let root_node = TreeNode::new(root_data);

    let zipper = TreeZipper::new(root_node, None, None);

    DocTree {
      tree: zipper,
      src_line: 0,
      indirect_target_nodes: Vec::new(),
      substitution_defs: HashMap::new(),
      substitution_names: HashMap::new(),
      refs_to_nodes: HashMap::new(),
      ids_to_nodes: HashMap::new(),
      names_to_ids: HashMap::new(),
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
  pub fn push_child (&mut self, node : TreeNode) {

    self.children.push(node);

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
#[derive(Debug, PartialEq)]
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
  DefinitionList(body_nodes::DefinitionList),

  /// #### DefinitionListItem
  /// A child node type of `DefinitionList`.
  /// Contains a map of `DefinitionTerm`s and the corresponding
  /// `TermDefinitions`, in addition to optional term classifiers.
  DefinitionListItem(body_nodes::DefinitionListItem),

  /// #### DefinitionTerm
  /// The term that is to be defined in a `DefinitionList`.
  DefinitionTerm(body_nodes::Term),

  /// #### Classifier
  /// A classifier for a `DefinitionTerm` in a `DefinitionList`.
  /// Could be the type of a varible in a function decraration, or something similar.
  Classifier(body_nodes::Classifier),

  /// #### TermDefinition
  /// A definition of a term in a `DefinitionList`.
  TermDefinition(body_nodes::Definition),

  /// #### FieldList
  /// A list of fields, that are used as a part of the
  /// reStructuredText extension syntax, such as directives.
  /// Bibliographies are a special case of these types of lists.
  FieldList {

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
    name: String,

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
