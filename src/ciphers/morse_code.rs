use std::collections::HashMap;

// The character used to represent an unknown morse code sequence
const UNKNOWN_CHARACTER: &str = "........";

// The character used to represent an unknown morse code character
const _UNKNOWN_MORSE_CHARACTER: &str = "_";

/// Encode a message into morse code.
///
/// Given a message, this function encodes it into morse code.
/// It uses a dictionary to map each character to its corresponding morse code sequence.
/// If a character is not found in the dictionary, it is replaced with the unknown character sequence.
///
/// # Arguments
///
/// * `message` - The message to encode into morse code.
///
/// # Returns
///
/// The encoded morse code as a string.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::ciphers::encode;
///
/// let message = "Hello Morse";
/// let cipher = encode(message);
///
/// assert_eq!(cipher, ".... . .-.. .-.. --- / -- --- .-. ... .");
/// ```
pub fn encode(message: &str) -> String {
    let dictionary = _morse_dictionary();
    message
        .chars()
        .into_iter()
        .map(|char| char.to_uppercase().to_string())
        .map(|letter| dictionary.get(letter.as_str()))
        .map(|option| option.unwrap_or(&UNKNOWN_CHARACTER).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// Declarative macro for creating readable map declarations, for more info see https://doc.rust-lang.org/book/ch19-06-macros.html
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        std::iter::Iterator::collect(IntoIterator::into_iter([$(($key, $value),)*]))
    };
}

/// Create the morse code to alphanumeric dictionary.
///
/// This function creates a HashMap that maps each morse code sequence to its corresponding alphanumeric character.
///
/// # Returns
///
/// The morse code to alphanumeric dictionary as a HashMap.
fn _morse_dictionary() -> HashMap<&'static str, &'static str> {
    map! {
        "A" => ".-",      "B" => "-...",    "C" => "-.-.",
        "D" => "-..",     "E" => ".",       "F" => "..-.",
        "G" => "--.",     "H" => "....",    "I" => "..",
        "J" => ".---",    "K" => "-.-",     "L" => ".-..",
        "M" => "--",      "N" => "-.",      "O" => "---",
        "P" => ".--.",    "Q" => "--.-",    "R" => ".-.",
        "S" => "...",     "T" => "-",       "U" => "..-",
        "V" => "...-",    "W" => ".--",     "X" => "-..-",
        "Y" => "-.--",    "Z" => "--..",

        "1" => ".----",   "2" => "..---",   "3" => "...--",
        "4" => "....-",   "5" => ".....",   "6" => "-....",
        "7" => "--...",   "8" => "---..",   "9" => "----.",
        "0" => "-----",

        "&" => ".-...",   "@" => ".--.-.",  ":" => "---...",
        "," => "--..--",  "." => ".-.-.-",  "'" => ".----.",
        "\"" => ".-..-.", "?" => "..--..",  "/" => "-..-.",
        "=" => "-...-",   "+" => ".-.-.",   "-" => "-....-",
        "(" => "-.--.",   ")" => "-.--.-",  " " => "/",
        "!" => "-.-.--",
    }
}

/// Create the morse code to alphanumeric dictionary.
///
/// This function creates a HashMap that maps each morse code sequence to its corresponding alphanumeric character.
///
/// # Returns
///
/// The morse code to alphanumeric dictionary as a HashMap.
fn _morse_to_alphanumeric_dictionary() -> HashMap<&'static str, &'static str> {
    map! {
        ".-"   =>  "A",      "-..." => "B",    "-.-." => "C",
        "-.."  =>  "D",      "."    => "E",       "..-." => "F",
        "--."  =>  "G",      "...." => "H",    ".." => "I",
        ".---" =>  "J",     "-.-" => "K",     ".-.." => "L",
        "--"   =>  "M",       "-." => "N",      "---" => "O",
        ".--." =>  "P",     "--.-" => "Q",    ".-." => "R",
        "..."  =>  "S",      "-" => "T",       "..-" => "U",
        "...-" =>  "V",     ".--" => "W",     "-..-" => "X",
        "-.--" =>  "Y",     "--.." => "Z",

        ".----" => "1",    "..---" => "2",   "...--" => "3",
        "....-" => "4",    "....." => "5",   "-...." => "6",
        "--..." => "7",    "---.." => "8",   "----." => "9",
        "-----" => "0",

        ".-..." => "&",    ".--.-." => "@",  "---..." => ":",
        "--..--" => ",",   ".-.-.-" => ".",  ".----." => "'",
        ".-..-." => "\"",  "..--.." => "?",  "-..-." => "/",
        "-...-" => "=",   ".-.-." => "+",   "-....-" => "-",
        "-.--." => "(",   "-.--.-" => ")",  "/" => " ",
        "-.-.--" => "!",  " " => " ",       "" => ""
    }
}

