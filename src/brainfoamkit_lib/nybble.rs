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
    ops::Not,
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
    pub fn from_u8(n: u8) -> Option<Self> {
        // Check if the input value is within the valid range for a Nybble (0-15)
        if n <= 15 {
            // Create a new Nybble instance with default Bit values
            let mut nybble = Self::default();

            // Test each bit in the u8 value and flip the corresponding bit in the Nybble if necessary
            if n & 0b0001 != 0 {
                nybble.bit_0.flip();
            };
            if n & 0b0010 != 0 {
                nybble.bit_1.flip();
            };
            if n & 0b0100 != 0 {
                nybble.bit_2.flip();
            };
            if n & 0b1000 != 0 {
                nybble.bit_3.flip();
            };

            Some(nybble)
        } else {
            None // Return None if the input value is out of range
        }
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
            0 => self.bit_0 = Bit::one(),
            1 => self.bit_1 = Bit::one(),
            2 => self.bit_2 = Bit::one(),
            3 => self.bit_3 = Bit::one(),
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
            0 => self.bit_0 = Bit::zero(),
            1 => self.bit_1 = Bit::zero(),
            2 => self.bit_2 = Bit::zero(),
            3 => self.bit_3 = Bit::zero(),
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
    #[must_use]
    pub fn to_u8(&self) -> u8 {
        let mut n = 0;

        if self.get_bit(3) == Bit::One {
            n |= 8; // Set the fourth least significant bit of n to 1
        }
        if self.get_bit(2) == Bit::One {
            n |= 4; // Set the third least significant bit of n to 1
        }
        if self.get_bit(1) == Bit::One {
            n |= 2; // Set the second least significant bit of n to 1
        }
        if self.get_bit(0) == Bit::One {
            n |= 1; // Set the least significant bit of n to 1
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nybble_zero() {
        let nybble = Nybble::default();
        assert_eq!(nybble.to_u8(), 0);
        assert_eq!(nybble.to_string(), "0x00");
        assert_eq!(Nybble::from_u8(0), Some(nybble));
    }

    #[test]
    fn test_nybble_one() {
        let nybble = Nybble::from_u8(1).unwrap();
        assert_eq!(nybble.to_u8(), 1);
        assert_eq!(nybble.to_string(), "0x01");
        assert_eq!(Nybble::from_u8(1), Some(nybble));
    }

    #[test]
    fn test_nybble_two() {
        let nybble = Nybble::from_u8(2).unwrap();
        assert_eq!(nybble.to_u8(), 2);
        assert_eq!(nybble.to_string(), "0x02");
        assert_eq!(Nybble::from_u8(2), Some(nybble));
    }

    #[test]
    fn test_nybble_three() {
        let nybble = Nybble::from_u8(3).unwrap();
        assert_eq!(nybble.to_u8(), 3);
        assert_eq!(nybble.to_string(), "0x03");
        assert_eq!(Nybble::from_u8(3), Some(nybble));
    }

    #[test]
    fn test_nybble_four() {
        let nybble = Nybble::from_u8(4).unwrap();
        assert_eq!(nybble.to_u8(), 4);
        assert_eq!(nybble.to_string(), "0x04");
        assert_eq!(Nybble::from_u8(4), Some(nybble));
    }

    #[test]
    fn test_nybble_five() {
        let nybble = Nybble::from_u8(5).unwrap();
        assert_eq!(nybble.to_u8(), 5);
        assert_eq!(nybble.to_string(), "0x05");
        assert_eq!(Nybble::from_u8(5), Some(nybble));
    }

    #[test]
    fn test_nybble_six() {
        let nybble = Nybble::from_u8(6).unwrap();
        assert_eq!(nybble.to_u8(), 6);
        assert_eq!(nybble.to_string(), "0x06");
        assert_eq!(Nybble::from_u8(6), Some(nybble));
    }

    #[test]
    fn test_nybble_seven() {
        let nybble = Nybble::from_u8(7).unwrap();
        assert_eq!(nybble.to_u8(), 7);
        assert_eq!(nybble.to_string(), "0x07");
        assert_eq!(Nybble::from_u8(7), Some(nybble));
    }

    #[test]
    fn test_nybble_eight() {
        let nybble = Nybble::from_u8(8).unwrap();
        assert_eq!(nybble.to_u8(), 8);
        assert_eq!(nybble.to_string(), "0x08");
        assert_eq!(Nybble::from_u8(8), Some(nybble));
    }

    #[test]
    fn test_nybble_nine() {
        let nybble = Nybble::from_u8(9).unwrap();
        assert_eq!(nybble.to_u8(), 9);
        assert_eq!(nybble.to_string(), "0x09");
        assert_eq!(Nybble::from_u8(9), Some(nybble));
    }

    #[test]
    fn test_nybble_ten() {
        let nybble = Nybble::from_u8(10).unwrap();
        assert_eq!(nybble.to_u8(), 10);
        assert_eq!(nybble.to_string(), "0x0A");
        assert_eq!(Nybble::from_u8(10), Some(nybble));
    }

    #[test]
    fn test_nybble_eleven() {
        let nybble = Nybble::from_u8(11).unwrap();
        assert_eq!(nybble.to_u8(), 11);
        assert_eq!(nybble.to_string(), "0x0B");
        assert_eq!(Nybble::from_u8(11), Some(nybble));
    }

    #[test]
    fn test_nybble_twelve() {
        let nybble = Nybble::from_u8(12).unwrap();
        assert_eq!(nybble.to_u8(), 12);
        assert_eq!(nybble.to_string(), "0x0C");
        assert_eq!(Nybble::from_u8(12), Some(nybble));
    }

    #[test]
    fn test_nybble_thirteen() {
        let nybble = Nybble::from_u8(13).unwrap();
        assert_eq!(nybble.to_u8(), 13);
        assert_eq!(nybble.to_string(), "0x0D");
        assert_eq!(Nybble::from_u8(13), Some(nybble));
    }

    #[test]
    fn test_nybble_fourteen() {
        let nybble = Nybble::from_u8(14).unwrap();
        assert_eq!(nybble.to_u8(), 14);
        assert_eq!(nybble.to_string(), "0x0E");
        assert_eq!(Nybble::from_u8(14), Some(nybble));
    }

    #[test]
    fn test_nybble_fifteen() {
        let nybble = Nybble::from_u8(15).unwrap();
        assert_eq!(nybble.to_u8(), 15);
        assert_eq!(nybble.to_string(), "0x0F");
        assert_eq!(Nybble::from_u8(15), Some(nybble));
    }

    #[test]
    fn test_nybble_oob() {
        assert_eq!(Nybble::from_u8(16), None);
    }

    #[test]
    fn test_get_bit() {
        let nybble = Nybble::from_u8(12).unwrap();
        assert_eq!(nybble.get_bit(0), Bit::zero());
        assert_eq!(nybble.get_bit(1), Bit::zero());
        assert_eq!(nybble.get_bit(2), Bit::one());
        assert_eq!(nybble.get_bit(3), Bit::one());
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_bit_oob() {
        let nybble = Nybble::from_u8(12).unwrap();
        nybble.get_bit(4);
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
        let mut nybble = Nybble::from_u8(12).unwrap();
        nybble.set_bit(4);
    }

    #[test]
    fn test_unset_bit() {
        let mut nybble = Nybble::from_u8(15).unwrap();
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
        let mut nybble = Nybble::from_u8(12).unwrap();
        nybble.unset_bit(4);
    }

    #[test]
    fn test_flip_bit() {
        let mut nybble = Nybble::from_u8(15).unwrap();
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
        let mut nybble = Nybble::from_u8(12).unwrap();
        nybble.flip_bit(4);
    }

    #[test]
    fn test_flip() {
        let mut nybble = Nybble::from_u8(15).unwrap();
        nybble.flip();
        assert_eq!(nybble.to_u8(), 0);
        assert_eq!(nybble.to_string(), "0x00");
    }

    #[test]
    fn test_not() {
        let nybble = Nybble::from_u8(15).unwrap();
        let nybble_not = !nybble;
        assert_eq!(nybble_not.to_u8(), 0);
        assert_eq!(nybble_not.to_string(), "0x00");
    }
}
