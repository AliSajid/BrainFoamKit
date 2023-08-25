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

use crate::{Bit, Nybble};
use std::{
    fmt::{self, Display, Formatter},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// A Byte is an 8-bit unsigned integer (u8).
///
/// This is a wrapper around four Bit instances. The least significant bit is `bit_0` and the most significant bit is `bit_7`.
/// This struct is used to conveniently manipulate 8-bit values.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::Byte;
/// use brainfoamkit_lib::Bit;
///
/// let Byte = Byte::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero);
/// assert_eq!(Byte.to_u8(), 170);
/// assert_eq!(Byte.to_string(), "0xAA");
/// ```
///
/// ```
/// use brainfoamkit_lib::Byte;
/// use brainfoamkit_lib::Bit;
///
/// let mut byte = Byte::default();
/// byte.set_bit(0);
/// byte.set_bit(1);
/// byte.set_bit(2);
/// byte.set_bit(3);
/// byte.set_bit(4);
/// byte.set_bit(5);
/// byte.set_bit(6);
/// byte.set_bit(7);
/// assert_eq!(byte.to_u8(), 255);
/// assert_eq!(byte.to_string(), "0xFF");
/// ```
///
/// # Panics
///
/// The methods `set_bit()`, `unset_bit()` and `get_bit()` will panic if the index is out of bounds.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Byte {
    bit_0: Bit,
    bit_1: Bit,
    bit_2: Bit,
    bit_3: Bit,
    bit_4: Bit,
    bit_5: Bit,
    bit_6: Bit,
    bit_7: Bit,
}

