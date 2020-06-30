/// This submodule contains `Body` level node specifications.

use super::*;
  
#[derive(Debug, PartialEq)]
pub struct Paragraph {
  
}
#[derive(Debug, PartialEq)]
pub struct Compound {
  
}
#[derive(Debug, PartialEq)]
pub struct Container {
  
}
#[derive(Debug, PartialEq)]
pub struct BulletList {
  pub bullet: char, // might need to be changed to a grapheme (needs external crate)
  pub indent: usize,
}

impl BulletList {

  /// ### new
  /// BulletList node type constructor
  pub fn new (bullet: char, indent: usize) -> Self {
    Self {
      bullet: bullet,
      indent: indent,
    }
  }

}


#[derive(Debug, PartialEq)]
pub struct EnumeratedList {
  
}

#[derive(Debug, PartialEq)]
pub struct ListItem {
  
}

#[derive(Debug, PartialEq)]
pub struct DefinitionList {
  
}

#[derive(Debug, PartialEq)]
pub struct DefinitionListItem {
  
}

#[derive(Debug, PartialEq)]
pub struct Term {
  
}

#[derive(Debug, PartialEq)]
pub struct Classifier {
  
}

#[derive(Debug, PartialEq)]
pub struct Definition {
  
}

#[derive(Debug, PartialEq)]
pub struct FieldList {
  
}

#[derive(Debug, PartialEq)]
pub struct Field {
  
}

#[derive(Debug, PartialEq)]
pub struct FieldName {
  
}

#[derive(Debug, PartialEq)]
pub struct FieldBody {
  
}

#[derive(Debug, PartialEq)]
pub struct Option {
  
}

#[derive(Debug, PartialEq)]
pub struct OptionArgument {
  
}

#[derive(Debug, PartialEq)]
pub struct OptionGroup {
  
}

#[derive(Debug, PartialEq)]
pub struct OptionList {
  
}

#[derive(Debug, PartialEq)]
pub struct OptionListItem {
  
}
#[derive(Debug, PartialEq)]
pub struct OptionString {
  
}

#[derive(Debug, PartialEq)]
pub struct Description {
  
}

#[derive(Debug, PartialEq)]
pub struct LiteralBlock {
  
}
#[derive(Debug, PartialEq)]
pub struct DoctestBlock {
  
}
#[derive(Debug, PartialEq)]
pub struct MathBlock {
  
}
#[derive(Debug, PartialEq)]
pub struct LineBlock {
  
}
#[derive(Debug, PartialEq)]
pub struct Line {
  
}
#[derive(Debug, PartialEq)]
pub struct BlockQuote {
  
}
#[derive(Debug, PartialEq)]
pub struct Attribution {
  
}
#[derive(Debug, PartialEq)]
pub struct Attention {
  
}
#[derive(Debug, PartialEq)]
pub struct Caution {
  
}
#[derive(Debug, PartialEq)]
pub struct Danger {
  
}
#[derive(Debug, PartialEq)]
pub struct Error {
  
}
#[derive(Debug, PartialEq)]
pub struct Important {
  
}
#[derive(Debug, PartialEq)]
pub struct Note {
  
}
#[derive(Debug, PartialEq)]
pub struct Tip {
  
}
#[derive(Debug, PartialEq)]
pub struct Hint {
  
}
#[derive(Debug, PartialEq)]
pub struct Warning {
  
}
#[derive(Debug, PartialEq)]
pub struct Admonition {
  
}
#[derive(Debug, PartialEq)]
pub struct Comment {
  
}
#[derive(Debug, PartialEq)]
pub struct SubstitutionDefinition {
  
}

#[derive(Debug, PartialEq)]
pub struct Target {
  
}
#[derive(Debug, PartialEq)]
pub struct Footnote {
  
}
#[derive(Debug, PartialEq)]
pub struct Citation {
  
}
#[derive(Debug, PartialEq)]
pub struct Label {
  
}
#[derive(Debug, PartialEq)]
pub struct Figure {
  
}
#[derive(Debug, PartialEq)]
pub struct Caption {
  
}
#[derive(Debug, PartialEq)]
pub struct Legend {
  
}
#[derive(Debug, PartialEq)]
pub struct Table {
  
}
#[derive(Debug, PartialEq)]
pub struct TableGroup {
  
}
#[derive(Debug, PartialEq)]
pub struct ColSpec {
  
}
#[derive(Debug, PartialEq)]
pub struct TableHead {
  
}
#[derive(Debug, PartialEq)]
pub struct TableBody {
  
}
#[derive(Debug, PartialEq)]
pub struct TableRow {
  
}
#[derive(Debug, PartialEq)]
pub struct TableEntry {
  
}
