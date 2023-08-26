// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2023
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// *     http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

use crate::Bit;
use std::{
    fmt::{self, Display, Formatter},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// A Nybble is a 4-bit unsigned integer (u4).
///
/// This is a wrapper around four Bit instances. The least significant bit is `bit_0` and the most significant bit is `bit_3`.
/// This struct is used to conveniently manipulate 4-bit values.
///
/// # Examples
///
/// ## Single Digit
///
/// ```
/// use brainfoamkit_lib::Nybble;
/// use brainfoamkit_lib::Bit;
///
/// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
/// assert_eq!(nybble.to_u8(), 10);
/// assert_eq!(nybble.to_string(), "0xA");
/// ```
///
/// ## Double Digits
///
/// ```
/// use brainfoamkit_lib::Nybble;
/// use brainfoamkit_lib::Bit;
///
/// let mut nybble = Nybble::default();
/// nybble.set_bit(0);
/// nybble.set_bit(1);
/// nybble.set_bit(2);
/// nybble.set_bit(3);
/// assert_eq!(nybble.to_u8(), 15);
/// assert_eq!(nybble.to_string(), "0xF");
/// ```
///
/// # Panics
///
/// The methods [`set_bit()`](#method.set_bit), [`unset_bit()`](#method.unset_bit)
/// and [`get_bit()`](#method.get_bit) will panic if the index is out of bounds.
///
/// # See Also
///
/// * [`Bit`](crate::Bit): The building block of a Nybble.
/// * [`Byte`](crate::Byte): A Byte is a collection of eight Bits.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nybble {
    /// The least significant bit.
    bit_0: Bit,
    /// The second least significant bit.
    bit_1: Bit,
    /// The second most significant bit.
    bit_2: Bit,
    /// The most significant bit.
    bit_3: Bit,
}

impl Nybble {
    /// Creates a new Nybble instance with the specified Bit values.
    ///
    /// This method takes four Bit instances as arguments.
    /// The least significant bit is `bit_0` and the most significant bit is `bit_3`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// assert_eq!(nybble.to_u8(), 10);
    /// assert_eq!(nybble.to_string(), "0xA");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble with the specified Bit values.
    ///
    /// # See Also
    ///
    /// * [`Nybble::from_u8()`](#method.from_u8): Creates a new Nybble from a u8 value.
    /// * [`Nybble::default()`](#method.default): Creates a new Nybble with default (all [`Bit::Zero`](crate::Bit::Zero)) Bit values.
    ///
    #[must_use]
    pub const fn new(first: Bit, second: Bit, third: Bit, fourth: Bit) -> Self {
        Self {
            bit_0: fourth, // Least significant bit
            bit_1: third,
            bit_2: second,
            bit_3: first, // Most Significant Bit
        }
    }

    /// Creates a new Nybble from a u8 value.
    ///
    /// This method takes a u8 value as an argument and creates a new Nybble truncating to only the least significant four bits.
    ///
    /// # Arguments
    ///
    /// * `n` - The u8 value to create the Nybble from.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::from_u8(5);
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x5");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble from the specified u8 value, or an all [`Bit::One`](crate::Bit::One) Nybble if the value is larger than 15.
    ///
    #[must_use]
    pub fn from_u8(n: u8) -> Self {
        // Use only the first four bits of the input value if it is larger than 15
        let n = if n > 15 { n & 0b1111 } else { n };

        // Create a new Nybble instance with default Bit values
        let mut nybble = Self::default();

        // Test each bit in the u8 value and flip the corresponding bit in the Nybble if necessary
        if n & 0b0001 != 0 {
            nybble.bit_0.set();
        };
        if n & 0b0010 != 0 {
            nybble.bit_1.set();
        };
        if n & 0b0100 != 0 {
            nybble.bit_2.set();
        };
        if n & 0b1000 != 0 {
            nybble.bit_3.set();
        };

        nybble
    }

