/// This submodule contains `Body` level node specifications.

use super::*;
  
#[derive(Debug)]
pub struct Paragraph {
  id: usize,
}
#[derive(Debug)]
pub struct Compound {
  id: usize,
}
#[derive(Debug)]
pub struct Container {
  id: usize,
}
#[derive(Debug)]
pub struct BulletList {
  id: usize,

}
#[derive(Debug)]
pub struct EnumeratedList {
  id: usize,
}

#[derive(Debug)]
pub struct ListItem {
  id: usize,
}

#[derive(Debug)]
pub struct DefinitionList {
  id: usize,
}

#[derive(Debug)]
pub struct DefinitionListItem {
  id: usize,
}

#[derive(Debug)]
pub struct Term {
  id: usize,
}

#[derive(Debug)]
pub struct Classifier {
  id: usize,
}

#[derive(Debug)]
pub struct Definition {
  id: usize,
}

#[derive(Debug)]
pub struct FieldList {
  id: usize,
}

#[derive(Debug)]
pub struct Field {
  id: usize,
}

#[derive(Debug)]
pub struct FieldName {
  id: usize,
}

#[derive(Debug)]
pub struct FieldBody {
  id: usize,
}

#[derive(Debug)]
pub struct Option {
  id: usize,
}

#[derive(Debug)]
pub struct OptionArgument {
  id: usize,
}

#[derive(Debug)]
pub struct OptionGroup {
  id: usize,
}

#[derive(Debug)]
pub struct OptionList {
  id: usize,
}

#[derive(Debug)]
pub struct OptionListItem {
  id: usize,
}
#[derive(Debug)]
pub struct OptionString {
  id: usize,
}

#[derive(Debug)]
pub struct Description {
  id: usize,
}

#[derive(Debug)]
pub struct LiteralBlock {
  id: usize,
}
#[derive(Debug)]
pub struct DoctestBlock {
  id: usize,
}
#[derive(Debug)]
pub struct MathBlock {
  id: usize,
}
#[derive(Debug)]
pub struct LineBlock {
  id: usize,
}
#[derive(Debug)]
pub struct Line {
  id: usize,
}
#[derive(Debug)]
pub struct BlockQuote {
  id: usize,
}
#[derive(Debug)]
pub struct Attribution {
  id: usize,
}
#[derive(Debug)]
pub struct Attention {
  id: usize,
}
#[derive(Debug)]
pub struct Caution {
  id: usize,
}
#[derive(Debug)]
pub struct Danger {
  id: usize,
}
#[derive(Debug)]
pub struct Error {
  id: usize,
}
#[derive(Debug)]
pub struct Important {
  id: usize,
}
#[derive(Debug)]
pub struct Note {
  id: usize,
}
#[derive(Debug)]
pub struct Tip {
  id: usize,
}
#[derive(Debug)]
pub struct Hint {
  id: usize,
}
#[derive(Debug)]
pub struct Warning {
  id: usize,
}
#[derive(Debug)]
pub struct Admonition {
  id: usize,
}
#[derive(Debug)]
pub struct Comment {
  id: usize,
}
#[derive(Debug)]
pub struct SubstitutionDefinition {
  id: usize,
}

#[derive(Debug)]
pub struct Target {
  id: usize,
}
#[derive(Debug)]
pub struct Footnote {
  id: usize,
}
#[derive(Debug)]
pub struct Citation {
  id: usize,
}
#[derive(Debug)]
pub struct Label {
  id: usize,
}
#[derive(Debug)]
pub struct Figure {
  id: usize,
}
#[derive(Debug)]
pub struct Caption {
  id: usize,
}
#[derive(Debug)]
pub struct Legend {
  id: usize,
}
#[derive(Debug)]
pub struct Table {
  id: usize,
}
#[derive(Debug)]
pub struct TableGroup {
  id: usize,
}
#[derive(Debug)]
pub struct ColSpec {
  id: usize,
}
#[derive(Debug)]
pub struct TableHead {
  id: usize,
}
#[derive(Debug)]
pub struct TableBody {
  id: usize,
}
#[derive(Debug)]
pub struct TableRow {
  id: usize,
}
#[derive(Debug)]
pub struct TableEntry {
  id: usize,
}
