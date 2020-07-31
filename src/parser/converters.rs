/// ## Converters
/// A submodule for converters functiosn related to the parser.
/// These include Roman numeral -> integer transofrmations and the like.


use super::*;

impl Parser {

  /// ### enum_str_to_int_and_kind
  /// Converts an enumerator &str to an integer using one of the
  /// coverters, if possible.
  pub fn enum_str_to_int_and_kind (detected_enum_str: &str, detected_kind: &EnumKind, list_kind: &EnumKind, in_list_item: bool, list_item_number: Option<usize>, list_start_index: Option<usize>) -> Option<(usize, EnumKind)> {

    let list_item_number = list_item_number.unwrap_or(0);
    let list_start_index = list_start_index.unwrap_or(1);

    if detected_enum_str == "i" && list_item_number == 0 {
      // LowerRoman list at our hands
      return Some((1, EnumKind::LowerRoman))
    } else if detected_enum_str == "I" && list_item_number == 0 {
      // UpperRoman list at our hands
      return Some((1, EnumKind::UpperRoman))
    }
    
    let mut detected_kind = *detected_kind;
    let list_kind = *list_kind;

    let detected_enum_as_usize = match detected_kind {

      EnumKind::Arabic => {
        detected_enum_str.parse::<usize>().unwrap() // Standard library has implemented conversions from str to integers
      }
  
      EnumKind::LowerAlpha | EnumKind::UpperAlpha => {
        if let Some(num) = Parser::alpha_to_usize(detected_enum_str) {
          num
        } else {
          eprintln!("Couldn't convert given alphabet to an integer...\n");
          return None
        }
      }
  
      EnumKind::LowerRoman => {
        if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
          num
        } else {
          eprintln!("Couldn't convert lower-case Roman numeral to an integer...\n");
          return None
        }
      }
  
      EnumKind::UpperRoman => {
        if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
          num
        } else {
          eprintln!("Couldn't convert upper-case Roman numeral to an integer...\n");
          return None
        }
      }

