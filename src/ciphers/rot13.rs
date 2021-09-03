//! Rot13 or "rotate by 13 places"
//!
//! # Algorithm

/// Encrypts a given [`&str`] using ROT13 cipher.
///
/// See [ROT13](https://en.wikipedia.org/wiki/ROT13) for more information.
///
/// Replaces each character with the 13th letter after it in the alphabet.
/// Rot13 is a special case of [Caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher)
///
/// The most basic example is ROT 13, which rotates 'a' to 'n'.
/// This implementation does not rotate unicode characters.
///
/// # Arguments
///
/// `text` - String to transform.
///
/// # Returns
///
/// An owned [`String`]
///
/// # Panic
///
/// This function won't panic.
///
/// # Examples
/// ```
/// # use rust_algorithms::ciphers::rot13;
/// let encoded = rot13("hello world");
/// assert_eq!(encoded, "URYYB JBEYQ");
/// ```
pub fn rot13(text: &str) -> String {
    let to_enc = text.to_uppercase();
    to_enc
        .chars()
        .map(|c| match c {
            'A'..='M' => ((c as u8) + 13) as char,
            'N'..='Z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::rot13;

    #[test]
    fn test_single_letter() {
        assert_eq!("N", rot13("A"));
    }

    #[test]
    fn test_bunch_of_letters() {
        assert_eq!("NOP", rot13("ABC"));
    }

    #[test]
    fn test_non_ascii() {
        assert_eq!("😀NO", rot13("😀AB"));
    }

    #[test]
    fn test_twice() {
        assert_eq!("ABCD", rot13(&rot13("ABCD")));
    }
}