/// Check if a string is a valid morse code part.
///
/// This function checks if a string contains only valid morse code characters ('.', '-', and ' ').
///
/// # Arguments
///
/// * `string` - The string to check.
///
/// # Returns
///
/// `true` if the string is a valid morse code part, `false` otherwise.
fn _check_part(string: &str) -> bool {
    for c in string.chars() {
        match c {
            '.' | '-' | ' ' => (),
            _ => return false,
        }
    }
    true
}

/// Check if a string is a valid morse code.
///
/// This function checks if a string is a valid morse code by splitting it into parts and checking each part.
///
/// # Arguments
///
/// * `string` - The string to check.
///
/// # Returns
///
/// `true` if the string is a valid morse code, `false` otherwise.
fn _check_all_parts(string: &str) -> bool {
    string.split('/').all(_check_part)
}

/// Decode a morse code into an alphanumeric message.
///
/// Given a morse code, this function decodes it into an alphanumeric message.
/// It uses a dictionary to map each morse code sequence to its corresponding alphanumeric character.
/// If a morse code sequence is not found in the dictionary, it is replaced with the unknown morse code character.
/// If the morse code is invalid, an `InvalidData` error is returned.
///
/// # Arguments
///
/// * `string` - The morse code to decode into an alphanumeric message.
///
/// # Returns
///
/// The decoded alphanumeric message as a `Result` containing a `String` if successful, or an `InvalidData` error.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::ciphers::decode;
///
/// let message = decode(".... . .-.. .-.. --- / -- --- .-. ... .").unwrap();
///
/// assert_eq!(message, "HELLO MORSE");
/// ```
pub fn decode(string: &str) -> Result<String, std::io::Error> {
    if !_check_all_parts(string) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid morse code",
        ));
    }

    let mut partitions: Vec<String> = vec![];

    for part in string.split('/') {
        partitions.push(_decode_part(part));
    }

    Ok(partitions.join(" "))
}

/// Decode a morse code token into an alphanumeric character.
///
/// This function decodes a morse code token into its corresponding alphanumeric character.
/// It uses a dictionary to map each morse code sequence to its corresponding alphanumeric character.
/// If the morse code token is not found in the dictionary, it is replaced with the unknown morse code character.
///
/// # Arguments
///
/// * `string` - The morse code token to decode into an alphanumeric character.
///
/// # Returns
///
/// The decoded alphanumeric character as a string.
///
fn _decode_token(string: &str) -> String {
    _morse_to_alphanumeric_dictionary()
        .get(string)
        .unwrap_or(&_UNKNOWN_MORSE_CHARACTER)
        .to_string()
}

/// Decode a morse code part into an alphanumeric string.
///
/// This function decodes a morse code part into its corresponding alphanumeric string.
/// It splits the part into tokens, decodes each token, and joins them together.
///
/// # Arguments
///
/// * `string` - The morse code part to decode into an alphanumeric string.
///
/// # Returns
///
/// The decoded alphanumeric string.
fn _decode_part(string: &str) -> String {
    string
        .split(' ')
        .map(_decode_token)
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_only_letters() {
        let message = "Hello Morse";
        let cipher = encode(message);
        assert_eq!(
            cipher,
            ".... . .-.. .-.. --- / -- --- .-. ... .".to_string()
        )
    }

    #[test]
    fn encrypt_letters_and_special_characters() {
        let message = "What's a great day!";
        let cipher = encode(message);
        assert_eq!(
            cipher,
            ".-- .... .- - .----. ... / .- / --. .-. . .- - / -.. .- -.-- -.-.--".to_string()
        )
    }

    #[test]
    fn encrypt_message_with_unsupported_character() {
        let message = "Error?? {}";
        let cipher = encode(message);
        assert_eq!(
            cipher,
            ". .-. .-. --- .-. ..--.. ..--.. / ........ ........".to_string()
        )
    }

    #[test]
    fn decrypt_valid_morsecode_with_spaces() {
        let expected = "Hello Morse! How's it goin, \"eh\"?"
            .to_string()
            .to_uppercase();
        let encypted = encode(&expected);
        let result = decode(&encypted).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn decrypt_valid_character_set_invalid_morsecode() {
        let expected = format!(
            "{}{}{}{} {}",
            _UNKNOWN_MORSE_CHARACTER,
            _UNKNOWN_MORSE_CHARACTER,
            _UNKNOWN_MORSE_CHARACTER,
            _UNKNOWN_MORSE_CHARACTER,
            _UNKNOWN_MORSE_CHARACTER,
        );

        let encypted = ".-.-.--.-.-. --------. ..---.-.-. .-.-.--.-.-. / .-.-.--.-.-.".to_string();
        let result = decode(&encypted).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn decrypt_invalid_morsecode_with_spaces() {
        let encypted = "1... . .-.. .-.. --- / -- --- .-. ... .";
        let result = decode(encypted).map_err(|e| e.kind());
        let expected = Err(std::io::ErrorKind::InvalidData);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_decode() {
        let message = ".... . .-.. .-.. --- / -- --- .-. ... .";
        let cipher = decode(message).unwrap();
        assert_eq!(cipher, "HELLO MORSE");
    }
}
