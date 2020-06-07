/// This submodule contains the different
/// document tree node types


/// #BranchNodeType
/// An enumeration fo the different kinds of
/// branch nodes, as in nodes that can have children.
/// Taken from https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils/docutils/nodes.py#l1168
pub enum BranchNodeType {
  Root,
  Titular,
  PreBibliographic,
  Bibliographic,
  Decorative,
  Structural,
  Body,
  Sequential,
  Admonition,
  Special,
  Invisible,
  Part,
  Inline,
  Referenctial,
  Targetable,
  Labeled,
}
