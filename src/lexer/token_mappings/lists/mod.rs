/// This file contains regexes for reStructuredText
/// lists. It is a submodule of lexer::regex.

mod test;

use crate::lexer::token::TokenType;

/// A map (list) of possible list
/// type--regex-pairs.
static LIST_RE_MAP: &[(TokenType, &'static str)] = &[
  (TokenType::UnnumberedList, r"(?m)^\s*[*\-+] .+\n(?:[*\-+] .+\n)+"),
  (TokenType::NumberedDotList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\. .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*"),
  (TokenType::NumberedLRparList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\) .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*"),
  (TokenType::NumberedRparList, r"(?m)^\s*[0-9#ivxlcmIVXLCM]+\) .+\n(?:[0-9#ivxlcmIVXLCM]+\) .+\n)*"),
  (TokenType::NoBolAlphaDotList, r"(?m)^\s*[A-Z]+\. .+\n(?:[ \t]*[A-Z]+\. .+\n)+"),
  (TokenType::AlphaLRparList, r"(?m)^\s*\(?[a-zA-Z]+\) .+\n(?:[ \t]*\([a-zA-Z]+\) .+\n)+"),
  (TokenType::AlphaRparList, r"(?m)^\s*[a-zA-Z]+\) .+\n(?:[ \t]*[a-zA-Z]+\) .+\n)+"),
  (TokenType::DefinitionList, r"(?m)^(?:(\s*).+\n(?:  .+\n)+\s)+"),
  (TokenType::FieldList, r"(?m)^\s*(?::.+: .+\n(?:[ \t]{2}.+\n)*)+")
];

