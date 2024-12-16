use bimap::BiHashMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Bidirectional map for keeping translations for the nisse language
    static ref TRANSLATIONS: BiHashMap<char, char> = {
        let mut elements: BiHashMap<char, char> = BiHashMap::new();
        elements.insert('a', 'd');
        elements.insert('b', 'e');
        elements.insert('c', 'f');
        elements.insert('d', 'g');
        elements.insert('e', 'h');
        elements.insert('f', 'i');
        elements.insert('g', 'j');
        elements.insert('h', 'k');
        elements.insert('i', 'l');
        elements.insert('j', 'm');
        elements.insert('k', 'n');
        elements.insert('l', 'o');
        elements.insert('m', 'p');
        elements.insert('n', 'q');
        elements.insert('o', 'r');
        elements.insert('p', 's');
        elements.insert('q', 't');
        elements.insert('r', 'u');
        elements.insert('s', 'v');
        elements.insert('t', 'w');
        elements.insert('u', 'x');
        elements.insert('v', 'y');
        elements.insert('w', 'z');
        elements.insert('x', 'æ');
        elements.insert('y', 'ø');
        elements.insert('z', 'å');
        elements.insert('æ', 'a');
        elements.insert('ø', 'b');
        elements.insert('å', 'c');
        elements.insert('A', 'D');
        elements.insert('B', 'E');
        elements.insert('C', 'F');
        elements.insert('D', 'G');
        elements.insert('E', 'H');
        elements.insert('F', 'I');
        elements.insert('G', 'J');
        elements.insert('H', 'K');
        elements.insert('I', 'L');
        elements.insert('J', 'M');
        elements.insert('K', 'N');
        elements.insert('L', 'O');
        elements.insert('M', 'P');
        elements.insert('N', 'Q');
        elements.insert('O', 'R');
        elements.insert('P', 'S');
        elements.insert('Q', 'T');
        elements.insert('R', 'U');
        elements.insert('S', 'V');
        elements.insert('T', 'W');
        elements.insert('U', 'X');
        elements.insert('V', 'Y');
        elements.insert('W', 'Z');
        elements.insert('X', 'Æ');
        elements.insert('Y', 'Ø');
        elements.insert('Z', 'Å');
        elements.insert('Æ', 'A');
        elements.insert('Ø', 'B');
        elements.insert('Å', 'C');

        elements
    };
}

/// Santa Translator for communicating in encrypted nisse language
fn main() {
    let encrypted_result = process_input("Senge", true).to_string();
    println!("{encrypted_result}");

    let decrypted_result = process_input(&encrypted_result, false);
    println!("{decrypted_result}");
}

/// Process the input from the user and select whether the input should be encrypted or decrypted
fn process_input(input: &str, should_encrypt: bool) -> String {
    let mut output: String = "".to_string();
    for line in input.lines() {
        for cha in line.chars() {
            if should_encrypt {
                output.push(encrypt(&cha));
            } else {
                output.push(decrypt(&cha));
            }
        }
    }

    return output;
}

/// If the key is found, return the corresponding value from left to right in the BiMap
fn encrypt(input: &char) -> char {
    match TRANSLATIONS.get_by_left(input) {
        Some(c) => return *c,
        None => return *input,
    }
}

/// If the key is found, return the corresponding value from right to left in the BiMap
fn decrypt(input: &char) -> char {
    match TRANSLATIONS.get_by_right(input) {
        Some(c) => return *c,
        None => return *input,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_input_encryption_lowercase_success() {
        assert_eq!(process_input(&"abc", true), "def");
    }

    #[test]
    fn process_input_encryption_uppercase_success() {
        assert_eq!(process_input(&"ABC", true), "DEF");
    }

    #[test]
    fn process_input_encryption_mixed_success() {
        assert_eq!(process_input(&"aBc", true), "dEf");
    }

    #[test]
    fn process_input_encryption_seletion_failed() {
        assert_ne!(process_input(&"abc", false), "def");
    }

    #[test]
    fn process_input_decryption_lowercase_success() {
        assert_eq!(process_input(&"def", false), "abc");
    }

    #[test]
    fn process_input_decryption_uppercase_success() {
        assert_eq!(process_input(&"DEF", false), "ABC");
    }

    #[test]
    fn process_input_decryption_mixed_success() {
        assert_eq!(process_input(&"dEf", false), "aBc");
    }

    #[test]
    fn process_input_decryption_seletion_failed() {
        assert_ne!(process_input(&"def", true), "abc");
    }

    #[test]
    fn test_char_encryption_lowercase() {
        assert_eq!(encrypt(&'a'), 'd');
    }

    #[test]
    fn test_char_wrong_encryption() {
        assert_ne!(encrypt(&'a'), 'a');
    }

    #[test]
    fn test_char_encryption_uppercase() {
        assert_eq!(encrypt(&'A'), 'D');
    }

    #[test]
    fn test_special_not_changed_encryption() {
        assert_eq!(encrypt(&'/'), '/');
    }

    #[test]
    fn test_whitespace_char_not_changed_encryption() {
        assert_eq!(encrypt(&' '), ' ');
    }

    #[test]
    fn test_char_decryption_lowercase_encryption() {
        assert_eq!(decrypt(&'d'), 'a');
    }

    #[test]
    fn test_char_wrong_decryption() {
        assert_ne!(decrypt(&'a'), 'a');
    }

    #[test]
    fn test_char_decryption_uppercase() {
        assert_eq!(decrypt(&'D'), 'A');
    }

    #[test]
    fn test_special_not_changed_decryption() {
        assert_eq!(decrypt(&'/'), '/');
    }

    #[test]
    fn test_whitespace_char_not_changed_decryption() {
        assert_eq!(decrypt(&' '), ' ');
    }
}
