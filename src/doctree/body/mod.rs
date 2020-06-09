/// This submodule contains `Body` level node specifications.

use super::*;
  
pub struct Paragraph {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Compound {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Container {
  id: usize,
  children: Vec<DocNode>,
}
pub struct BulletList {
  id: usize,
  children: Vec<DocNode>,
}
pub struct EnumeratedList {
  id: usize,
  children: Vec<DocNode>,
}

pub struct ListItem {
  id: usize,
  children: Vec<DocNode>,
}
pub struct DefinitionList {
  id: usize,
  children: Vec<DocNode>,
}
pub struct DefinitionListItem {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Term {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Classifier {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Definition {
  id: usize,
  children: Vec<DocNode>,
}
pub struct FieldList {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Field {
  id: usize,
  children: Vec<DocNode>,
}
pub struct FieldName {
  id: usize,
  children: Vec<DocNode>,
}
pub struct FieldBody {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Option {
  id: usize,
  children: Vec<DocNode>,
}
pub struct OptionArgument {
  id: usize,
  children: Vec<DocNode>,
}
pub struct OptionGroup {
  id: usize,
  children: Vec<DocNode>,
}
pub struct OptionList {
  id: usize,
  children: Vec<DocNode>,
}
pub struct OptionListItem {
  id: usize,
  children: Vec<DocNode>,
}
pub struct OptionString {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Description {
  id: usize,
  children: Vec<DocNode>,
}
pub struct LiteralBlock {
  id: usize,
  children: Vec<DocNode>,
}
pub struct DoctestBlock {
  id: usize,
  children: Vec<DocNode>,
}
pub struct MathBlock {
  id: usize,
  children: Vec<DocNode>,
}
pub struct LineBlock {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Line {
  id: usize,
  children: Vec<DocNode>,
}
pub struct BlockQuote {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Attribution {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Attention {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Caution {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Danger {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Error {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Important {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Note {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Tip {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Hint {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Warning {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Admonition {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Comment {
  id: usize,
  children: Vec<DocNode>,
}
pub struct SubstitutionDefinition {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Target {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Footnote {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Citation {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Label {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Figure {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Caption {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Legend {
  id: usize,
  children: Vec<DocNode>,
}
pub struct Table {
  id: usize,
  children: Vec<DocNode>,
}
pub struct TableGroup {
  id: usize,
  children: Vec<DocNode>,
}
pub struct ColSpec {
  id: usize,
  children: Vec<DocNode>,
}
pub struct TableHead {
  id: usize,
  children: Vec<DocNode>,
}
pub struct TableBody {
  id: usize,
  children: Vec<DocNode>,
}
pub struct TableRow {
  id: usize,
  children: Vec<DocNode>,
}
pub struct TableEntry {
  id: usize,
  children: Vec<DocNode>,
}