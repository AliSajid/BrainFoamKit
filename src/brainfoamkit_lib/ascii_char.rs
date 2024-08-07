// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::Byte;

/// A struct representing an ASCII character.
///
/// This struct is used to represent an ASCII character in terms of a
/// [`Byte`](struct.Byte.html) value. This struct allows for interrogating the
/// character in terms of its binary, decimal, and hexadecimal values, as well
/// as its character code, character description, and character value.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
///
/// assert_eq!(ascii_char.binary_value(), Byte::from(97));
/// assert_eq!(ascii_char.decimal_value(), 97);
/// assert_eq!(ascii_char.hexadecimal_value(), "0x61");
/// assert_eq!(ascii_char.character_code(), "LCA");
/// assert_eq!(ascii_char.character_description(), "Lowercase letter a");
/// assert_eq!(ascii_char.character_value(), "a");
/// ```
///
/// # ASCII Character Types
///
/// The following ASCII character types are available:
///
/// * Control characters
/// * Printable characters
///     * Whitespace characters
///     * Digit characters
///     * Letter characters
///         * Uppercase characters
///         * Lowercase characters
///     * Symbol characters
///
/// # ASCII Character Type Examples
///
/// ## Control Characters
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(0), "NUL", "Null character", "\0");
///
/// assert_eq!(ascii_char.is_control(), true);
/// ```
///
/// ## Printable Characters
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
///
/// assert_eq!(ascii_char.is_printable(), true);
/// ```
///
/// ## Whitespace Characters
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(9), "CTAB", "Horizontal tab", "\t");
///
/// assert_eq!(ascii_char.is_whitespace(), true);
/// ```
///
/// ## Digit Characters
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(49), "DIG1", "Digit one", "1");
///
/// assert_eq!(ascii_char.is_digit(), true);
/// ```
///
/// ## Letter Characters
///
/// ### Uppercase Characters
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(65), "UCA", "Uppercase letter a", "A");
///
/// assert_eq!(ascii_char.is_uppercase(), true);
/// assert_eq!(ascii_char.is_lowercase(), false);
/// assert_eq!(ascii_char.is_letter(), true);
/// ```
///
/// ### Lowercase Characters
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
///
/// assert_eq!(ascii_char.is_lowercase(), true);
/// assert_eq!(ascii_char.is_uppercase(), false);
/// assert_eq!(ascii_char.is_letter(), true);
/// ```
///
/// ## Symbol Characters
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let ascii_char: AsciiChar =
///     AsciiChar::new(Byte::from(64), "AT", "At Symbol", "@");
///
/// assert_eq!(ascii_char.is_symbol(), true);
/// ```
///
/// # References
///
/// * [ASCII](https://en.wikipedia.org/wiki/ASCII)
/// * [ASCII Table](https://www.asciitable.com/)
/// * [ASCII Table and Description](https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsciiChar {
    binary_value:          Byte,
    character_code:        String,
    character_description: String,
    character_value:       String,
}

impl AsciiChar {
    /// Create a new `AsciiChar` instance.
    ///
    /// This function creates a new `AsciiChar` instance from the given
    /// [`Byte`](struct.Byte.html) value, character code, character
    /// description, and character value.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.binary_value(), Byte::from(97));
    /// assert_eq!(ascii_char.decimal_value(), 97);
    /// assert_eq!(ascii_char.hexadecimal_value(), "0x61");
    /// assert_eq!(ascii_char.character_code(), "LCA");
    /// assert_eq!(ascii_char.character_description(), "Lowercase letter a");
    /// ```
    #[must_use]
    pub fn new(
        byte: Byte,
        character_code: &str,
        character_description: &str,
        character_value: &str,
    ) -> Self {
        let binary_value: Byte = byte;
        let character_code: String = character_code.to_string().to_uppercase();
        let character_description: String = character_description.to_string();
        let character_value: String = character_value.to_string();

        Self {
            binary_value,
            character_code,
            character_description,
            character_value,
        }
    }

