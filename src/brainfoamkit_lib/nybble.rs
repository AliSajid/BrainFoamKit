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
/// ```
/// use brainfoamkit_lib::Nybble;
/// use brainfoamkit_lib::Bit;
///
/// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
/// assert_eq!(nybble.to_u8(), 5);
/// assert_eq!(nybble.to_string(), "0x05");
/// ```
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
/// assert_eq!(nybble.to_string(), "0x0F");
/// ```
///
/// # Panics
///
/// The methods `set_bit()`, `unset_bit()` and `get_bit()` will panic if the index is out of bounds.
///

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nybble {
    bit_0: Bit,
    bit_1: Bit,
    bit_2: Bit,
    bit_3: Bit,
}

impl Nybble {
    /// Creates a new Nybble instance with the specified Bit values.
    ///
    /// This method takes four Bit instances as arguments. The least significant bit is `bit_0` and the most significant bit is `bit_3`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x05");
    /// ```
    #[must_use]
    pub const fn new(bit_0: Bit, bit_1: Bit, bit_2: Bit, bit_3: Bit) -> Self {
        Self {
            bit_0, // Least significant bit
            bit_1,
            bit_2,
            bit_3, // Most Significant Bit
        }
    }

    /// Creates a new Nybble instance from a u8 value.
    ///
    /// This method returns an Option. If the input value is out of range, None is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::from_u8(5).unwrap();
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x05");
    /// ```
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
    /// This method is used "Set" the bit value at a given index. This means that that bit value is set to 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut nybble = Nybble::default();
    /// nybble.set_bit(0);
    /// nybble.set_bit(2,);
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x05");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    pub fn set_bit(&mut self, index: usize) {
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
    /// This method is used "Uns" the bit value at a given index. This means that that bit value is set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut nybble = Nybble::default();
    /// nybble.set_bit(0);
    /// nybble.set_bit(2,);
    /// assert_eq!(nybble.to_u8(), 5);
    /// assert_eq!(nybble.to_string(), "0x05");
    /// nybble.unset_bit(0);
    /// assert_eq!(nybble.to_u8(), 4);
    /// assert_eq!(nybble.to_string(), "0x04");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    pub fn unset_bit(&mut self, index: usize) {
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
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// let result = nybble.to_u8();
    /// assert_eq!(result, 5);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// assert_eq!(nybble.get_bit(0), Bit::One);
    /// assert_eq!(nybble.get_bit(1), Bit::Zero);
    /// assert_eq!(nybble.get_bit(2), Bit::One);
    /// assert_eq!(nybble.get_bit(3), Bit::Zero);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    #[must_use]
    pub fn get_bit(&self, index: usize) -> Bit {
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
    /// This method is used to flip the bit value at a given index. This means that that bit value is set to the opposite of its current value.
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
    /// assert_eq!(nybble.to_string(), "0x05");
    ///
    /// nybble.flip_bit(0);
    /// nybble.flip_bit(1);
    /// nybble.flip_bit(2);
    /// nybble.flip_bit(3);
    ///
    /// assert_eq!(nybble.to_u8(), 10);
    /// assert_eq!(nybble.to_string(), "0x0A");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
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
    /// assert_eq!(nybble.to_string(), "0x05");
    ///
    /// nybble.flip();
    ///
    /// assert_eq!(nybble.to_u8(), 10);
    /// assert_eq!(nybble.to_string(), "0x0A");
    /// ```
    pub fn flip(&mut self) {
        self.bit_0.flip();
        self.bit_1.flip();
        self.bit_2.flip();
        self.bit_3.flip();
    }
}

impl Display for Nybble {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#04X}", self.to_u8())
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
        assert_eq!(nybble.to_string(), "0x0F");
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
        assert_eq!(nybble.to_string(), "0x00");
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
        assert_eq!(nybble.to_string(), "0x00");
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
        assert_eq!(nybble.to_string(), "0x00");
    }

    #[test]
    fn test_not() {
        let nybble = Nybble::from_u8(15);
        let nybble_not = !nybble;
        assert_eq!(nybble_not.to_u8(), 0);
        assert_eq!(nybble_not.to_string(), "0x00");
    }

    #[test]
    fn test_and() {
        let nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        let nybble_3 = nybble_1 & nybble_2;
        assert_eq!(nybble_3.to_u8(), 0b1000);
        assert_eq!(nybble_3.to_string(), "0x08");
    }

    #[test]
    fn test_and_assign() {
        let mut nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        nybble_1 &= nybble_2;
        assert_eq!(nybble_1.to_u8(), 0b1000);
        assert_eq!(nybble_1.to_string(), "0x08");
    }

    #[test]
    fn test_or() {
        let nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        let nybble_3 = nybble_1 | nybble_2;
        assert_eq!(nybble_3.to_u8(), 0b1110);
        assert_eq!(nybble_3.to_string(), "0x0E");
    }

    #[test]
    fn test_or_assign() {
        let mut nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        nybble_1 |= nybble_2;
        assert_eq!(nybble_1.to_u8(), 0b1110);
        assert_eq!(nybble_1.to_string(), "0x0E");
    }

    #[test]
    fn test_xor() {
        let nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        let nybble_3 = nybble_1 ^ nybble_2;
        assert_eq!(nybble_3.to_u8(), 0b0110);
        assert_eq!(nybble_3.to_string(), "0x06");
    }

    #[test]
    fn test_xor_assign() {
        let mut nybble_1 = Nybble::from_u8(0b1010);
        let nybble_2 = Nybble::from_u8(0b1100);
        nybble_1 ^= nybble_2;
        assert_eq!(nybble_1.to_u8(), 0b0110);
        assert_eq!(nybble_1.to_string(), "0x06");
    }

    #[test]
    fn test_display() {
        let nybble = Nybble::from_u8(10);
        assert_eq!(format!("{}", nybble), "0x0A");
    }
}
