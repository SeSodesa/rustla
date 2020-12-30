/*!
A submodule for converter functions related to the parser.
These include Roman numeral -> integer transofrmations and the like.

(c) Santtu Söderholm <santtu.soderholm@tuni.fi>
*/
use crate::common::{EnumDelims, EnumKind, HTMLAlignment, HorizontalAlignment, Length, LengthNum};
use crate::parser::Parser;

impl Parser {

    /// Converts an enumerator &str to an integer using one of the
    /// coverters, if possible.
    pub fn enum_str_to_int_and_kind(
        detected_enum_str: &str,
        detected_kind: &EnumKind,
        list_kind: &EnumKind,
        in_list_item: bool,
        list_item_number: Option<usize>,
        list_start_index: Option<usize>,
    ) -> Option<(usize, EnumKind)> {
        let list_item_number = list_item_number.unwrap_or(0);
        let list_start_index = list_start_index.unwrap_or(1);

        if detected_enum_str == "i" && list_item_number == 0 {
            // LowerRoman list at our hands
            return Some((1, EnumKind::LowerRoman));
        } else if detected_enum_str == "I" && list_item_number == 0 {
            // UpperRoman list at our hands
            return Some((1, EnumKind::UpperRoman));
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
                    // eprintln!("Couldn't convert given alphabet to an integer...\n");
                    return None;
                }
            }