    /// Returns `true` if the `AsciiChar` instance is a control character.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(0), "NUL", "Null character", "\0");
    ///
    /// assert_eq!(ascii_char.is_control(), true);
    /// ```
    ///
    /// ## Control Characters
    ///
    /// * NUL (Null character)
    /// * SOH (Start of heading)
    /// * STX (Start of text)
    /// * ETX (End of text)
    /// * EOT (End of transmission)
    /// * ENQ (Enquiry)
    /// * ACK (Acknowledgement)
    /// * BEL (Bell)
    /// * BS (Backspace)
    /// * HT (Horizontal tab)
    /// * LF (Line feed)
    /// * VT (Vertical tab)
    /// * FF (Form feed)
    /// * CR (Carriage return)
    /// * SO (Shift out)
    /// * SI (Shift in)
    /// * DLE (Data link escape)
    /// * DC1 (Device control 1)
    /// * DC2 (Device control 2)
    /// * DC3 (Device control 3)
    /// * DC4 (Device control 4)
    /// * NAK (Negative acknowledgement)
    /// * SYN (Synchronous idle)
    /// * ETB (End of transmission block)
    /// * CAN (Cancel)
    /// * EM (End of medium)
    /// * SUB (Substitute)
    /// * ESC (Escape)
    /// * FS (File separator)
    /// * GS (Group separator)
    /// * RS (Record separator)
    /// * US (Unit separator)
    /// * DEL (Delete)
    ///
    /// # References
    ///
    /// * [ASCII Control Characters](https://en.wikipedia.org/wiki/ASCII#Control_characters)
    #[must_use]
    pub fn is_control(&self) -> bool {
        self.decimal_value() < 32 || self.decimal_value() == 127
    }

    /// Returns `true` if the `AsciiChar` instance is a printable character.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let lca: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    /// let uca: AsciiChar =
    ///     AsciiChar::new(Byte::from(65), "UCA", "Uppercase letter a", "A");
    /// let symat: AsciiChar =
    ///     AsciiChar::new(Byte::from(64), "SYMAT", "Symbol At", "@");
    /// let dig1: AsciiChar =
    ///     AsciiChar::new(Byte::from(49), "DIG1", "Digit one", "1");
    /// let sp: AsciiChar = AsciiChar::new(Byte::from(32), "SP", "Space", " ");
    ///
    /// assert_eq!(lca.is_printable(), true);
    /// assert_eq!(uca.is_printable(), true);
    /// assert_eq!(symat.is_printable(), true);
    /// assert_eq!(dig1.is_printable(), true);
    /// assert_eq!(sp.is_printable(), true);
    /// ```
    ///
    /// ## Printable Characters
    ///
    /// * [All uppercase letters](#methods.is_uppercase)
    /// * [All lowercase letters](#methods.is_lowercase)
    /// * [All digit characters](#methods.is_digit)
    /// * [All symbol characters](#methods.is_symbol)
    /// * [All whitespace characters](#methods.is_whitespace)
    ///
    /// # References
    ///
    /// * [ASCII Printable Characters](https://en.wikipedia.org/wiki/ASCII#Printable_characters)
    #[must_use]
    pub fn is_printable(&self) -> bool {
        self.decimal_value() > 31 && self.decimal_value() < 127
    }