    /// Sets the Bit value at the specified index.
    ///
    /// This method is used "Set" the bit value at a given index.
    /// This means that that bit value is set to 1.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 3.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit value to set.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut nybble = Nybble::default();
    /// nybble.set_bit(0);
    /// nybble.set_bit(2);
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x5");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # See Also
    ///
    /// * [`unset_bit()`](#method.unset_bit): Unsets the Bit value at the specified index.
    /// * [`get_bit()`](#method.get_bit): Gets the Bit value at the specified index.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified index.
    ///
    pub fn set_bit(&mut self, index: u8) {
        match index {
            0 => self.bit_0.set(),
            1 => self.bit_1.set(),
            2 => self.bit_2.set(),
            3 => self.bit_3.set(),
            _ => panic!("Index out of bounds"),
        }
    }

    /// Unsets the Bit value at the specified index.
    ///
    /// This method is used "Unset" the bit value at a given index.
    /// This means that that bit value is set to 0.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 3.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit value to unset.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut nybble = Nybble::default();
    /// nybble.set_bit(0);
    /// nybble.set_bit(2);
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x5");
    /// nybble.unset_bit(0);
    /// assert_eq!(nybble.to_u8(), 4);
    /// assert_eq!(nybble.to_string(), "0x4");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # Side Effects
    ///
    /// This method will [unset](crate::Bit#method.unset) the Bit value at the specified index.
    ///
    /// # See Also
    ///
    /// * [`set_bit()`](#method.set_bit): Sets the Bit value at the specified index.
    /// * [`get_bit()`](#method.get_bit): Gets the Bit value at the specified index.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified index.
    ///
    pub fn unset_bit(&mut self, index: u8) {
        match index {
            0 => self.bit_0.unset(),
            1 => self.bit_1.unset(),
            2 => self.bit_2.unset(),
            3 => self.bit_3.unset(),
            _ => panic!("Index out of bounds"),
        }
    }

    /// Converts the Nybble to an 8-bit unsigned integer (u8).
    ///
    /// This method converts the Nybble to an 8-bit unsigned integer (u8).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// let result = nybble.to_u8();
    /// assert_eq!(result, 0b1010);
    /// ```
    /// # Returns
    ///
    /// The Nybble as an 8-bit unsigned integer (u8).
    ///
    /// # See Also
    ///
    /// * [`from_u8()`](#method.from_u8): Creates a new Nybble from a u8 value.
    /// * [`to_string()`](#method.to_string): Converts the Nybble to a string.
    ///
    #[must_use]
    pub fn to_u8(&self) -> u8 {
        let mut n = 0;

        for i in 0..4 {
            if self.get_bit(i) == Bit::One {
                n |= 1 << i;
            }
        }

        n
    }

    /// Get the Bit value at the specified index.
    ///
    /// This method is used to get the bit value at a given index.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 3.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit value to get.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// assert_eq!(nybble.get_bit(0), Bit::Zero);
    /// assert_eq!(nybble.get_bit(1), Bit::One);
    /// assert_eq!(nybble.get_bit(2), Bit::Zero);
    /// assert_eq!(nybble.get_bit(3), Bit::One);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # Returns
    ///
    /// The Bit value at the specified index.
    ///
    /// # See Also
    ///
    /// * [`set_bit()`](#method.set_bit): Sets the Bit value at the specified index.
    /// * [`unset_bit()`](#method.unset_bit): Unsets the Bit value at the specified index.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified index.
    ///
    #[must_use]
    pub fn get_bit(&self, index: u8) -> Bit {
        match index {
            0 => self.bit_0,
            1 => self.bit_1,
            2 => self.bit_2,
            3 => self.bit_3,
            _ => panic!("Index out of bounds"),
        }
    }