            EnumKind::LowerRoman => {
                if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
                    num
                } else {
                    // eprintln!("Couldn't convert lower-case Roman numeral to an integer...");
                    return None;
                }
            }

            EnumKind::UpperRoman => {
                if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
                    num
                } else {
                    // eprintln!("Couldn't convert upper-case Roman numeral to an integer...");
                    return None;
                }
            }

            EnumKind::Automatic => {
                if list_item_number == 0 && !in_list_item {
                    // eprintln!("No items in list yet.\nSetting enumerator kind to Arabic...");
                    detected_kind = EnumKind::Arabic;
                } else {
                    detected_kind = list_kind;
                }

                list_item_number + list_start_index
            }
        };

        Some((detected_enum_as_usize, detected_kind))
    }

    /// Transforms a given `regex::Captures` instance into an `Option`-wrapped integer--EnumKind--EnumDelims triple,
    /// assuming the captures are of the form found in `crate::parser::regex_patterns::ENUMERATOR_PATTERN`.
    /// If the conversion is not succssful, returns `None`.
    pub fn enum_captures_to_int_kind_and_delims(
        captures: &regex::Captures,
        list_kind: Option<&EnumKind>,
        in_list_item: bool,
        list_item_number: Option<usize>,
        list_start_index: Option<usize>,
    ) -> Option<(usize, EnumKind, EnumDelims)> {
        let list_item_number = list_item_number.unwrap_or(0);
        let list_start_index = list_start_index.unwrap_or(1);

        let (opt_number, enum_kind, enum_delims) = if let Some(number_str) =
            captures.name("arabic_parens")
        {
            let number = match number_str.as_str().parse::<usize>() {
                Ok(int) => Some(int),
                Err(e) => None,
            };

            (number, EnumKind::Arabic, EnumDelims::Parens)
        } else if let Some(number_str) = captures.name("lower_roman_parens") {
            if number_str.as_str() == "i" && list_item_number == 0 {
                return Some((1, EnumKind::LowerRoman, EnumDelims::Parens));
            }
            let number = Parser::lower_roman_to_usize(number_str.as_str());
            (number, EnumKind::LowerRoman, EnumDelims::Parens)
        } else if let Some(number_str) = captures.name("upper_roman_parens") {
            if number_str.as_str() == "I" && list_item_number == 0 {
                return Some((1, EnumKind::UpperRoman, EnumDelims::Parens));
            }
            let number = Parser::upper_roman_to_usize(number_str.as_str());
            (number, EnumKind::UpperRoman, EnumDelims::Parens)
        } else if let Some(number_str) = captures.name("lower_alpha_parens") {
            let number = Parser::alpha_to_usize(number_str.as_str());
            (number, EnumKind::LowerAlpha, EnumDelims::Parens)
        } else if let Some(number_str) = captures.name("upper_alpha_parens") {
            let number = Parser::alpha_to_usize(number_str.as_str());
            (number, EnumKind::UpperAlpha, EnumDelims::Parens)
        } else if let Some(number_str) = captures.name("auto_enumerator_parens") {
            let number = list_item_number.checked_add(list_start_index);
            let kind = if list_item_number == 0 && !in_list_item {
                EnumKind::Arabic
            } else if let Some(kind) = list_kind {
                *kind
            } else {
                EnumKind::Arabic
            };
            (number, kind, EnumDelims::Parens)
        } else if let Some(number_str) = captures.name("arabic_rparen") {
            let number = match number_str.as_str().parse::<usize>() {
                Ok(int) => Some(int),
                Err(e) => None,
            };

            (number, EnumKind::Arabic, EnumDelims::RParen)
        } else if let Some(number_str) = captures.name("lower_roman_rparen") {
            if number_str.as_str() == "i" && list_item_number == 0 {
                return Some((1, EnumKind::LowerRoman, EnumDelims::RParen));
            }
            let number = Parser::lower_roman_to_usize(number_str.as_str());
            (number, EnumKind::LowerRoman, EnumDelims::RParen)
        } else if let Some(number_str) = captures.name("upper_roman_rparen") {
            if number_str.as_str() == "i" && list_item_number == 0 {
                return Some((1, EnumKind::UpperRoman, EnumDelims::RParen));
            }
            let number = Parser::upper_roman_to_usize(number_str.as_str());
            (number, EnumKind::UpperRoman, EnumDelims::RParen)
        } else if let Some(number_str) = captures.name("lower_alpha_rparen") {
            let number = Parser::alpha_to_usize(number_str.as_str());
            (number, EnumKind::LowerAlpha, EnumDelims::RParen)
        } else if let Some(number_str) = captures.name("upper_alpha_rparen") {
            let number = Parser::alpha_to_usize(number_str.as_str());
            (number, EnumKind::UpperAlpha, EnumDelims::RParen)
        } else if let Some(number_str) = captures.name("auto_enumerator_rparen") {
            let number = list_item_number.checked_add(list_start_index);
            let kind = if list_item_number == 0 && !in_list_item {
                EnumKind::Arabic
            } else if let Some(kind) = list_kind {
                *kind
            } else {
                EnumKind::Arabic
            };
            (number, kind, EnumDelims::RParen)
        } else if let Some(number_str) = captures.name("arabic_period") {
            let number = match number_str.as_str().parse::<usize>() {
                Ok(int) => Some(int),
                Err(e) => None,
            };

            (number, EnumKind::Arabic, EnumDelims::Period)
        } else if let Some(number_str) = captures.name("lower_roman_period") {
            if number_str.as_str() == "i" && list_item_number == 0 {
                return Some((1, EnumKind::LowerRoman, EnumDelims::Period));
            }
            let number = Parser::lower_roman_to_usize(number_str.as_str());
            (number, EnumKind::LowerRoman, EnumDelims::Period)
        } else if let Some(number_str) = captures.name("upper_roman_period") {
            if number_str.as_str() == "i" && list_item_number == 0 {
                return Some((1, EnumKind::UpperRoman, EnumDelims::Period));
            }
            let number = Parser::upper_roman_to_usize(number_str.as_str());
            (number, EnumKind::UpperRoman, EnumDelims::Period)
        } else if let Some(number_str) = captures.name("lower_alpha_period") {
            let number = Parser::alpha_to_usize(number_str.as_str());
            (number, EnumKind::LowerAlpha, EnumDelims::Period)
        } else if let Some(number_str) = captures.name("upper_alpha_period") {
            let number = Parser::alpha_to_usize(number_str.as_str());
            (number, EnumKind::UpperAlpha, EnumDelims::Period)
        } else if let Some(number_str) = captures.name("auto_enumerator_period") {
            let number = list_item_number.checked_add(list_start_index);
            let kind = if list_item_number == 0 && !in_list_item {
                EnumKind::Arabic
            } else if let Some(kind) = list_kind {
                *kind
            } else {
                EnumKind::Arabic
            };
            (number, kind, EnumDelims::Period)
        } else {
            panic!("Tried converting a set of regex captures \"{}\" into a list enumerator, but captured string was not of the form specified by enumerator pattern. Computer says no...", captures.get(0).unwrap().as_str());
        };

        if opt_number.is_none() {
            return None;
        }

        Some((opt_number.unwrap(), enum_kind, enum_delims))
    }

    /// Converts and ASCII letter to a corresponding `Option`al integer between 1--26 inclusive.
    /// Returns `None` if not successful.
    pub fn alpha_to_usize(alpha_str: &str) -> Option<usize> {
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
            _ => {
                eprintln!(
                    "Error: Letter '{}' not recognized as integer by reStructuredText...\n",
                    alpha_str
                );
                None
            }
        }
    }

    /// Converts a valid given upper-case Roman numeral to a `Some(usize)`.
    /// If the numeral isn't valid, `None` is returned instead
    pub fn upper_roman_to_usize(roman_str: &str) -> Option<usize> {
        let mut num_val: usize = 0;
        let mut buffer = String::with_capacity(2);
        let mut chars_iter = roman_str.chars().peekable();

        const ROMAN_MAX: usize = 4999;

        while let Some(c1) = chars_iter.next() {
            buffer.push(c1);

            match c1 {
                'C' => match chars_iter.peek() {
                    None => (),
                    Some(c2) if *c2 == 'M' || *c2 == 'D' => {
                        buffer.push(*c2);
                        chars_iter.next();
                    }
                    _ => (),
                },
                'X' => match chars_iter.peek() {
                    None => (),
                    Some(c2) if *c2 == 'C' || *c2 == 'L' => {
                        buffer.push(*c2);
                        chars_iter.next();
                    }
                    _ => (),
                },
                'I' => match chars_iter.peek() {
                    None => (),
                    Some(c2) if *c2 == 'X' || *c2 == 'V' => {
                        buffer.push(*c2);
                        chars_iter.next();
                    }
                    _ => (),
                },
                _ => (),
            }

            // Convert the contents of the buffer to usize, if valid.
            match buffer.as_str() {
                "M" => num_val += 1000,
                "CM" => num_val += 900,
                "D" => num_val += 500,
                "CD" => num_val += 400,
                "C" => num_val += 100,
                "XC" => num_val += 90,
                "L" => num_val += 50,
                "XL" => num_val += 40,
                "X" => num_val += 10,
                "IX" => num_val += 9,
                "V" => num_val += 5,
                "IV" => num_val += 4,
                "I" => num_val += 1,
                _ => {
                    eprintln!(
                        "No match for supposed upper-case Roman numeral {}...\n",
                        buffer.as_str()
                    );
                    return None;
                }
            }

            if num_val > ROMAN_MAX {
                eprintln!(
                    "Roman numerals greater than {} not supported by reStructuredText\n",
                    ROMAN_MAX
                );
                return None;
            }

            buffer.clear();
        }

        Some(num_val)
    }

    /// Converts a valid given lower-case Roman numeral to a `Some(usize)`.
    /// If the numeral isn't valid, `None` is returned instead
    pub fn lower_roman_to_usize(roman_str: &str) -> Option<usize> {
        let mut num_val: usize = 0;
        let mut buffer = String::with_capacity(2);
        let mut chars_iter = roman_str.chars().peekable();

        const ROMAN_MAX: usize = 4999;

        while let Some(c1) = chars_iter.next() {
            buffer.push(c1);

            match c1 {
                'c' => match chars_iter.peek() {
                    None => (),
                    Some(c2) if *c2 == 'm' || *c2 == 'd' => {
                        buffer.push(*c2);
                        chars_iter.next();
                    }
                    _ => (),
                },
                'x' => match chars_iter.peek() {
                    None => (),
                    Some(c2) if *c2 == 'c' || *c2 == 'l' => {
                        buffer.push(*c2);
                        chars_iter.next();
                    }
                    _ => (),
                },
                'i' => match chars_iter.peek() {
                    None => (),
                    Some(c2) if *c2 == 'x' || *c2 == 'v' => {
                        buffer.push(*c2);
                        chars_iter.next();
                    }
                    _ => (),
                },
                _ => (),
            }

            // Convert the contents of the buffer to usize, if valid.
            match buffer.as_str() {
                "m" => num_val += 1000,
                "cm" => num_val += 900,
                "d" => num_val += 500,
                "cd" => num_val += 400,
                "c" => num_val += 100,
                "xc" => num_val += 90,
                "l" => num_val += 50,
                "xl" => num_val += 40,
                "x" => num_val += 10,
                "ix" => num_val += 9,
                "v" => num_val += 5,
                "iv" => num_val += 4,
                "i" => num_val += 1,
                _ => {
                    eprintln!(
                        "No match for supposed lower-case Roman numeral {}...\n",
                        buffer.as_str()
                    );
                    return None;
                }
            }

            if num_val > ROMAN_MAX {
                eprintln!(
                    "Roman numerals greater than {} not supported by reStructuredText\n",
                    ROMAN_MAX
                );
                return None;
            }

            buffer.clear();
        }

        Some(num_val)
    }

    /// Converts a given string into a `Length` enum variant, if possible.
    /// If conversion succeeds, returns `Some(Length)`, else returns `None`.
    pub fn str_to_length(length_str: &str) -> Option<Length> {

        const VALID_LENGTH_PATTERN: &str =
            r#"^(?P<number>[0-9]+(?:[.][0-9]*)?|[.][0-9]+)(?P<unit>em|ex|mm|cm|in|px|pt|pc)$"#;

        lazy_static::lazy_static! {
            static ref VALID_LENGTH_RE: regex::Regex = regex::Regex::new(VALID_LENGTH_PATTERN).unwrap();
        }

        let captures = if let Some(capts) = VALID_LENGTH_RE.captures(length_str.trim()) {
            capts
        } else {
            return None;
        };

        let number: LengthNum = if let Some(num) = captures.name("number") {
            if let Ok(result) = num.as_str().parse() {
                result
            } else {
                return None;
            }
        } else {
            return None;
        };

        let length_unit = if let Some(unit) = captures.name("unit") {
            match unit.as_str() {
                "em" => Length::Em(number),
                "ex" => Length::Ex(number),
                "mm" => Length::Mm(number),
                "cm" => Length::Cm(number),
                "in" => Length::In(number),
                "px" => Length::Px(number),
                "pt" => Length::Pt(number),
                "pc" => Length::Pc(number),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(length_unit)
    }

    /// Converts a given string to a positive percentage (`Option<f64>`),
    /// if it can be intrepreted as such. Else returns `None`
    pub fn str_to_percentage(percentage_str: &str) -> Option<f64> {

        const VALID_PERCENTAGE_PATTERN: &str =
            r#"^(?P<number>[0-9]+(?:[.][0-9]*)?|[.][0-9]+)\s*(?P<unit>%)$"#;

        lazy_static::lazy_static! {
            static ref VALID_PERCENTAGE_RE: regex::Regex = regex::Regex::new(VALID_PERCENTAGE_PATTERN).unwrap();
        }

        let captures = if let Some(capts) = VALID_PERCENTAGE_RE.captures(percentage_str.trim()) {
            capts
        } else {
            return None;
        };

        let percentage = if let Some(num) = captures.name("number") {
            if let Ok(result) = num.as_str().parse::<f64>() {
                result
            } else {
                return None;
            }
        } else {
            return None;
        };

        Some(percentage)
    }

    /// Converts a given `&str` to a suitable HTMLAlignment variant, if possible.
    /// Successful conversion results in a `Some(HTMLAlignment)`.
    /// Else `None` is returned.
    pub fn str_to_html_alignment(alignment_str: &str) -> Option<HTMLAlignment> {
        match alignment_str.trim() {
            "top" => Some(HTMLAlignment::Top),
            "middle" => Some(HTMLAlignment::Middle),
            "bottom" => Some(HTMLAlignment::Bottom),
            "left" => Some(HTMLAlignment::Left),
            "center" => Some(HTMLAlignment::Center),
            "right" => Some(HTMLAlignment::Right),
            _ => None,
        }
    }

    /// Converts a given `&str` to a suitable HTMLAlignment variant, if possible.
    /// Successful conversion results in a `Some(HTMLAlignment)`.
    /// Else `None` is returned.
    pub fn str_to_horizontal_alignment(alignment_str: &str) -> Option<HorizontalAlignment> {
        match alignment_str.trim() {
            "left" => Some(HorizontalAlignment::Left),
            "center" => Some(HorizontalAlignment::Center),
            "right" => Some(HorizontalAlignment::Right),
            _ => None,
        }
    }
}
