/// XOR cipher
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be ciphered.
/// * `key` - A u8 that holds the key to be used for ciphering.
///
/// # Returns
///
/// * A String that holds the ciphered text.
///
/// # Example
///
/// ```rust
/// use rust_algorithms::ciphers::xor;
///
/// let test_string = "The quick brown fox jumps over the lazy dog";
/// let ciphered_text = xor(test_string, 64);
///
/// assert_eq!(test_string, xor(&ciphered_text, 64));
/// ```
pub fn xor(text: &str, key: u8) -> String {
    text.chars().map(|c| ((c as u8) ^ key) as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let test_string = "test string";
        let ciphered_text = xor(test_string, 32);
        assert_eq!(test_string, xor(&ciphered_text, 32));
    }
}