    /// Flips the Bit value at the specified index.
    ///
    /// This method is used to flip the bit value at a given index.
    /// This means that that bit value is set to the opposite of its current value.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 3.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit value to flip.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut nybble = Nybble::default();
    /// nybble.set_bit(0);
    /// nybble.set_bit(2);
    ///
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x5");
    ///
    /// nybble.flip_bit(0);
    /// nybble.flip_bit(1);
    /// nybble.flip_bit(2);
    /// nybble.flip_bit(3);
    ///
    /// assert_eq!(nybble.to_u8(), 10);
    /// assert_eq!(nybble.to_string(), "0xA");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # Side Effects
    ///
    /// This method will [flip](crate::Bit#method.flip) the Bit value at the specified index.
    ///
    /// # See Also
    ///
    /// * [`set_bit()`](#method.set_bit): Sets the Bit value at the specified index.
    /// * [`unset_bit()`](#method.unset_bit): Unsets the Bit value at the specified index.
    /// * [`get_bit()`](#method.get_bit): Gets the Bit value at the specified index.
    ///
    pub fn flip_bit(&mut self, index: u8) {
        match index {
            0 => self.bit_0.flip(),
            1 => self.bit_1.flip(),
            2 => self.bit_2.flip(),
            3 => self.bit_3.flip(),
            _ => panic!("Index out of bounds"),
        }
    }

    /// Flips all of the Bit values in the Nybble.
    ///
    /// This method is used to flip all of the bit values in the Nybble.
    /// This means that all of the bit values are set to the opposite of their current values.
    /// This can be used to find the two's complement of a Nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut nybble = Nybble::default();
    ///
    /// nybble.set_bit(0);
    /// nybble.set_bit(2);
    ///
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x5");
    ///
    /// nybble.flip();
    ///
    /// assert_eq!(nybble.to_u8(), 10);
    /// assert_eq!(nybble.to_string(), "0xA");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will [flip](crate::Bit#method.flip) all of the Bit values in the Nybble.
    ///
    /// # See Also
    ///
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified index.
    ///
    pub fn flip(&mut self) {
        self.bit_0.flip();
        self.bit_1.flip();
        self.bit_2.flip();
        self.bit_3.flip();
    }
}

impl Display for Nybble {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#03X}", self.to_u8())
    }
}

impl Default for Nybble {
    fn default() -> Self {
        Self::new(Bit::zero(), Bit::zero(), Bit::zero(), Bit::zero())
    }
}

impl Not for Nybble {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut nybble = self;
        nybble.flip();
        nybble
    }
}

impl BitAnd for Nybble {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut nybble = self;
        nybble.bit_0 &= rhs.bit_0;
        nybble.bit_1 &= rhs.bit_1;
        nybble.bit_2 &= rhs.bit_2;
        nybble.bit_3 &= rhs.bit_3;
        nybble
    }
}

impl BitAndAssign for Nybble {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bit_0 &= rhs.bit_0;
        self.bit_1 &= rhs.bit_1;
        self.bit_2 &= rhs.bit_2;
        self.bit_3 &= rhs.bit_3;
    }
}

impl BitOr for Nybble {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut nybble = self;
        nybble.bit_0 |= rhs.bit_0;
        nybble.bit_1 |= rhs.bit_1;
        nybble.bit_2 |= rhs.bit_2;
        nybble.bit_3 |= rhs.bit_3;
        nybble
    }
}

impl BitOrAssign for Nybble {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bit_0 |= rhs.bit_0;
        self.bit_1 |= rhs.bit_1;
        self.bit_2 |= rhs.bit_2;
        self.bit_3 |= rhs.bit_3;
    }
}

impl BitXor for Nybble {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut nybble = self;
        nybble.bit_0 ^= rhs.bit_0;
        nybble.bit_1 ^= rhs.bit_1;
        nybble.bit_2 ^= rhs.bit_2;
        nybble.bit_3 ^= rhs.bit_3;
        nybble
    }
}

