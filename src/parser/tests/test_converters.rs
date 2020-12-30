/// ## test_converters
///
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use super::*;
use crate::parser::converters;

#[cfg(test)]
#[test]
fn upper_roman_to_usize_01() {
    let iii = "III";
    let iv = "IV";
    let mmmmcmxcix = "MMMMCMXCIX";
    let over_max = "MMMMCMXCX";

    let iii_as_u32 = match converters::upper_roman_to_usize(iii) {
        Some(num) => num,
        None => panic!("Couldn't convert the Roman numeral III to u32\n"),
    };
    let iv_as_u32 = match converters::upper_roman_to_usize(iv) {
        Some(num) => num,
        None => panic!("Couldn't convert the Roman numeral iv to u32\n"),
    };
    let mmmmcmxcix_as_u32 = match converters::upper_roman_to_usize(mmmmcmxcix) {
        Some(num) => num,
        None => panic!("Couldn't convert the Roman numeral MMMMCMXCIX to u32\n"),
    };
    let over_max_as_u32 = converters::upper_roman_to_usize(over_max);

    assert_eq!(3, iii_as_u32);
    assert_eq!(4, iv_as_u32);
    assert_eq!(4999, mmmmcmxcix_as_u32);
    assert_eq!(None, over_max_as_u32);
}

#[test]
fn lower_roman_to_usize_01() {
    let iii = "iii";
    let iv = "iv";
    let mmmmcmxcix = "mmmmcmxcix";
    let over_max = "mmmmcmxcx";

    let iii_as_u32 = match converters::lower_roman_to_usize(iii) {
        Some(num) => num,
        None => panic!("Couldn't convert the Roman numeral iii to u32\n"),
    };
    let iv_as_u32 = match converters::lower_roman_to_usize(iv) {
        Some(num) => num,
        None => panic!("Couldn't convert the Roman numeral iv to u32\n"),
    };
    let mmmmcmxcix_as_u32 = match converters::lower_roman_to_usize(mmmmcmxcix) {
        Some(num) => num,
        None => panic!("Couldn't convert the Roman numeral mmmmcmxcix to u32\n"),
    };
    let over_max_as_u32 = converters::lower_roman_to_usize(over_max);

    assert_eq!(3, iii_as_u32);
    assert_eq!(4, iv_as_u32);
    assert_eq!(4999, mmmmcmxcix_as_u32);
    assert_eq!(None, over_max_as_u32);
}