impl Byte {
    /// Creates a new Byte instance with the specified Bit values.
    ///
    /// This method takes four Bit instances as arguments. The least significant bit is `bit_0` and the most significant bit is `bit_3`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte = Byte::new(Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// assert_eq!(byte.to_u8(), 5);
    /// assert_eq!(byte.to_string(), "0x05");
    /// ```
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        bit_0: Bit,
        bit_1: Bit,
        bit_2: Bit,
        bit_3: Bit,
        bit_4: Bit,
        bit_5: Bit,
        bit_6: Bit,
        bit_7: Bit,
    ) -> Self {
        Self {
            bit_0, // Least significant bit
            bit_1,
            bit_2,
            bit_3, // Low Nybble from here on Up
            bit_4, // High Nybble from here on Down
            bit_5,
            bit_6,
            bit_7, // Most Significant Bit
        }
    }

    /// Creates a new Byte instance from a u8 value.
    ///
    /// This method returns an Option. If the input value is out of range, None is returned.
    ///
    /// # Examples
    ///
    /// ## Single Digit
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte_single = Byte::from_u8(5).unwrap();
    /// assert_eq!(byte.to_u8(), 5);
    /// assert_eq!(byte.to_string(), "0x05");
    /// ```
    /// ## Double Digits
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte_double = Byte::from_u8(85).unwrap();
    /// assert_eq!(byte.to_u8(), 85);
    /// assert_eq!(byte.to_string(), "0x55");
    /// ```
    ///
    #[must_use]
    pub fn from_u8(n: u8) -> Self {
        let mut byte = Self::default();

        for i in 0..8 {
            if n & (1 << i) != 0 {
                byte.flip_bit(i);
            }
        }

        byte
    }

    /// Creates a new Byte from two Nybbles.
    ///
    /// This method takes two Nybbles as arguments.
    /// The first Nybble is the most significant nybble and the second Nybble is the least significant nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let byte = Byte::from_nybbles(Nybble::new(0b1010), Nybble::new(0b0101));
    /// assert_eq!(byte.to_u8(), 85);
    /// assert_eq!(byte.to_string(), "0x55");
    /// ```
    ///
    #[must_use]
    pub fn from_nybbles(high_nybble: Nybble, low_nybble: Nybble) -> Self {
        let mut byte = Self::default();

        for i in 0..4 {
            if high_nybble.get_bit(i) == Bit::One {
                byte.set_bit(i + 4);
            }
            if low_nybble.get_bit(i) == Bit::One {
                byte.set_bit(i);
            }
        }

        byte
    }

    /// Gets the most significant nybble from the Byte.
    /// This method returns a Nybble.
    /// The most significant nybble is the first nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let byte = Byte::from_u8(85).unwrap();
    /// let high_nybble = byte.get_high_nybble();
    /// assert_eq!(high_nybble.to_u8(), 10);
    /// assert_eq!(high_nybble.to_string(), "0xA");
    /// ```
    ///
    pub fn get_high_nybble(&self) -> Nybble {
        let mut nybble = Nybble::default();

        for i in 0..4 {
            if self.get_bit(i + 4) == Bit::One {
                nybble.set_bit(i);
            }
        }

        nybble
    }

    /// Gets the least significant nybble from the Byte.
    /// This method returns a Nybble.
    /// The least significant nybble is the second nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let byte = Byte::from_u8(85).unwrap();
    /// let low_nybble = byte.get_low_nybble();
    /// assert_eq!(low_nybble.to_u8(), 5);
    /// assert_eq!(low_nybble.to_string(), "0x5");
    /// ```
    ///
    pub fn get_low_nybble(&self) -> Nybble {
        let mut nybble = Nybble::default();

        for i in 0..4 {
            if self.get_bit(i) == Bit::One {
                nybble.set_bit(i);
            }
        }

        nybble
    }

    /// Sets the Bit value at the specified index.
    ///
    /// This method is used "Set" the bit value at a given index. This means that that bit value is set to 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut byte = Byte::default();
    /// byte.set_bit(0);
    /// byte.set_bit(2);
    /// assert_eq!(byte.to_u8(), 5);
    /// assert_eq!(byte.to_string(), "0x05");
    /// byte.set_bit(4);
    /// byte.set_bit(6);
    /// assert_eq!(byte.to_u8(), 85);
    /// assert_eq!(byte.to_string(), "0x55");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    pub fn set_bit(&mut self, index: u8) {
        match index {
            0 => self.bit_0.set(),
            2 => self.bit_2.set(),
            1 => self.bit_1.set(),
            3 => self.bit_3.set(),
            4 => self.bit_4.set(),
            5 => self.bit_5.set(),
            6 => self.bit_6.set(),
            7 => self.bit_7.set(),
            _ => unreachable!("Index out of bounds"),
        }
    }

    /// Unsets the Bit value at the specified index.
    ///
    /// This method is used "Unset" the bit value at a given index. This means that that bit value is set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut byte = Byte::default();
    /// byte.set_bit(0);
    /// byte.set_bit(2,);
    /// assert_eq!(byte.to_u8(), 5);
    /// assert_eq!(byte.to_string(), "0x05");
    /// byte.unset_bit(0);
    /// assert_eq!(byte.to_u8(), 4);
    /// assert_eq!(byte.to_string(), "0x04");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    pub fn unset_bit(&mut self, index: u8) {
        match index {
            0 => self.bit_0.unset(),
            1 => self.bit_1.unset(),
            2 => self.bit_2.unset(),
            3 => self.bit_3.unset(),
            4 => self.bit_4.unset(),
            5 => self.bit_5.unset(),
            6 => self.bit_6.unset(),
            7 => self.bit_7.unset(),
            _ => unreachable!("Index out of bounds"),
        }
    }

    /// Converts the Byte to an 8-bit unsigned integer (u8).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte = Byte::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// let result = byte.to_u8();
    /// assert_eq!(result, 170);
    /// ```
    #[must_use]
    pub fn to_u8(&self) -> u8 {
        let mut n = 0;

        for i in 0..8 {
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
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte = Byte::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// assert_eq!(byte.get_bit(0), Bit::One);
    /// assert_eq!(byte.get_bit(1), Bit::Zero);
    /// assert_eq!(byte.get_bit(2), Bit::One);
    /// assert_eq!(byte.get_bit(3), Bit::Zero);
    /// assert_eq!(byte.get_bit(4), Bit::One);
    /// assert_eq!(byte.get_bit(5), Bit::Zero);
    /// assert_eq!(byte.get_bit(6), Bit::One);
    /// assert_eq!(byte.get_bit(7), Bit::Zero);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    #[must_use]
    pub fn get_bit(&self, index: u8) -> Bit {
        match index {
            0 => self.bit_0,
            1 => self.bit_1,
            2 => self.bit_2,
            3 => self.bit_3,
            4 => self.bit_4,
            5 => self.bit_5,
            6 => self.bit_6,
            7 => self.bit_7,
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
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut byte = Byte::default();
    /// byte.set_bit(0);
    /// byte.set_bit(2);
    /// byte.set_bit(4);
    /// byte.set_bit(6);
    ///
    /// assert_eq!(byte.to_u8(), 85);
    /// assert_eq!(byte.to_string(), "0x55");
    ///
    /// byte.flip_bit(0);
    /// byte.flip_bit(1);
    /// byte.flip_bit(2);
    /// byte.flip_bit(3);
    /// byte.flip_bit(4);
    /// byte.flip_bit(5);
    /// byte.flip_bit(6);
    /// byte.flip_bit(7);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    /// assert_eq!(byte.to_string(), "0xAA");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    pub fn flip_bit(&mut self, index: u8) {
        match index {
            0..=7 => {
                match index {
                    0 => &mut self.bit_0,
                    1 => &mut self.bit_1,
                    2 => &mut self.bit_2,
                    3 => &mut self.bit_3,
                    4 => &mut self.bit_4,
                    5 => &mut self.bit_5,
                    6 => &mut self.bit_6,
                    7 => &mut self.bit_7,
                    _ => unreachable!(),
                }
                .flip();
            }
            _ => panic!("Index out of bounds"),
        }
    }

    /// Flips all of the Bit values in the Byte.
    ///
    /// This method is used to flip all of the bit values in the Byte.
    /// This means that all of the bit values are set to the opposite of their current values.
    /// This can be used to find the two's complement of a Byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut byte = Byte::default();
    ///
    /// byte.set_bit(0);
    /// byte.set_bit(2);
    /// byte.set_bit(4);
    /// byte.set_bit(6);
    ///
    /// assert_eq!(byte.to_u8(), 85);
    /// assert_eq!(byte.to_string(), "0x55");
    ///
    /// byte.flip();
    ///
    /// assert_eq!(byte.to_u8(), 170);
    /// assert_eq!(byte.to_string(), "0xAA");
    /// ```
    pub fn flip(&mut self) {
        self.bit_0.flip();
        self.bit_1.flip();
        self.bit_2.flip();
        self.bit_3.flip();
        self.bit_4.flip();
        self.bit_5.flip();
        self.bit_6.flip();
        self.bit_7.flip();
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#04X}", self.to_u8())
    }
}

impl Default for Byte {
    fn default() -> Self {
        Self::new(
            Bit::zero(),
            Bit::zero(),
            Bit::zero(),
            Bit::zero(),
            Bit::zero(),
            Bit::zero(),
            Bit::zero(),
            Bit::zero(),
        )
    }
}

impl Not for Byte {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut byte = self;
        byte.flip();
        byte
    }
}

impl BitAnd for Byte {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut byte = self;
        byte.bit_0 &= rhs.bit_0;
        byte.bit_1 &= rhs.bit_1;
        byte.bit_2 &= rhs.bit_2;
        byte.bit_3 &= rhs.bit_3;
        byte.bit_4 &= rhs.bit_4;
        byte.bit_5 &= rhs.bit_5;
        byte.bit_6 &= rhs.bit_6;
        byte.bit_7 &= rhs.bit_7;
        byte
    }
}

