// SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::Byte;

/// A struct representing an ASCII character.
///
/// This wraps a `Byte` value with metadata about its ASCII classification.
/// It provides methods to query character properties and retrieve
/// representations in different formats.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiChar,
///     Byte,
/// };
///
/// let letter =
///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
///
/// assert_eq!(letter.decimal_value(), 97);
/// assert_eq!(letter.hexadecimal_value(), "0x61");
/// assert_eq!(letter.character_value(), "a");
/// ```
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

    /// Returns `true` if the character is a control character.
    ///
    /// Control characters are non-printing characters (ASCII 0-31 and 127).
    /// They include tab, newline, carriage return, and other special codes.
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
    /// # References
    ///
    /// * [ASCII Control Characters](https://en.wikipedia.org/wiki/ASCII#Control_characters)
    #[must_use]
    pub fn is_control(&self) -> bool {
        self.decimal_value() < 32 || self.decimal_value() == 127
    }

    /// Returns `true` if the character is printable.
    ///
    /// Printable characters include letters, digits, punctuation, and spaces
    /// (ASCII 32-126). Control characters and the delete character (127) are
    /// not printable.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let letter =
    ///     AsciiChar::new(Byte::from(97), "LCA", "Lowercase letter a", "a");
    /// let digit = AsciiChar::new(Byte::from(49), "DIG1", "Digit one", "1");
    /// let space = AsciiChar::new(Byte::from(32), "SP", "Space", " ");
    ///
    /// assert_eq!(letter.is_printable(), true);
    /// assert_eq!(digit.is_printable(), true);
    /// assert_eq!(space.is_printable(), true);
    /// ```
    ///
    /// # References
    ///
    /// * [ASCII Printable Characters](https://en.wikipedia.org/wiki/ASCII#Printable_characters)
    #[must_use]
    pub fn is_printable(&self) -> bool {
        self.decimal_value() > 31 && self.decimal_value() < 127
    }

    /// Returns `true` if the character is whitespace.
    ///
    /// Whitespace includes space, tab, newline, and other spacing characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiChar,
    ///     Byte,
    /// };
    ///
    /// let tab = AsciiChar::new(Byte::from(9), "CTAB", "Horizontal tab", "\t");
    /// let space = AsciiChar::new(Byte::from(32), "SP", "Space", " ");
    ///
    /// assert_eq!(tab.is_whitespace(), true);
    /// assert_eq!(space.is_whitespace(), true);
    /// ```
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
                "Character with decimal value {val} should be identified as whitespace"
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
                "Character with decimal value {val} should be identified as a digit"
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
                "Character with decimal value {val} should be identified as a symbol"
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
