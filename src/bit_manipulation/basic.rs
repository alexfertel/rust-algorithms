//! This module provides basic bit manipulation operations.
//!
//! The following functions are available:
//!
//! - `get_bit`: Gets the value of a specific bit in a number.
//! - `set_bit`: Sets a specific bit in a number to 1.
//! - `clear_bit`: Sets a specific bit in a number to 0.
//! - `update_bit`: Updates a specific bit in a number based on a boolean value.
//! - `is_even`: Checks if a number is even.
//! - `is_positive`: Checks if a number is positive.
//! - `multiply_by_two`: Multiplies a number by two.
//! - `divide_by_two`: Divides a number by two.
//! - `twos_complement`: Calculates the two's complement of a number.
//! - `multiply_signed`: Multiplies two signed numbers.
//! - `multiply_unsigned`: Multiplies two unsigned numbers.
//! - `count_ones`: Counts the number of ones in a number.
//! - `bit_equivalence`: Counts the number of equal bits between two numbers.
//! - `bit_distance`: Calculates the bit distance between two numbers.
//! - `is_power_of_two`: Checks if a number is a power of two.
//! - `is_power_of_two_difference`: Checks if a number is a power of two using a different approach.
//! - `rightmost_one`: Returns the position of the rightmost one-bit in a number.
//! - `rightmost_zero`: Returns the position of the rightmost zero-bit in a number.
//!
//! For more information on each function, please refer to their individual documentation.

/// Gets specific bits from a number.
/// 
/// Returns the value of the bit at position `n` in `bits`.
/// 
/// see: [Get Bit](https://en.wikipedia.org/wiki/Bit_manipulation)
/// 
/// # Arguments
/// 
/// `bits` - The number to extract the bit from.
/// `n` - The position of the bit to extract.
/// 
/// # Returns
/// 
/// The value of the bit at position `n` in `bits`.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::get_bit;
/// 
/// let bits = 0b0101_0101;
/// 
/// assert_eq!(0, get_bit(bits, 7));
/// assert_eq!(1, get_bit(bits, 6));
/// assert_eq!(1, get_bit(bits, 0));
/// 
/// ```
///
pub fn get_bit(bits: i8, n: usize) -> i8 {
    (bits >> n) & 1
}

/// Sets a specific bit in a number.
///
/// Sets the bit at position `n` in `bits` to 1.
///
/// see: [Set Bit](https://en.wikipedia.org/wiki/Bit_manipulation)
///
/// # Arguments
///
/// `bits` - The number to set the bit in.
/// `n` - The position of the bit to set.
///
/// # Returns
///
/// The number with the bit at position `n` set to 1.
///
/// # Panic
///
/// This function will not panic.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::bit_manipulation::set_bit;
///
/// let bits = 0b0101_0100;
///
/// assert_eq!(0b0101_0101, set_bit(bits, 0));
/// assert_eq!(0b0101_0110, set_bit(bits, 1));
/// assert_eq!(0b0101_1100, set_bit(bits, 3));
///
/// ```
///
pub fn set_bit(bits: i8, n: usize) -> i8 {
    bits | (1 << n)
}

/// Clears a specific bit in a number.
///
/// Sets the bit at position `n` in `bits` to 0.
///
/// see: [Clear Bit](https://en.wikipedia.org/wiki/Bit_manipulation)
///
/// # Arguments
///
/// `bits` - The number to clear the bit in.
/// `n` - The position of the bit to clear.
///
/// # Returns
///
/// The number with the bit at position `n` set to 0.
///
/// # Panic
///
/// This function will not panic.
///
/// # Examples
///
/// ```rust
/// use rust_algorithms::bit_manipulation::clear_bit;
///
/// let bits = 0b0101_0101;
///
/// assert_eq!(0b0101_0100, clear_bit(bits, 0));
/// assert_eq!(0b0101_0001, clear_bit(bits, 2));
///
/// ```
///
pub fn clear_bit(bits: i8, n: usize) -> i8 {
    bits & !(1 << n)
}

