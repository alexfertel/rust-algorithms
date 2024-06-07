/// Applies the ROT13 cipher to the given text.
///
/// ROT13 is a simple letter substitution cipher that replaces a letter with the 13th letter after it in the alphabet.
/// The cipher wraps around, so 'Z' becomes 'M' and 'z' becomes 'm'.
/// Non-alphabetic characters are left unchanged.
///
/// # Arguments
///
/// * `text` - The text to be encrypted using ROT13 cipher.
///
/// # Returns
///
/// The encrypted text.
///
///  # Example
///
/// ```rust
/// use rust_algorithms::ciphers::another_rot13;
///
/// let text = "ABCzyx";
/// let encrypted = another_rot13(text);
///
/// assert_eq!(encrypted, "NOPmlk");
/// ```
///
pub fn another_rot13(text: &str) -> String {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let output = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm";
    text.chars()
        .map(|c| match input.find(c) {
            Some(i) => output.chars().nth(i).unwrap(),
            None => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(another_rot13("ABCzyx"), "NOPmlk");
    }

    #[test]
    fn test_every_alphabet_with_space() {
        assert_eq!(
            another_rot13("The quick brown fox jumps over the lazy dog"),
            "Gur dhvpx oebja sbk whzcf bire gur ynml qbt"
        );
    }

    #[test]
    fn test_non_alphabet() {
        assert_eq!(another_rot13("🎃 Jack-o'-lantern"), "🎃 Wnpx-b'-ynagrea");
    }
}