impl BitAndAssign for Byte {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bit_0 &= rhs.bit_0;
        self.bit_1 &= rhs.bit_1;
        self.bit_2 &= rhs.bit_2;
        self.bit_3 &= rhs.bit_3;
        self.bit_4 &= rhs.bit_4;
        self.bit_5 &= rhs.bit_5;
        self.bit_6 &= rhs.bit_6;
        self.bit_7 &= rhs.bit_7;
    }
}

impl BitOr for Byte {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut byte = self;
        byte.bit_0 |= rhs.bit_0;
        byte.bit_1 |= rhs.bit_1;
        byte.bit_2 |= rhs.bit_2;
        byte.bit_3 |= rhs.bit_3;
        byte.bit_4 |= rhs.bit_4;
        byte.bit_5 |= rhs.bit_5;
        byte.bit_6 |= rhs.bit_6;
        byte.bit_7 |= rhs.bit_7;
        byte
    }
}

impl BitOrAssign for Byte {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bit_0 |= rhs.bit_0;
        self.bit_1 |= rhs.bit_1;
        self.bit_2 |= rhs.bit_2;
        self.bit_3 |= rhs.bit_3;
        self.bit_4 |= rhs.bit_4;
        self.bit_5 |= rhs.bit_5;
        self.bit_6 |= rhs.bit_6;
        self.bit_7 |= rhs.bit_7;
    }
}