    /// Returns `true` if the `AsciiChar` instance is a whitespace character.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(9), "CTAB", "Horizontal tab", "\t");
    ///
    /// assert_eq!(ascii_char.is_whitespace(), true);
    /// ```
    ///
    /// ## Whitespace Characters
    ///
    /// * SP (Space)
    /// * HT (Horizontal tab)
    /// * LF (Line feed)
    /// * VT (Vertical tab)
    /// * FF (Form feed)
    /// * CR (Carriage return)
    ///
    /// # References
    ///
    /// * [ASCII Whitespace Characters](https://en.wikipedia.org/wiki/Whitespace_character)
    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        self.decimal_value() == 9
            || self.decimal_value() == 10
            || self.decimal_value() == 11
            || self.decimal_value() == 12
            || self.decimal_value() == 13
            || self.decimal_value() == 32
    }

    /// Returns `true` if the `AsciiChar` instance is a digit character.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(49), "DIG1", "Digit one", "1");
    ///
    /// assert_eq!(ascii_char.is_digit(), true);
    /// ```
    ///
    /// ## Digit Characters
    ///
    /// * All digit characters from 0 to 9
    ///
    /// # References
    ///
    /// * [ASCII Digit Characters](https://en.wikipedia.org/wiki/ASCII)
    #[must_use]
    pub fn is_digit(&self) -> bool {
        self.decimal_value() > 47 && self.decimal_value() < 58
    }

    /// Returns `true` if the `AsciiChar` instance is a letter character.
    ///
    /// # Examples
    ///
    /// ## Lowercase Letters
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.is_letter(), true);
    /// ```
    ///
    /// ## Uppercase Letters
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let lca: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    /// let uca: AsciiChar =
    ///     AsciiChar::new(Byte::from(65), "UCA", "Uppercase letter a", "A");
    ///
    /// assert_eq!(lca.is_letter(), true);
    /// assert_eq!(uca.is_letter(), true);
    /// ```
    ///
    /// ## Letter Characters
    ///
    /// * [All uppercase letters](#methods.is_uppercase)
    /// * [All lowercase letters](#methods.is_lowercase)
    ///
    /// # References
    ///
    /// * [ASCII Letter Characters](https://en.wikipedia.org/wiki/ASCII)
    #[must_use]
    pub fn is_letter(&self) -> bool {
        self.is_lowercase() || self.is_uppercase()
    }

    /// Returns `true` if the `AsciiChar` instance is an uppercase letter
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(65), "UCA", "Uppercase letter A", "A");
    ///
    /// assert_eq!(ascii_char.is_uppercase(), true);
    /// ```
    ///
    /// ## Uppercase Letters
    ///
    /// * All uppercase letters from A to Z
    ///
    /// # References
    ///
    /// * [ASCII Uppercase Characters](https://en.wikipedia.org/wiki/ASCII)
    #[must_use]
    pub fn is_uppercase(&self) -> bool {
        self.decimal_value() > 64 && self.decimal_value() < 91
    }

    /// Returns `true` if the `AsciiChar` instance is a lowercase letter
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.is_lowercase(), true);
    /// ```
    ///
    /// ## Lowercase Letters
    ///
    /// * All lowercase letters from a to z
    ///
    /// # References
    ///
    /// * [ASCII Lowercase Characters](https://en.wikipedia.org/wiki/ASCII)
    #[must_use]
    pub fn is_lowercase(&self) -> bool {
        self.decimal_value() > 96 && self.decimal_value() < 123
    }

    /// Returns `true` if the `AsciiChar` instance is a symbol character.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(64), "SYMAT", "Symbol at", "@");
    ///
    /// assert_eq!(ascii_char.is_symbol(), true);
    /// ```
    ///
    /// ## Symbol Characters
    ///
    /// * All symbol characters from ! to /
    /// * All symbol characters from : to @
    /// * All symbol characters from [ to `
    /// * All symbol characters from { to ~
    ///
    /// # References
    ///
    /// * [ASCII Symbol Characters](https://en.wikipedia.org/wiki/ASCII)
    #[must_use]
    #[allow(clippy::doc_markdown)]
    pub fn is_symbol(&self) -> bool {
        self.decimal_value() > 32 && self.decimal_value() < 48
            || self.decimal_value() > 57 && self.decimal_value() < 65
            || self.decimal_value() > 90 && self.decimal_value() < 97
            || self.decimal_value() > 122 && self.decimal_value() < 127
    }

    /// Returns the `AsciiChar` instance's binary value.
    ///
    /// This function returns the `AsciiChar` instance's binary value as a
    /// [`Byte`](struct.Byte.html).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.binary_value(), Byte::from(97));
    /// ```
    #[must_use]
    pub const fn binary_value(&self) -> Byte {
        self.binary_value
    }

    /// Returns the `AsciiChar` instance's decimal value.
    ///
    /// This function returns the `AsciiChar` instance's decimal value as a
    /// `u8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.decimal_value(), 97);
    /// ```
    #[must_use]
    pub fn decimal_value(&self) -> u8 {
        u8::from(&self.binary_value)
    }

    /// Returns the `AsciiChar` instance's hexadecimal value.
    ///
    /// This function returns the `AsciiChar` instance's hexadecimal value as a
    /// `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.hexadecimal_value(), "0x61");
    /// ```
    #[must_use]
    pub fn hexadecimal_value(&self) -> String {
        format!("{:#04X}", self.decimal_value())
    }

    /// Returns the `AsciiChar` instance's character code.
    ///
    /// This function returns the `AsciiChar` instance's character code as a
    /// `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.character_code(), "LCA");
    /// ```
    #[must_use]
    pub fn character_code(&self) -> String {
        self.character_code.clone()
    }

    /// Returns the `AsciiChar` instance's character description.
    ///
    /// This function returns the `AsciiChar` instance's character description
    /// as a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.character_description(), "Lowercase letter a");
    /// ```
    #[must_use]
    pub fn character_description(&self) -> String {
        self.character_description.clone()
    }

    /// Returns the `AsciiChar` instance's character value.
    ///
    /// This function returns the `AsciiChar` instance's character value as a
    /// `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let ascii_char: AsciiChar =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    ///
    /// assert_eq!(ascii_char.character_value(), "a");
    /// ```
    #[must_use]
    pub fn character_value(&self) -> String {
        self.character_value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_char() {
        let ascii_char: AsciiChar =
            AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");

        assert_eq!(ascii_char.binary_value(), Byte::from(97));
        assert_eq!(ascii_char.decimal_value(), 97);
        assert_eq!(ascii_char.hexadecimal_value(), "0x61");
        assert_eq!(ascii_char.character_code(), "LCA");
        assert_eq!(ascii_char.character_description(), "Lowercase letter a");
        assert_eq!(ascii_char.character_value(), "a");
    }

    #[test]
    fn test_ascii_char_is_control() {
        let ascii_char: AsciiChar = AsciiChar::new(Byte::from(0), "NUL", "Null character", "\0");

        assert!(ascii_char.is_control());
    }

    #[test]
    fn test_ascii_char_is_printable() {
        let lca: AsciiChar = AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
        let uca: AsciiChar = AsciiChar::new(Byte::from(65), "UCA", "Uppercase letter a", "A");
        let symat: AsciiChar = AsciiChar::new(Byte::from(64), "SYMAT", "Symbol At", "@");
        let dig1: AsciiChar = AsciiChar::new(Byte::from(49), "DIG1", "Digit one", "1");
        let sp: AsciiChar = AsciiChar::new(Byte::from(32), "SP", "Space", " ");

        assert!(lca.is_printable());
        assert!(uca.is_printable());
        assert!(symat.is_printable());
        assert!(dig1.is_printable());
        assert!(sp.is_printable());
    }

    #[test]
    fn test_ascii_char_is_letter() {
        let lca: AsciiChar = AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
        let uca: AsciiChar = AsciiChar::new(Byte::from(65), "UCA", "Uppercase letter a", "A");

        assert!(lca.is_letter());
        assert!(uca.is_letter());
    }

    #[test]
    fn test_ascii_char_is_uppercase() {
        let ascii_char: AsciiChar =
            AsciiChar::new(Byte::from(65), "UCA", "Uppercase letter a", "A");

        assert!(ascii_char.is_uppercase());
        assert!(!ascii_char.is_lowercase());
        assert!(ascii_char.is_letter());
    }

    #[test]
    fn test_ascii_char_is_lowercase() {
        let ascii_char: AsciiChar =
            AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");

        assert!(ascii_char.is_lowercase());
        assert!(!ascii_char.is_uppercase());
        assert!(ascii_char.is_letter());
    }

    #[test]
    fn test_ascii_char_is_whitespace() {
        let whitespace_chars = vec![9, 10, 11, 12, 13, 32];
        for &val in &whitespace_chars {
            let ascii_char = AsciiChar::new(Byte::from(val), "", "", "");
            assert!(
                ascii_char.is_whitespace(),
                "Character with decimal value {} should be identified as whitespace",
                val
            );
        }

        let non_whitespace_char = AsciiChar::new(Byte::from(65), "", "", "");
        assert!(
            !non_whitespace_char.is_whitespace(),
            "Character with decimal value 65 should not be identified as whitespace"
        );
    }

    #[test]
    fn test_ascii_char_is_digit() {
        let digit_chars = vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
        for &val in &digit_chars {
            let ascii_char = AsciiChar::new(Byte::from(val), "", "", "");
            assert!(
                ascii_char.is_digit(),
                "Character with decimal value {} should be identified as a digit",
                val
            );
        }

        let non_digit_char = AsciiChar::new(Byte::from(65), "", "", "");
        assert!(
            !non_digit_char.is_digit(),
            "Character with decimal value 65 should not be identified as a digit"
        );
    }

    #[test]
    fn test_ascii_char_is_symbol() {
        let symbol_chars = vec![
            33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64,
            91, 92, 93, 94, 95, 96, 123, 124, 125, 126,
        ];
        for &val in &symbol_chars {
            let ascii_char = AsciiChar::new(Byte::from(val), "", "", "");
            assert!(
                ascii_char.is_symbol(),
                "Character with decimal value {} should be identified as a symbol",
                val
            );
        }

        let non_symbol_char = AsciiChar::new(Byte::from(65), "", "", "");
        assert!(
            !non_symbol_char.is_symbol(),
            "Character with decimal value 65 should not be identified as a symbol"
        );
    }

    #[test]
    fn test_ascii_char_binary_value() {
        let ascii_char = AsciiChar::new(Byte::from(97), "", "", "");
        assert_eq!(
            ascii_char.binary_value(),
            Byte::from(97),
            "Binary value should be equal to the input value"
        );
    }
}