impl BitXorAssign for Nybble {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bit_0 ^= rhs.bit_0;
        self.bit_1 ^= rhs.bit_1;
        self.bit_2 ^= rhs.bit_2;
        self.bit_3 ^= rhs.bit_3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let nybble = Nybble::from_u8(10);
        assert_eq!(nybble.to_u8(), 0b1010);
    }

    #[test]
    fn test_from_u8_zero() {
        let nybble = Nybble::from_u8(0);
        assert_eq!(nybble.to_u8(), 0);
    }

    #[test]
    fn test_from_u8_all_ones() {
        let nybble = Nybble::from_u8(0b1111);
        assert_eq!(nybble.to_u8(), 0b1111);
    }

    #[test]
    fn test_from_u8_high_bits() {
        let nybble = Nybble::from_u8(0b10101010);
        assert_eq!(nybble.to_u8(), 0b1010);
    }

    #[test]
    fn test_get_bit() {
        let nybble = Nybble::from_u8(12);
        assert_eq!(nybble.get_bit(0), Bit::zero());
        assert_eq!(nybble.get_bit(1), Bit::zero());
        assert_eq!(nybble.get_bit(2), Bit::one());
        assert_eq!(nybble.get_bit(3), Bit::one());
    }

    #[test]
    #[allow(unused_variables)]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_bit_oob() {
        let nybble = Nybble::from_u8(12);
        let p = nybble.get_bit(4);
    }

    #[test]
    fn test_set_bit() {
        let mut nybble = Nybble::default();
        nybble.set_bit(0);
        nybble.set_bit(1);
        nybble.set_bit(2);
        nybble.set_bit(3);
        assert_eq!(nybble.to_u8(), 15);
        assert_eq!(nybble.to_string(), "0xF");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_set_bit_oob() {
        let mut nybble = Nybble::from_u8(12);
        nybble.set_bit(4);
    }

    #[test]
    fn test_unset_bit() {
        let mut nybble = Nybble::from_u8(15);
        nybble.unset_bit(0);
        nybble.unset_bit(1);
        nybble.unset_bit(2);
        nybble.unset_bit(3);
        assert_eq!(nybble.to_u8(), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_unset_bit_oob() {
        let mut nybble = Nybble::from_u8(12);
        nybble.unset_bit(4);
    }

    #[test]
    fn test_flip_bit() {
        let mut nybble = Nybble::from_u8(15);
        nybble.flip_bit(0);
        nybble.flip_bit(1);
        nybble.flip_bit(2);
        nybble.flip_bit(3);
        assert_eq!(nybble.to_u8(), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_flip_bit_oob() {
        let mut nybble = Nybble::from_u8(12);
        nybble.flip_bit(4);
    }

    #[test]
    fn test_flip() {
        let mut nybble = Nybble::from_u8(15);
        nybble.flip();
        assert_eq!(nybble.to_u8(), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    fn test_not() {
        let nybble = Nybble::from_u8(15);
        let nybble_not = !nybble;
        assert_eq!(nybble_not.to_u8(), 0);
        assert_eq!(nybble_not.to_string(), "0x0");
    }

    #[test]
    fn test_and() {
        let nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        let nybble_3 = nybble_1 & nybble_2;
        assert_eq!(nybble_3.to_u8(), 0b1000);
        assert_eq!(nybble_3.to_string(), "0x8");
    }

    #[test]
    fn test_and_assign() {
        let mut nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        nybble_1 &= nybble_2;
        assert_eq!(nybble_1.to_u8(), 0b1000);
        assert_eq!(nybble_1.to_string(), "0x8");
    }

    #[test]
    fn test_or() {
        let nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        let nybble_3 = nybble_1 | nybble_2;
        assert_eq!(nybble_3.to_u8(), 0b1110);
        assert_eq!(nybble_3.to_string(), "0xE");
    }

    #[test]
    fn test_or_assign() {
        let mut nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        nybble_1 |= nybble_2;
        assert_eq!(nybble_1.to_u8(), 0b1110);
        assert_eq!(nybble_1.to_string(), "0xE");
    }

    #[test]
    fn test_xor() {
        let nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        let nybble_3 = nybble_1 ^ nybble_2;
        assert_eq!(nybble_3.to_u8(), 0b0110);
        assert_eq!(nybble_3.to_string(), "0x6");
    }

    #[test]
    fn test_xor_assign() {
        let mut nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        nybble_1 ^= nybble_2;
        assert_eq!(nybble_1.to_u8(), 0b0110);
        assert_eq!(nybble_1.to_string(), "0x6");
    }

    #[test]
    fn test_display() {
        let nybble = Nybble::from_u8(10);
        assert_eq!(format!("{}", nybble), "0xA");
    }
}