/// Updates a specific bit in a number.
/// 
/// Sets the bit at position `n` in `bits` to 1 if `set_it` is true, otherwise sets it to 0.
/// 
/// see: [Update Bit](https://en.wikipedia.org/wiki/Bit_manipulation)
/// 
/// # Arguments
/// 
/// `bits` - The number to update the bit in.
/// `n` - The position of the bit to update.
/// `set_it` - If true, the bit will be set to 1, otherwise it will be set to 0.
/// 
/// # Returns
/// 
/// The number with the bit at position `n` set to 1 if `set_it` is true, otherwise set to 0.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::update_bit;
/// 
/// let bits = 0b0101_0101;
/// 
/// // Clears a bit.
/// assert_eq!(0b0101_0100, update_bit(bits, 0, false));
/// assert_eq!(0b0101_0001, update_bit(bits, 2, false));
/// 
/// // Sets a bit.
/// assert_eq!(0b0101_0111, update_bit(bits, 1, true));
/// assert_eq!(0b0101_1101, update_bit(bits, 3, true));
/// 
/// // Does nothing when setting a set bit.
/// assert_eq!(bits, update_bit(bits, 0, true));
/// assert_eq!(bits, update_bit(bits, 2, true));
/// 
/// // Does nothing when clearing an unset bit.
/// assert_eq!(bits, update_bit(bits, 1, false));
/// assert_eq!(bits, update_bit(bits, 3, false));
/// 
/// ```
/// 
pub fn update_bit(bits: i8, n: usize, set_it: bool) -> i8 {
    if set_it {
        set_bit(bits, n)
    } else {
        clear_bit(bits, n)
    }
}

/// Checks if a number is even.
/// 
/// Returns true if the least significant bit of `bits` is 0, otherwise false.
/// 
/// see: [Parity](https://en.wikipedia.org/wiki/Parity_(mathematics))
/// 
/// # Arguments
/// 
/// `bits` - The number to check.
/// 
/// # Returns
/// 
/// True if the least significant bit of `bits` is 0, otherwise false.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::is_even;
/// 
/// assert!(is_even(2));
/// assert!(is_even(0));
/// assert!(is_even(6));
/// assert!(is_even(36));
/// 
/// assert!(!is_even(33));
/// assert!(!is_even(1));
/// assert!(!is_even(17));
/// assert!(!is_even(127));
/// 
/// ```
/// 
pub fn is_even(bits: i8) -> bool {
    bool::from(bits & 1 == 0)
}

/// Checks if a number is positive.
/// 
/// Returns true if the most significant bit of `bits` is 0, otherwise false.
/// 
/// see: [Sign Bit](https://en.wikipedia.org/wiki/Sign_bit)
/// 
/// # Arguments
/// 
/// `bits` - The number to check.
/// 
/// # Returns
/// 
/// True if the most significant bit of `bits` is 0, otherwise false.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::is_positive;
/// 
/// assert!(is_positive(5));
/// assert!(is_positive(1));
/// assert!(is_positive(100));
/// assert!(is_positive(127));
/// 
/// assert!(!is_positive(-1));
/// assert!(!is_positive(-45));
/// assert!(!is_positive(-128));
/// assert!(!is_positive(0));
/// 
/// ```
/// 
pub fn is_positive(bits: i8) -> bool {
    if bits == 0 {
        return false;
    }

    get_bit(bits, 7) == 0
}

/// Multiplies a number by two.
/// 
/// Shifts the bits of `bits` one position to the left.
/// 
/// see: [Bitwise Shift](https://en.wikipedia.org/wiki/Bitwise_operation#Bit_shifts)
/// 
/// # Arguments
/// 
/// `bits` - The number to multiply.
/// 
/// # Returns
/// 
/// The result of shifting the bits of `bits` one position to the left.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::multiply_by_two;
/// 
/// assert_eq!(4, multiply_by_two(2));
/// assert_eq!(12, multiply_by_two(6));
/// assert_eq!(0, multiply_by_two(0));
/// assert_eq!(2, multiply_by_two(1));
/// 
/// ```
/// 
pub fn multiply_by_two(bits: i8) -> i8 {
    bits << 1
}

/// Divides a number by two.
/// 
/// Shifts the bits of `bits` one position to the right.
/// 
/// see: [Bitwise Shift](https://en.wikipedia.org/wiki/Bitwise_operation#Bit_shifts)
/// 
/// # Arguments
/// 
/// `bits` - The number to divide.
/// 
/// # Returns
/// 
/// The result of shifting the bits of `bits` one position to the right.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::divide_by_two;
/// 
/// assert_eq!(2, divide_by_two(4));
/// assert_eq!(12, divide_by_two(24));
/// assert_eq!(0, divide_by_two(0));
/// assert_eq!(0, divide_by_two(1));
/// 
/// ```
/// 
pub fn divide_by_two(bits: i8) -> i8 {
    bits >> 1
}

