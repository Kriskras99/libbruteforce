/// Latin Digits
pub static DIGITS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

/// Latin letters (upper case)
pub static UC_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

/// Latin letters (lower case)
pub static LC_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

/// Common german umlauts (upper case)
pub static UC_UMLAUTS: [char; 3] = [
    'Ä', 'Ö', 'Ü'
];

/// Common german umlauts (lower case)
pub static LC_UMLAUTS: [char; 3] = [
    'ä', 'ö', 'ü'
];

/// Common special chars on a qwertz keyboard
pub static COMMON_SPECIAL_CHARS: [char; 16] = [
    ' ', '!', '$', '%', '&', '/', '(', ')', '=', '?', '+', '#', '-', '.', ',', 'ß'
];

/// all special chars on a common qwertz keyboard
pub static ALL_SPECIAL_CHARS: [char; 26] = [
    '_', ':', ';', '<', '>', '|', '\'', '*', '~', '`', '´',
    '\\', '^', '°', '(', ')', '{', '}', '[', ']', '€', '@', '"', '§',
    '/', 'µ'
];

/// Shorthand for build_alphabet with all parameters set to true.
pub fn full_alphabet() -> Box<[char]> {
    build_alphabet(true,
                   true,
                   true,
                   true,
                   true,
                   true,
                   true
    )
}

/// Builds the alphabet from the given flags.
pub fn build_alphabet(lc_letters: bool,
                      up_letters: bool,
                      digits: bool,
                      lc_umlauts: bool,
                      uc_umlauts: bool,
                      common_special_chars: bool,
                      all_special_chars: bool) -> Box<[char]> {
    let mut symbols = vec![];
    if lc_letters {
        symbols.extend_from_slice(&LC_LETTERS);
    }
    if up_letters {
        symbols.extend_from_slice(&UC_LETTERS);
    }
    if digits {
        symbols.extend_from_slice(&DIGITS);
    }
    if lc_umlauts {
        symbols.extend_from_slice(&LC_UMLAUTS);
    }
    if uc_umlauts {
        symbols.extend_from_slice(&UC_UMLAUTS);
    }
    if common_special_chars {
        symbols.extend_from_slice(&COMMON_SPECIAL_CHARS);
    }
    if all_special_chars {
        symbols.extend_from_slice(&ALL_SPECIAL_CHARS);
    }
    symbols.into_boxed_slice()
}

/// Calculates the amount of possible permutations if
/// n symbols are given and m slots are available.
/// Be aware that this solutions counts in that a
/// password can be zero-length, one-length and so on.
pub fn combinations_count(alphabet: &Box<[char]>, length: u32) -> usize {
    let mut sum = 0;
    for i in 0..(length + 1) {
        sum += alphabet.len().pow(i);
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_alphabet() {
        let alphabet = build_alphabet(
            false, true, true, false,
            false, false, false
        );
        assert_eq!(alphabet.len(), 36);

        let alphabet = build_alphabet(
            true, true, true, true,
            true, true, true
        );
        assert_eq!(alphabet.len(), 10 + 26 + 26 + 3 + 3 + 16 + 26);
        assert_eq!(alphabet.len(), full_alphabet().len());
    }

    #[test]
    fn test_get_permutation_count() {
        let alphabet1: Box<[char]> = Box::from(['a']);
        let alphabet2: Box<[char]> = Box::from(['a', 'b', 'c']);
        let alphabet3: Box<[char]> = Box::from(['a', 'b']);
        assert_eq!(combinations_count(&alphabet1, 3), 4, "1 symbol and a maximum length of 3");
        assert_eq!(combinations_count(&alphabet2, 1), 4, "3 symbols and a maximum length of 1");
        assert_eq!(combinations_count(&alphabet3, 3), 15, "3 symbols and a maximum length of 3");
    }
}