impl BitXor for Byte {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut byte = self;
        byte.bit_0 ^= rhs.bit_0;
        byte.bit_1 ^= rhs.bit_1;
        byte.bit_2 ^= rhs.bit_2;
        byte.bit_3 ^= rhs.bit_3;
        byte.bit_4 ^= rhs.bit_4;
        byte.bit_5 ^= rhs.bit_5;
        byte.bit_6 ^= rhs.bit_6;
        byte.bit_7 ^= rhs.bit_7;
        byte
    }
}

impl BitXorAssign for Byte {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bit_0 ^= rhs.bit_0;
        self.bit_1 ^= rhs.bit_1;
        self.bit_2 ^= rhs.bit_2;
        self.bit_3 ^= rhs.bit_3;
        self.bit_4 ^= rhs.bit_4;
        self.bit_5 ^= rhs.bit_5;
        self.bit_6 ^= rhs.bit_6;
        self.bit_7 ^= rhs.bit_7;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let byte = Byte::from_u8(0b10101010);
        assert_eq!(format!("{}", byte), "0xAA");
    }

    #[test]
    fn test_default() {
        let byte = Byte::default();
        assert_eq!(byte.to_u8(), 0b00000000);
    }

    #[test]
    fn test_from_u8_zero() {
        let byte = Byte::from_u8(0);
        assert_eq!(byte, Byte::default());
    }

    #[test]
    fn test_from_u8_one() {
        let byte = Byte::from_u8(1);
        let mut expected = Byte::default();
        expected.set_bit(0);
        assert_eq!(byte, expected);
    }

    #[test]
    fn test_from_u8_max() {
        let byte = Byte::from_u8(u8::MAX);
        let mut expected = Byte::default();
        for i in 0..8 {
            expected.set_bit(i);
        }
        assert_eq!(byte, expected);
    }

    #[test]
    fn test_bitnot() {
        let byte = Byte::from_u8(0b10101010);
        let result = !byte;
        assert_eq!(result.to_u8(), 0b01010101);

        let byte = Byte::from_u8(0b11110000);
        let result = !byte;
        assert_eq!(result.to_u8(), 0b00001111);

        let byte = Byte::from_u8(0b00000000);
        let result = !byte;
        assert_eq!(result.to_u8(), 0b11111111);
    }

    #[test]
    fn test_bitand() {
        let byte1 = Byte::from_u8(0b10101010);
        let byte2 = Byte::from_u8(0b11001100);
        let result = byte1 & byte2;
        assert_eq!(result.to_u8(), 0b10001000);

        let byte1 = Byte::from_u8(0b11110000);
        let byte2 = Byte::from_u8(0b00001111);
        let result = byte1 & byte2;
        assert_eq!(result.to_u8(), 0b00000000);

        let byte1 = Byte::from_u8(0b11111111);
        let byte2 = Byte::from_u8(0b11111111);
        let result = byte1 & byte2;
        assert_eq!(result.to_u8(), 0b11111111);
    }

    #[test]
    fn test_bitand_assign() {
        let mut byte1 = Byte::from_u8(0b10101010);
        let byte2 = Byte::from_u8(0b11001100);
        byte1 &= byte2;
        assert_eq!(byte1.to_u8(), 0b10001000);

        let mut byte1 = Byte::from_u8(0b11110000);
        let byte2 = Byte::from_u8(0b00001111);
        byte1 &= byte2;
        assert_eq!(byte1.to_u8(), 0b00000000);

        let mut byte1 = Byte::from_u8(0b11111111);
        let byte2 = Byte::from_u8(0b11111111);
        byte1 &= byte2;
        assert_eq!(byte1.to_u8(), 0b11111111);
    }

    #[test]
    fn test_bitor() {
        let byte1 = Byte::from_u8(0b10101010);
        let byte2 = Byte::from_u8(0b11001100);
        let result = byte1 | byte2;
        assert_eq!(result.to_u8(), 0b11101110);

        let byte1 = Byte::from_u8(0b11110000);
        let byte2 = Byte::from_u8(0b00001111);
        let result = byte1 | byte2;
        assert_eq!(result.to_u8(), 0b11111111);

        let byte1 = Byte::from_u8(0b00000000);
        let byte2 = Byte::from_u8(0b11111111);
        let result = byte1 | byte2;
        assert_eq!(result.to_u8(), 0b11111111);
    }

    #[test]
    fn test_bitor_assign() {
        let mut byte1 = Byte::from_u8(0b10101010);
        let byte2 = Byte::from_u8(0b11001100);
        byte1 |= byte2;
        assert_eq!(byte1.to_u8(), 0b11101110);

        let mut byte1 = Byte::from_u8(0b11110000);
        let byte2 = Byte::from_u8(0b00001111);
        byte1 |= byte2;
        assert_eq!(byte1.to_u8(), 0b11111111);

        let mut byte1 = Byte::from_u8(0b00000000);
        let byte2 = Byte::from_u8(0b11111111);
        byte1 |= byte2;
        assert_eq!(byte1.to_u8(), 0b11111111);
    }

    #[test]
    fn test_bitxor() {
        let byte1 = Byte::from_u8(0b10101010);
        let byte2 = Byte::from_u8(0b11001100);
        let result = byte1 ^ byte2;
        assert_eq!(result.to_u8(), 0b01100110);

        let byte1 = Byte::from_u8(0b11110000);
        let byte2 = Byte::from_u8(0b00001111);
        let result = byte1 ^ byte2;
        assert_eq!(result.to_u8(), 0b11111111);

        let byte1 = Byte::from_u8(0b00000000);
        let byte2 = Byte::from_u8(0b11111111);
        let result = byte1 ^ byte2;
        assert_eq!(result.to_u8(), 0b11111111);
    }

    #[test]
    fn test_bitxor_assign() {
        let mut byte1 = Byte::from_u8(0b10101010);
        let byte2 = Byte::from_u8(0b11001100);
        byte1 ^= byte2;
        assert_eq!(byte1.to_u8(), 0b01100110);

        let mut byte1 = Byte::from_u8(0b11110000);
        let byte2 = Byte::from_u8(0b00001111);
        byte1 ^= byte2;
        assert_eq!(byte1.to_u8(), 0b11111111);

        let mut byte1 = Byte::from_u8(0b00000000);
        let byte2 = Byte::from_u8(0b11111111);
        byte1 ^= byte2;
        assert_eq!(byte1.to_u8(), 0b11111111);
    }
}