/// Calculates the two's complement of a number.
/// 
/// Returns the two's complement of `bits`.
/// 
/// see: [Two's Complement](https://en.wikipedia.org/wiki/Two%27s_complement)
/// 
/// # Arguments
/// 
/// `bits` - The number to calculate the two's complement of.
/// 
/// # Returns
/// 
/// The two's complement of `bits`.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::twos_complement;
/// 
/// assert_eq!(-1, twos_complement(1));
/// assert_eq!(0b000_0000, twos_complement(0b000_0000));
/// assert_eq!(-127, twos_complement(127));
/// assert_eq!(5, twos_complement(twos_complement(5)));
/// assert_eq!(10, twos_complement(twos_complement(10)));
/// 
/// ```
/// 
pub fn twos_complement(bits: i8) -> i8 {
    (!bits).wrapping_add(1)
}

/// Multiplies two signed numbers.
/// 
/// Multiplies `a` by `b` using the Russian peasant algorithm, (also known as Egyptian multiplication, Ethiopian 
/// multiplication, Russian multiplication, peasant multiplication, or mediation and duplation)
/// 
/// see: [Ancient Egyptian multiplication](https://en.wikipedia.org/wiki/Ancient_Egyptian_multiplication)
/// 
/// # Arguments
/// 
/// `a` - The first number to multiply.
/// `b` - The second number to multiply.
/// 
/// # Returns
/// 
/// The result of multiplying `a` by `b`.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::multiply_signed;
/// 
/// assert_eq!(-12, multiply_signed(-6, 2));
/// assert_eq!(-8, multiply_signed(2, -4));
/// assert_eq!(-12, multiply_signed(2, -6));
/// assert_eq!(120, multiply_signed(30, 4));
/// assert_eq!(120, multiply_signed(-30, -4));
/// assert_eq!(36, multiply_signed(36, 1));
/// assert_eq!(36, multiply_signed(1, 36));
/// 
/// ```
/// 
pub fn multiply_signed(a: i8, b: i8) -> i8 {
    if a == 0 || b == 0 {
        return 0;
    }

    if is_even(b) {
        multiply_signed(multiply_by_two(a), divide_by_two(b))
    } else if is_positive(b) {
        multiply_signed(multiply_by_two(a), divide_by_two(b.wrapping_sub(1))).wrapping_add(a)
    } else {
        multiply_signed(multiply_by_two(a), divide_by_two(b.wrapping_add(1))).wrapping_sub(a)
    }
}

/// Multiplies two unsigned numbers.
/// 
/// Multiplies `a` by `b` using the Russian peasant algorithm, (also known as Egyptian multiplication, Ethiopian 
/// multiplication, Russian multiplication, peasant multiplication, or mediation and duplation)
/// 
/// see: [Ancient Egyptian multiplication](https://en.wikipedia.org/wiki/Ancient_Egyptian_multiplication)
/// 
/// # Arguments
/// 
/// `a` - The first number to multiply.
/// `b` - The second number to multiply.
/// 
/// # Returns
/// 
/// The result of multiplying `a` by `b`.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::multiply_unsigned;
/// 
/// assert_eq!(120, multiply_unsigned(30, 4));
/// assert_eq!(120, multiply_unsigned(4, 30));
/// assert_eq!(36, multiply_unsigned(36, 1));
/// assert_eq!(36, multiply_unsigned(1, 36));
/// assert_eq!(0, multiply_unsigned(1, 0));
/// assert_eq!(0, multiply_unsigned(0, 1));
/// assert_eq!(0, multiply_unsigned(0, 0));
/// assert_eq!(64, multiply_unsigned(32, 2));
/// 
/// ```
/// 
pub fn multiply_unsigned(a: i8, b: i8) -> i8 {
    let mut result = 0;
    for i in 0..7 {
        if get_bit(b, i) == 1 {
            result += a * (1 << i);
        }
    }
    result
}