      EnumKind::Automatic => {

        if list_item_number == 0 && !in_list_item {
          eprintln!("No items in list yet.\nSetting enumerator kind to Arabic...\n");
          detected_kind = EnumKind::Arabic;
        } else {
          detected_kind = list_kind;
        }
        
        list_item_number + list_start_index
      }

    };

    Some(
      (detected_enum_as_usize, detected_kind)
    )

  }


  /// ### alpha_to_usize
  /// Converts and ASCII letter to a corresponding `Option`al integer between 1--26 inclusive.
  /// Returns `None` if not successful.
  pub fn alpha_to_usize (alpha_str: &str) -> Option<usize> {
    match alpha_str {
      "A" | "a" => Some(1),
      "B" | "b" => Some(2),
      "C" | "c" => Some(3),
      "D" | "d" => Some(4),
      "E" | "e" => Some(5),
      "F" | "f" => Some(6),
      "G" | "g" => Some(7),
      "H" | "h" => Some(8),
      "I" | "i" => Some(9),
      "J" | "j" => Some(10),
      "K" | "k" => Some(11),
      "L" | "l" => Some(12),
      "M" | "m" => Some(13),
      "N" | "n" => Some(14),
      "O" | "o" => Some(15),
      "P" | "p" => Some(16),
      "Q" | "q" => Some(17),
      "R" | "r" => Some(18),
      "S" | "s" => Some(19),
      "T" | "t" => Some(20),
      "U" | "u" => Some(21),
      "V" | "v" => Some(22),
      "W" | "w" => Some(23),
      "X" | "x" => Some(24),
      "Y" | "y" => Some(25),
      "Z" | "z" => Some(26),
      _         => {
        eprintln!("Error: Letter '{}' not recognized as integer by reStructuredText...\n", alpha_str);
        None
      }
    }
  }


  /// ### upper_roman_to_usize
  /// Converts a valid given upper-case Roman numeral to a `Some(usize)`.
  /// If the numeral isn't valid, `None` is returned instead
  pub fn upper_roman_to_usize (roman_str: &str) -> Option<usize> {

    let mut num_val: usize = 0;
    let mut buffer = String::with_capacity(2);
    let mut chars_iter = roman_str.chars().peekable();

    const ROMAN_MAX: usize = 4999;

    while let Some(c1) = chars_iter.next() {

      buffer.push(c1);

      match c1 {
        'C' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'M' || *c2 == 'D' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'X' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'C' || *c2 == 'L' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'I' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'X' || *c2 == 'V' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        _ => ()
      }


      // Convert the contents of the buffer to usize, if valid.
      let buf_str = buffer.as_str();

      match buf_str {
        "M"   => num_val += 1000,
        "CM"  => num_val += 900,
        "D"   => num_val += 500,
        "CD"  => num_val += 400,
        "C"   => num_val += 100,
        "XC"  => num_val += 90,
        "L"   => num_val += 50,
        "XL"  => num_val += 40,
        "X"   => num_val += 10,
        "IX"  => num_val += 9,
        "V"   => num_val += 5,
        "IV"  => num_val += 4,
        "I"   => num_val += 1,
        _     => {
          eprintln!("No match for supposed upper-case Roman numeral {}...\n", buf_str);
          return None
        }
      }

      if num_val > ROMAN_MAX {
        eprintln!("Roman numerals greater than {} not supported by reStructuredText\n", ROMAN_MAX);
        return None
      }

      buffer.clear();
    }

    Some(num_val)
  }


  /// ### lower_roman_to_usize
  /// Converts a valid given lower-case Roman numeral to a `Some(usize)`.
  /// If the numeral isn't valid, `None` is returned instead
  pub fn lower_roman_to_usize (roman_str: &str) -> Option<usize> {

    let mut num_val: usize = 0;
    let mut buffer = String::with_capacity(2);
    let mut chars_iter = roman_str.chars().peekable();

    const ROMAN_MAX: usize = 4999;

    while let Some(c1) = chars_iter.next() {

      buffer.push(c1);

      match c1 {
        'c' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'm' || *c2 == 'd' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'x' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'c' || *c2 == 'l' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        'i' => {
          match chars_iter.peek() {
            None                                        => (),
            Some(c2) if *c2 == 'x' || *c2 == 'v' => {
              buffer.push(*c2);
              chars_iter.next();
            }
            _                                           => ()
          }
        }
        _ => ()
      }


      // Convert the contents of the buffer to usize, if valid.
      let buf_str = buffer.as_str();

      match buf_str {
        "m"   => num_val += 1000,
        "cm"  => num_val += 900,
        "d"   => num_val += 500,
        "cd"  => num_val += 400,
        "c"   => num_val += 100,
        "xc"  => num_val += 90,
        "l"   => num_val += 50,
        "xl"  => num_val += 40,
        "x"   => num_val += 10,
        "ix"  => num_val += 9,
        "v"   => num_val += 5,
        "iv"  => num_val += 4,
        "i"   => num_val += 1,
        _     => {
          eprintln!("No match for supposed lower-case Roman numeral {}...\n", buf_str);
          return None
        }
      }

      if num_val > ROMAN_MAX {
        eprintln!("Roman numerals greater than {} not supported by reStructuredText\n", ROMAN_MAX);
        return None
      }

      buffer.clear();
    }

    Some(num_val)
  }

}


/// ### normalize_refname
/// Normalizes the given `&str` according to the reStructuredText specification.
/// In this context, normalization means converting any contiguous whitespace into
/// a single ASCII space character and converting letters to their lower-case variants.
/// 
/// #### Note
/// This might return something nonsensical, as converting a single multi-scalar grapheme
/// into lower-case will return the multiple constituent "characters" as their lower-case variants. 
pub fn normalize_refname (name: &str) -> String {
  name.split_whitespace() 
    .collect::<Vec<&str>>() // Collects the SplitWhiteSpace iterator into a vector of &strs
    .join(" ")            // Joins the vector of &strs into an allocated String
    .to_lowercase()                   // Performs a UTF8-compliant transformation of unicode scalars in the String
                                      // into their lower-case counterparts
}
