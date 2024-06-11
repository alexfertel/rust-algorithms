/// A Vigenère cipher is a method of encrypting alphabetic text by using a simple form of polyalphabetic substitution.
/// Each letter in the plain_text text is shifted by the corresponding letter in the key.
///
/// # Algorithm
///
/// Rotate each ascii character by the offset of the corresponding key character.
/// When we reach the last key character, we start over from the first one.
/// This implementation does not rotate unicode characters.
///
/// # Reference
///
/// [Vigenère Cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher).
///
/// # Arguments
///
/// * `plain_text` - A string slice that holds the text to be encrypted.
/// * `key` - A string slice that holds the key to be used for encryption.
///
/// # Returns
///
/// An owned String that holds the encrypted text.
///
/// # Example
///
/// ```rust
/// use rust_algorithms::ciphers::vigenere;
///
/// let plain_text = "LoremIpsumDolorSitAmet";
/// let key = "base";
///
/// let encrypted = vigenere(plain_text, key);
///
/// assert_eq!(encrypted, "MojinIhwvmVsmojWjtSqft");
/// ```
pub fn vigenere(plain_text: &str, key: &str) -> String {
    // Remove all unicode and non-ascii characters from key.
    let key: String = key.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
    let key = key.to_ascii_lowercase();

    let key_len = key.len();
    if key_len == 0 {
        return String::from(plain_text);
    }

    let mut index = 0;

    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';
                index += 1;
                // Modulo the distance to keep character range.
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(vigenere("", "test"), "");
    }

    #[test]
    fn vigenere_with_spaces() {
        assert_eq!(
            vigenere(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "spaces"
            ),
            "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx."
        );
    }

    #[test]
    fn vigenere_unicode_and_numbers() {
        assert_eq!(
            vigenere("1 Lorem ⏳ ipsum dolor sit amet Ѡ", "unicode"),
            "1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ"
        );
    }

    #[test]
    fn vigenere_unicode_key() {
        assert_eq!(
            vigenere("Lorem ipsum dolor sit amet", "😉 key!"),
            "Vspoq gzwsw hmvsp cmr kqcd"
        );
    }

    #[test]
    fn vigenere_empty_key() {
        assert_eq!(vigenere("Lorem ipsum", ""), "Lorem ipsum");
    }
}