/// Counts the number of ones in a number.
///
/// Returns the number of ones in `bits`.
/// 
/// see: [Hamming Weight](https://en.wikipedia.org/wiki/Hamming_weight)
/// 
/// # Arguments
/// 
/// `bits` - The number to count the ones in.
/// 
/// # Returns
/// 
/// The number of ones in `bits`.
/// 
/// # Panic
/// 
/// This function will not panic.
/// 
/// # Examples
/// 
/// ```rust
/// use rust_algorithms::bit_manipulation::count_ones;
/// 
/// assert_eq!(0, count_ones(0));
/// assert_eq!(1, count_ones(1));
/// assert_eq!(3, count_ones(0b0101_0100));
/// assert_eq!(1, count_ones(0b0000_0100));
/// assert_eq!(7, count_ones(0b0111_1111));
/// 
/// ```
/// 
pub fn count_ones(bits: i8) -> i8 {
    let mut result = 0;
    for i in 0..7 {
        result += (bits >> i) & 1;
    }
    result
}

/// Counts the number of equal bits.
pub fn bit_equivalence(a: i8, b: i8) -> i8 {
    count_ones(!(a ^ b))
}

pub fn bit_distance(a: i8, b: i8) -> i8 {
    count_ones(a ^ b)
}

// `x & (x - 1) == 0`
pub fn is_power_of_two(bits: i8) -> bool {
    bits & (bits.wrapping_sub(1)) == 0
}

// `((x | (x - 1)) + 1) & x == 0`
//
// Also `((x & -x) + x) & x == 0`
pub fn is_power_of_two_difference(bits: i8) -> bool {
    ((bits | (bits.saturating_sub(1)))
        .checked_add(1)
        .unwrap_or(0))
        & bits
        == 0
}

// Returns the rightmost 1-bit or 0 if none.
pub fn rightmost_one(bits: i8) -> i8 {
    bits & -bits
}

