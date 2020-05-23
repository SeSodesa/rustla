/// This file contains regex mappings for 
/// reStructuredText blocks

mod test;

use crate::lexer::token::TokenType;

static BLOCK_RE_MAP: &[(TokenType, &'static str)] = &[
  (TokenType::LiteralBlock, r"(?m)(::)[ \t]*(\n[ \t]*\n)([ \t]+)(.*)(\n)((?:(?:[ \t]+.*)?\n)+)"),
  (TokenType::PerLineLiteralBlock, r"(?m)(::)[ \t]*(\n[ \t]*\n)(>+ .*\n|>+[ \t]*\n)+\s*"),
  (TokenType::LineBlock, r"(?m)^\s*(?:\|.*\n|\|[ \t]*)+\s*"),
  (TokenType::Paragraph, r"(?m)^\s*(?:^.+\n)+\s+"),
];