// Creates a word with a single 1-bit at the position of the rightmost
// 0-bit in the input, producing 0 if none.
pub fn rightmost_zero(bits: i8) -> i8 {
    !bits & (bits.checked_add(1).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        let bits = 0b0101_0101;

        assert_eq!(0, get_bit(bits, 7));
        assert_eq!(1, get_bit(bits, 6));
        assert_eq!(1, get_bit(bits, 0));
    }

    #[test]
    fn test_set_bit() {
        let bits = 0b101_0101;

        assert_eq!(0b101_0111, set_bit(bits, 1));
        assert_eq!(0b101_1101, set_bit(bits, 3));
    }

    #[test]
    fn test_clear_bit() {
        let bits = 0b101_0101;

        assert_eq!(0b101_0100, clear_bit(bits, 0));
        assert_eq!(0b101_0001, clear_bit(bits, 2));
    }

    #[test]
    fn test_update_bit() {
        let bits = 0b101_0101;

        // Clears a bit.
        assert_eq!(0b101_0100, update_bit(bits, 0, false));
        assert_eq!(0b101_0001, update_bit(bits, 2, false));

        // Sets a bit.
        assert_eq!(0b101_0111, update_bit(bits, 1, true));
        assert_eq!(0b101_1101, update_bit(bits, 3, true));

        // Does nothing when setting a set bit.
        assert_eq!(bits, update_bit(bits, 0, true));
        assert_eq!(bits, update_bit(bits, 2, true));

        // Does nothing when clearing an unset bit.
        assert_eq!(bits, update_bit(bits, 1, false));
        assert_eq!(bits, update_bit(bits, 3, false));
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(6));
        assert!(is_even(36));

        assert!(!is_even(33));
        assert!(!is_even(1));
        assert!(!is_even(17));
        assert!(!is_even(127));
    }

    #[test]
    fn test_is_positive() {
        assert!(is_positive(5));
        assert!(is_positive(1));
        assert!(is_positive(100));
        assert!(is_positive(127));

        assert!(!is_positive(-1));
        assert!(!is_positive(-45));
        assert!(!is_positive(-128));
        assert!(!is_positive(0));
    }

    #[test]
    fn test_multiply_by_two() {
        assert_eq!(4, multiply_by_two(2));
        assert_eq!(12, multiply_by_two(6));
        assert_eq!(0, multiply_by_two(0));
        assert_eq!(2, multiply_by_two(1));
    }

    #[test]
    fn test_divide_by_two() {
        assert_eq!(2, divide_by_two(4));
        assert_eq!(12, divide_by_two(24));
        assert_eq!(0, divide_by_two(0));
        assert_eq!(0, divide_by_two(1));
    }

    #[test]
    fn test_twos_complement() {
        assert_eq!(-1, twos_complement(1));
        assert_eq!(0b000_0000, twos_complement(0b000_0000));
        assert_eq!(-127, twos_complement(127));
        assert_eq!(0, twos_complement(twos_complement(0)));
        assert_eq!(10, twos_complement(twos_complement(10)));
        assert_eq!(10, twos_complement(twos_complement(10)));
    }

    #[test]
    fn test_multiply_signed() {
        assert_eq!(-12, multiply_signed(-6, 2));
        assert_eq!(-8, multiply_signed(2, -4));
        assert_eq!(-12, multiply_signed(2, -6));
        assert_eq!(120, multiply_signed(30, 4));
        assert_eq!(120, multiply_signed(-30, -4));
        assert_eq!(36, multiply_signed(36, 1));
        assert_eq!(36, multiply_signed(1, 36));
    }

    #[test]
    fn test_multiply_unsigned() {
        assert_eq!(120, multiply_unsigned(30, 4));
        assert_eq!(120, multiply_unsigned(4, 30));
        assert_eq!(36, multiply_unsigned(36, 1));
        assert_eq!(36, multiply_unsigned(1, 36));
        assert_eq!(0, multiply_unsigned(1, 0));
        assert_eq!(0, multiply_unsigned(0, 1));
        assert_eq!(0, multiply_unsigned(0, 0));
        assert_eq!(64, multiply_unsigned(32, 2));
        assert_eq!(25, multiply_unsigned(5, 5));
    }

    #[test]
    fn test_count_ones() {
        assert_eq!(0, count_ones(0));
        assert_eq!(1, count_ones(1));
        assert_eq!(3, count_ones(0b101_0100));
        assert_eq!(1, count_ones(0b000_0100));
        assert_eq!(7, count_ones(0b111_1111));
    }

    #[test]
    fn test_bit_equivalence() {
        assert_eq!(0, bit_equivalence(0b000_0000, 0b111_1111));
        assert_eq!(6, bit_equivalence(0b000_0001, 0b000_0000));
        assert_eq!(7, bit_equivalence(0b111_1111, 0b111_1111));
    }

    #[test]
    fn test_bit_distance() {
        assert_eq!(0, bit_distance(0, 0));
        assert_eq!(0, bit_distance(127, 127));
        assert_eq!(0, bit_distance(55, 55));
        assert_eq!(6, bit_distance(0b111_1111, 0b000_0100));
        assert_eq!(6, bit_distance(0b101_1011, 0b000_0100));
        assert_eq!(7, bit_distance(0b111_1111, 0b000_0000));
    }

    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(0));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(16));
        assert!(is_power_of_two(64));

        assert!(!is_power_of_two(33));
        assert!(!is_power_of_two(124));
        assert!(!is_power_of_two(-13));
    }

    #[test]
    fn test_is_power_of_two_difference() {
        assert!(is_power_of_two_difference(2));
        assert!(is_power_of_two_difference(4));
        assert!(is_power_of_two_difference(12));
        assert!(is_power_of_two_difference(14));
        assert!(is_power_of_two_difference(124));

        assert!(!is_power_of_two_difference(33));
        assert!(!is_power_of_two_difference(-13));
    }

    #[test]
    fn test_rightmost_one() {
        assert_eq!(0b001_0000, rightmost_one(0b101_0000));
        assert_eq!(0b000_0000, rightmost_one(0b000_0000));
        assert_eq!(0b000_0001, rightmost_one(0b111_1111));
        assert_eq!(0b000_0010, rightmost_one(0b010_0110));
    }

    #[test]
    fn test_rightmost_zero() {
        assert_eq!(0b000_0001, rightmost_zero(0b101_0000));
        assert_eq!(0b000_0001, rightmost_zero(0b000_0000));
        assert_eq!(0b000_0000, rightmost_zero(0b111_1111));
        assert_eq!(0b000_1000, rightmost_zero(0b010_0111));
    }
}
