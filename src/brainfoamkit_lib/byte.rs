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
/// This is a wrapper around eight Bit instances. The least significant bit is `bit_0` and the most significant bit is `bit_7`.
/// This struct is used to conveniently manipulate 8-bit values.
///
/// Note that the Bit instances are stored in reverse (LSB to MSB) order,
/// but the constructor takes them in the correct order (MSB to LSB) to provide a predictable and intuitive interface.
///
/// # Examples
///
/// ## Create a byte from primitive Bit values
///
/// An easy way create a byte is to use the [`Byte::new()`](#method.new) method.
/// This method takes eight [Bit](crate::Bit) instances as arguments. The least significant bit is `bit_0` and the most significant bit is `bit_7`.
/// The order of the arguments is the same as the order of the bits in the byte.
///
/// ```
/// use brainfoamkit_lib::Byte;
/// use brainfoamkit_lib::Bit;
///
/// let byte = Byte::new(Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero());
/// assert_eq!(byte.to_u8(), 170);
/// assert_eq!(byte.to_string(), "0xAA");
/// ```
/// ## Create a byte from a primitive u8 value
///
/// ```
/// use brainfoamkit_lib::Byte;
///
/// let byte = Byte::from_u8(170);
/// assert_eq!(byte.to_u8(), 170);
/// assert_eq!(byte.to_string(), "0xAA");
/// ```
///
/// ## Set and Unset bits to generate desired byte
///
/// ```
/// use brainfoamkit_lib::Byte;
/// use brainfoamkit_lib::Bit;
///
/// let mut byte = Byte::default();
/// byte.set_bit(0);
/// byte.set_bit(1);
/// byte.set_bit(2);
/// assert_eq!(byte.to_u8(), 7);
/// assert_eq!(byte.to_string(), "0x07");
/// byte.unset_bit(1);
/// assert_eq!(byte.to_u8(), 5);
/// assert_eq!(byte.to_string(), "0x05");
/// ```
///
/// ## Get the Bit value at a given index
///
/// ```
/// use brainfoamkit_lib::Byte;
/// use brainfoamkit_lib::Bit;
///
/// let byte = Byte::from_u8(170);
/// assert_eq!(byte.get_bit(0), Bit::Zero);
/// assert_eq!(byte.get_bit(1), Bit::One);
/// assert_eq!(byte.get_bit(2), Bit::Zero);
/// assert_eq!(byte.get_bit(3), Bit::One);
/// assert_eq!(byte.get_bit(4), Bit::Zero);
/// assert_eq!(byte.get_bit(5), Bit::One);
/// assert_eq!(byte.get_bit(6), Bit::Zero);
/// assert_eq!(byte.get_bit(7), Bit::One);
/// ```
///
/// ## Flip the Bit value at a given index
///
/// ```
/// use brainfoamkit_lib::Byte;
/// use brainfoamkit_lib::Bit;
///
/// let mut byte = Byte::default();
/// byte.set_bit(0);
/// byte.set_bit(2);
/// byte.set_bit(4);
/// assert_eq!(byte.to_u8(), 0b00010101); // 21
/// assert_eq!(byte.to_string(), "0x15");
/// byte.flip_bit(2);
/// assert_eq!(byte.to_u8(), 0b00010001); // 17
/// assert_eq!(byte.to_string(), "0x11");
/// byte.flip_bit(7);
/// assert_eq!(byte.to_u8(), 0b10010001); // 145
/// assert_eq!(byte.to_string(), "0x91");
/// ```
///
/// # Panics
///
/// The methods [`set_bit()`](#method.set_bit), [`unset_bit()`](#method.unset_bit) and
/// [`get_bit()`](#method.get_bit) will panic if the index is out of bounds.
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
    /// This method takes eight Bit instances as arguments. The least significant bit is `bit_0`
    /// and the most significant bit is `bit_7`.
    ///
    /// Note that the Bit instances are stored in reverse (LSB to MSB) order,
    /// but the constructor takes them in the correct order (MSB to LSB) to provide a predictable and intuitive interface.
    ///
    /// # Arguments
    ///
    /// * `zeroth` - The value of the most significant bit.
    /// * `first` - The value of the second most significant bit.
    /// * `second` - The value of the third most significant bit.
    /// * `third` - The value of the fourth most significant bit.
    /// * `fourth` - The value of the fifth most significant bit.
    /// * `fifth` - The value of the sixth most significant bit.
    /// * `sixth` - The value of the seventh most significant bit.
    /// * `seventh` - The value of the least significant bit.
    ///
    /// # Examples
    ///
    /// ## Single Digit
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte_single = Byte::new(Bit::zero(), Bit::zero(), Bit::zero(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero());
    /// assert_eq!(byte_single.to_u8(), 0b00001010);
    /// assert_eq!(byte_single.to_string(), "0x0A");
    /// ```
    /// ## Double Digit
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte_double = Byte::new(Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero());
    /// assert_eq!(byte_double.to_u8(), 0b10101010);
    /// assert_eq!(byte_double.to_string(), "0xAA");
    /// ```
    ///
    /// # Returns
    ///
    /// A Byte containing the specified Bit values.
    ///
    /// # See Also
    ///
    /// [`from_u8()`](#method.from_u8)
    /// [`from_nybbles()`](#method.from_nybbles)
    ///
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        zeroth: Bit,
        first: Bit,
        second: Bit,
        third: Bit,
        fourth: Bit,
        fifth: Bit,
        sixth: Bit,
        seventh: Bit,
    ) -> Self {
        Self {
            bit_0: seventh, // Least significant bit
            bit_1: sixth,
            bit_2: fifth,
            bit_3: fourth, // Low Nybble from here on Up
            bit_4: third,  // High Nybble from here on Down
            bit_5: second,
            bit_6: first,
            bit_7: zeroth, // Most Significant Bit
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
    ///
    /// let byte_single = Byte::from_u8(5);
    /// assert_eq!(byte_single.to_u8(), 0b00000101); // 5
    /// assert_eq!(byte_single.to_string(), "0x05");
    /// ```
    /// ## Double Digits
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let byte_double = Byte::from_u8(85);
    /// assert_eq!(byte_double.to_u8(), 0b01010101); // 85
    /// assert_eq!(byte_double.to_string(), "0x55");
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
    /// This method takes two [Nybbles](crate::Nybble) as arguments.
    /// The first Nybble (`bit_7` to `bit_4`) is the High Nybble and
    /// the second Nybble (`bit_3` to `bit_0`) is the Low Nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let byte = Byte::from_nybbles(Nybble::from_u8(0b1011), Nybble::from_u8(0b0101));
    /// assert_eq!(byte.to_u8(), 181);
    /// assert_eq!(byte.to_string(), "0xB5");
    /// ```
    ///
    /// # Returns
    /// A Byte containing the value of the two Nybbles.
    ///
    /// # See Also
    ///
    /// [`get_high_nybble()`](#method.get_high_nybble)
    /// [`get_low_nybble()`](#method.get_low_nybble)
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

    /// Gets the High or First Nybble from the Byte.
    /// This method returns a [Nybble](crate::Nybble).
    /// The High Nybble is the first nybble (`bit_7` to `bit_4`).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let byte = Byte::from_u8(0b01010101);
    /// let high_nybble = byte.get_high_nybble();
    /// assert_eq!(high_nybble.to_u8(), 5);
    /// assert_eq!(high_nybble.to_string(), "0x5");
    /// ```
    ///
    /// # Returns
    ///
    /// A Nybble representing the High Nybble of the Byte.
    ///
    /// # See Also
    ///
    /// [`get_low_nybble()`](#method.get_low_nybble)
    /// [`from_nybbles()`](#method.from_nybbles)
    ///
    #[must_use]
    pub fn get_high_nybble(&self) -> Nybble {
        let mut nybble = Nybble::default();

        for i in 0..4 {
            if self.get_bit(i + 4) == Bit::One {
                nybble.set_bit(i);
            }
        }

        nybble
    }

    /// Gets the Low or Second Nybble from the Byte.
    /// This method returns a Nybble.
    /// The Low Nybble is the second nybble (`bit_3` to `bit_0`).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let byte = Byte::from_u8(0b01010101);
    /// let low_nybble = byte.get_low_nybble();
    /// assert_eq!(low_nybble.to_u8(), 5);
    /// assert_eq!(low_nybble.to_string(), "0x5");
    /// ```
    ///
    /// # Returns
    ///
    /// A Nybble containing the least significant nybble of the Byte.
    ///
    /// # See Also
    ///
    /// [`get_high_nybble()`](#method.get_high_nybble)
    /// [`from_nybbles()`](#method.from_nybbles)
    ///
    #[must_use]
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
    /// This method is used "Set" the bit value at a given index.
    /// This means that that bit value is set to 1.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 7.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit to set.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
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
    ///
    /// # Side Effects
    ///
    /// This method will flip the Bit value at the specified index.
    ///
    /// # See Also
    ///
    /// [`unset_bit()`](#method.unset_bit)
    /// [`flip_bit()`](#method.flip_bit)
    /// [`get_bit()`](#method.get_bit)
    ///
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
    /// This method is used "Unset" the bit value at a given index.
    /// This means that that bit value is set to 0.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 7.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit to unset.
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
    ///
    /// # Side Effects
    ///
    /// This method will [unset](crate::Bit#method.unset) the Bit value at the specified index.
    ///
    /// # See Also
    ///
    /// [`set_bit()`](#method.set_bit)
    /// [`flip_bit()`](#method.flip_bit)
    /// [`get_bit()`](#method.get_bit)
    ///
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
    /// This method returns the value of the Byte as an 8-bit unsigned integer (u8).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte = Byte::new(Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero());
    /// assert_eq!(byte.to_u8(), 170);
    /// assert_eq!(byte.to_string(), "0xAA");
    /// ```
    ///
    /// # Returns
    ///
    /// An 8-bit unsigned integer (u8) containing the value of the Byte.
    ///
    /// # See Also
    ///
    /// [`to_string()`](#method.to_string)
    /// [`from_u8()`](#method.from_u8)
    ///
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
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 7.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit to get.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let byte = Byte::new(Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero(), Bit::one(), Bit::zero());
    /// assert_eq!(byte.get_bit(0), Bit::Zero);
    /// assert_eq!(byte.get_bit(1), Bit::One);
    /// assert_eq!(byte.get_bit(2), Bit::Zero);
    /// assert_eq!(byte.get_bit(3), Bit::One);
    /// assert_eq!(byte.get_bit(4), Bit::Zero);
    /// assert_eq!(byte.get_bit(5), Bit::One);
    /// assert_eq!(byte.get_bit(6), Bit::Zero);
    /// assert_eq!(byte.get_bit(7), Bit::One);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # Returns
    ///
    /// A [Bit](crate::Bit) containing the value of the Bit at the specified index.
    ///
    /// # See Also
    ///
    /// [`set_bit()`](#method.set_bit)
    /// [`unset_bit()`](#method.unset_bit)
    /// [`flip_bit()`](#method.flip_bit)
    ///
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
    /// This method is used to flip the bit value at a given index.
    /// This means that that bit value is set to the opposite of its current value.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and the most significant bit is at index 7.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit to flip.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
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
    /// # Side Effects
    ///
    /// This method will [flip](crate::Bit#method.flip) the Bit value at the specified index.
    ///
    /// # See Also
    ///
    /// [`set_bit()`](#method.set_bit)
    /// [`unset_bit()`](#method.unset_bit)
    /// [`get_bit()`](#method.get_bit)
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
    ///
    /// # Side Effects
    ///
    /// This method will [flip](crate::Bit#method.flip) all of the Bit values in the Byte.
    ///
    /// # See Also
    ///
    /// [`flip_bit()`](#method.flip_bit)
    ///
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
    /// Converts the Byte to a String.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let byte = Byte::from_u8(170);
    /// assert_eq!(byte.to_string(), "0xAA");
    /// ```
    ///
    /// # Returns
    ///
    /// A String containing the value of the Byte.
    ///
    /// # See Also
    ///
    /// [`to_u8()`](#method.to_u8)
    /// [`from_u8()`](#method.from_u8)
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
    // The return type is Byte because the Not operation is in-place.
    type Output = Self;

    /// Performs the Not operation on the Byte.
    ///
    /// This method is used to perform the Not operation on the Byte.
    /// This also allows the use of the `!` operator on the Byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let mut byte = Byte::from_u8(170);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    /// assert_eq!(byte.to_string(), "0xAA");
    ///
    /// byte = !byte;
    ///
    /// assert_eq!(byte.to_u8(), 85);
    /// assert_eq!(byte.to_string(), "0x55");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method inverts the Bit values in the Byte.
    ///
    /// # See Also
    ///
    /// [`flip()`](#method.flip)
    ///
    fn not(self) -> Self::Output {
        let mut byte = self;
        byte.flip();
        byte
    }
}

impl BitAnd for Byte {
    // The return type is Byte because the BitAnd operation is symmetric.
    type Output = Self;

    /// Performs the Bitwise And operation on the Byte.
    ///
    /// This method is used to perform the Bitwise And operation on the Byte.
    /// This also allows the use of the `&` operator on the Byte.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the BitAnd operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let mut byte = Byte::from_u8(170);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    /// assert_eq!(byte.to_string(), "0xAA");
    ///
    /// byte = byte & Byte::from_u8(85);
    ///
    /// assert_eq!(byte.to_u8(), 0);
    /// assert_eq!(byte.to_string(), "0x00");
    /// ```
    ///
    /// # Returns
    ///
    /// A Byte containing the result of a bitwise AND operation on the two Bytes.
    ///
    /// # See Also
    ///
    /// [`bitor()`](#method.bitor)
    /// [`bitxor()`](#method.bitxor)
    /// [`bitand_assign()`](#method.bitand_assign)
    /// [`bitor_assign()`](#method.bitor_assign)
    /// [`bitxor_assign()`](#method.bitxor_assign)
    ///
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
    /// Performs the Bitwise And Assignment operation on the Byte.
    ///
    /// This method is used to perform the Bitwise And Assignment operation on the Byte.
    /// This also allows the use of the `&=` operator on the Byte.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the BitAnd Assignment operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let mut byte = Byte::from_u8(170);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    ///
    /// byte &= Byte::from_u8(85);
    ///
    /// assert_eq!(byte.to_u8(), 0);
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method performs a bitwise AND operation on the two Bytes, storing the result in the first Byte.
    ///
    /// # See Also
    ///
    /// [`bitand()`](#method.bitand)
    /// [`bitor()`](#method.bitor)
    /// [`bitxor()`](#method.bitxor)
    /// [`bitor_assign()`](#method.bitor_assign)
    /// [`bitxor_assign()`](#method.bitxor_assign)
    ///
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
    // The return type is Byte because the BitOr operation is symmetric.
    type Output = Self;

    /// Performs the Bitwise Or operation on the Byte.
    ///
    /// This method is used to perform the Bitwise Or operation on the Byte.
    /// This also allows the use of the `|` operator on the Byte.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Or operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let mut byte = Byte::from_u8(170);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    /// assert_eq!(byte.to_string(), "0xAA");
    ///
    /// byte = byte | Byte::from_u8(85);
    ///
    /// assert_eq!(byte.to_u8(), 255);
    /// assert_eq!(byte.to_string(), "0xFF");
    /// ```
    ///
    /// # Returns
    ///
    /// A Byte containing the result of a Bitwise Or operation on the two Bytes.
    ///
    /// # See Also
    ///
    /// [`bitand()`](#method.bitand)
    /// [`bitxor()`](#method.bitxor)
    /// [`bitand_assign()`](#method.bitand_assign)
    /// [`bitor_assign()`](#method.bitor_assign)
    /// [`bitxor_assign()`](#method.bitxor_assign)
    ///
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
    /// Performs the Bitwise Or Assignment operation on the Byte.
    ///
    /// This method is used to perform the Bitwise Or Assignment operation on the Byte.
    /// This also allows the use of the `|=` operator on the Byte.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Or Assignment operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let mut byte = Byte::from_u8(170);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    ///
    /// byte |= Byte::from_u8(85);
    ///
    /// assert_eq!(byte.to_u8(), 255);
    ///
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method performs a Bitwise Or operation on the two Bytes, storing the result in the first Byte.
    ///
    /// # See Also
    ///
    /// [`bitand()`](#method.bitand)
    /// [`bitor()`](#method.bitor)
    /// [`bitxor()`](#method.bitxor)
    /// [`bitand_assign()`](#method.bitand_assign)
    /// [`bitxor_assign()`](#method.bitxor_assign)
    ///
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
    // The return type is Byte because the BitXor operation is symmetric.
    type Output = Self;

    /// Performs the Bitwise Xor operation on the Byte.
    ///
    /// This method is used to perform the Bitwise Xor operation on the Byte.
    /// This also allows the use of the `^` operator on the Byte.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Xor operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let mut byte = Byte::from_u8(0b10101010);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    ///
    /// byte = byte ^ Byte::from_u8(85);
    ///
    /// assert_eq!(byte.to_u8(), 255);
    /// ```
    ///
    /// # Returns
    ///
    /// A Byte containing the result of a Bitwise Xor operation on the two Bytes.
    ///
    /// # See Also
    ///
    /// [`bitand()`](#method.bitand)
    /// [`bitor()`](#method.bitor)
    /// [`bitand_assign()`](#method.bitand_assign)
    /// [`bitor_assign()`](#method.bitor_assign)
    /// [`bitxor_assign()`](#method.bitxor_assign)
    ///
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
    /// Performs the Bitwise Xor Assignment operation on the Byte.
    ///
    /// This method is used to perform the Bitwise Xor Assignment operation on the Byte.
    /// This also allows the use of the `^=` operator on the Byte.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Xor Assignment operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Byte;
    ///
    /// let mut byte = Byte::from_u8(0b10101010);
    ///
    /// assert_eq!(byte.to_u8(), 170);
    ///
    /// byte ^= Byte::from_u8(0b01010101);
    ///
    /// assert_eq!(byte.to_u8(), 255);
    ///
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method performs a Bitwise Xor operation on the two Bytes, storing the result in the first Byte.
    ///
    /// # See Also
    ///
    /// [`bitand()`](#method.bitand)
    /// [`bitor()`](#method.bitor)
    /// [`bitxor()`](#method.bitxor)
    /// [`bitand_assign()`](#method.bitand_assign)
    /// [`bitor_assign()`](#method.bitor_assign)
    ///
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
    fn test_from_nybbles_ones() {
        let high_nybble = Nybble::from_u8(15);
        let low_nybble = Nybble::from_u8(15);
        let byte = Byte::from_nybbles(high_nybble, low_nybble);
        assert_eq!(byte.to_u8(), 255);
    }

    #[test]
    fn test_from_nybbles_zeros() {
        let high_nybble = Nybble::default();
        let low_nybble = Nybble::default();
        let byte = Byte::from_nybbles(high_nybble, low_nybble);
        assert_eq!(byte.to_u8(), 0);
    }

    #[test]
    fn test_from_nybbles_alternating() {
        let high_nybble = Nybble::from_u8(0b1010);
        let low_nybble = Nybble::from_u8(0b0101);
        let byte = Byte::from_nybbles(high_nybble, low_nybble);
        assert_eq!(byte.to_u8(), 0b10100101);

        let high_nybble = Nybble::from_u8(0b0101);
        let low_nybble = Nybble::from_u8(0b1010);
        let byte = Byte::from_nybbles(high_nybble, low_nybble);
        assert_eq!(byte.to_u8(), 0b01011010);
    }

    #[test]
    fn test_to_u8_all_zeros() {
        let byte = Byte::default();
        assert_eq!(byte.to_u8(), 0b00000000);
    }

    #[test]
    fn test_to_u8_all_ones() {
        let byte = Byte::from_u8(0b11111111);
        assert_eq!(byte.to_u8(), 0b11111111);
    }

    #[test]
    fn test_to_u8_alternating_raw() {
        let byte = Byte::new(
            Bit::one(),
            Bit::zero(),
            Bit::one(),
            Bit::zero(),
            Bit::one(),
            Bit::zero(),
            Bit::one(),
            Bit::zero(),
        );
        assert_eq!(byte.to_u8(), 0b10101010);
    }

    #[test]
    fn test_to_u8_alternating() {
        let byte = Byte::from_u8(0b10101010);
        assert_eq!(byte.to_u8(), 0b10101010);
    }

    #[test]
    fn test_to_u8_random() {
        let byte = Byte::from_u8(0b11001100);
        assert_eq!(byte.to_u8(), 0b11001100);
    }

    #[test]
    fn test_get_high_nybble_all_zeros() {
        let byte = Byte::default();
        let nybble = byte.get_high_nybble();
        assert_eq!(nybble.to_u8(), 0b0000);
    }

    #[test]
    fn test_get_high_nybble_all_ones() {
        let byte = Byte::from_u8(255);
        let nybble = byte.get_high_nybble();
        assert_eq!(nybble.to_u8(), 15);
    }

    #[test]
    fn test_get_high_nybble_alternating() {
        let byte = Byte::from_u8(0b10101010);
        let nybble = byte.get_high_nybble();
        assert_eq!(nybble.to_u8(), 0b1010);
    }

    #[test]
    fn test_get_high_nybble_random() {
        let byte = Byte::from_u8(0b11001100);
        let nybble = byte.get_high_nybble();
        assert_eq!(nybble.to_u8(), 0b1100);
    }

    #[test]
    fn test_get_low_nybble_all_zeros() {
        let byte = Byte::default();
        let nybble = byte.get_low_nybble();
        assert_eq!(nybble.to_u8(), 0b0000);
    }

    #[test]
    fn test_get_low_nybble_all_ones() {
        let byte = Byte::from_u8(255);
        let nybble = byte.get_low_nybble();
        assert_eq!(nybble.to_u8(), 15);
    }

    #[test]
    fn test_get_low_nybble_alternating() {
        let byte = Byte::from_u8(0b10101010);
        let nybble = byte.get_low_nybble();
        assert_eq!(nybble.to_u8(), 0b1010);
    }

    #[test]
    fn test_get_low_nybble_random() {
        let byte = Byte::from_u8(0b11001100);
        let nybble = byte.get_low_nybble();
        assert_eq!(nybble.to_u8(), 0b1100);
    }

    #[test]
    fn test_set_bit_valid() {
        let mut byte = Byte::default();
        byte.set_bit(0);
        assert_eq!(byte.to_u8(), 1);

        let mut byte = Byte::default();
        byte.set_bit(1);
        assert_eq!(byte.to_u8(), 2);

        let mut byte = Byte::default();
        byte.set_bit(2);
        assert_eq!(byte.to_u8(), 4);

        let mut byte = Byte::default();
        byte.set_bit(3);
        assert_eq!(byte.to_u8(), 0b00001000);

        let mut byte = Byte::default();
        byte.set_bit(4);
        assert_eq!(byte.to_u8(), 0b00010000);

        let mut byte = Byte::default();
        byte.set_bit(5);
        assert_eq!(byte.to_u8(), 0b00100000);

        let mut byte = Byte::default();
        byte.set_bit(6);
        assert_eq!(byte.to_u8(), 0b01000000);

        let mut byte = Byte::default();
        byte.set_bit(7);
        assert_eq!(byte.to_u8(), 0b10000000);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_set_bit_out_of_bounds() {
        let mut byte = Byte::default();
        byte.set_bit(8);
    }

    #[test]
    fn test_unset_bit_valid() {
        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(0);
        assert_eq!(byte.to_u8(), 0b11111110);

        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(1);
        assert_eq!(byte.to_u8(), 0b11111101);

        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(2);
        assert_eq!(byte.to_u8(), 0b11111011);

        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(3);
        assert_eq!(byte.to_u8(), 0b11110111);

        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(4);
        assert_eq!(byte.to_u8(), 0b11101111);

        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(5);
        assert_eq!(byte.to_u8(), 0b11011111);

        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(6);
        assert_eq!(byte.to_u8(), 0b10111111);

        let mut byte = Byte::from_u8(0b11111111);
        byte.unset_bit(7);
        assert_eq!(byte.to_u8(), 0b01111111);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_unset_bit_out_of_bounds() {
        let mut byte = Byte::from_u8(0b00000000);
        byte.unset_bit(8);
    }

    #[test]
    fn test_get_bit_valid() {
        let byte = Byte::from_u8(0b01010101);
        assert_eq!(byte.get_bit(0), Bit::One);
        assert_eq!(byte.get_bit(1), Bit::Zero);
        assert_eq!(byte.get_bit(2), Bit::One);
        assert_eq!(byte.get_bit(3), Bit::Zero);
        assert_eq!(byte.get_bit(4), Bit::One);
        assert_eq!(byte.get_bit(5), Bit::Zero);
        assert_eq!(byte.get_bit(6), Bit::One);
        assert_eq!(byte.get_bit(7), Bit::Zero);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_bit_out_of_bounds() {
        let byte = Byte::from_u8(0b00000000);
        let _ = byte.get_bit(8);
    }

    #[test]
    fn test_flip_bit_valid() {
        let mut byte = Byte::from_u8(0b01010101);
        byte.flip_bit(0);
        assert_eq!(byte.to_u8(), 0b01010100);
        byte.flip_bit(1);
        assert_eq!(byte.to_u8(), 0b01010110);
        byte.flip_bit(2);
        assert_eq!(byte.to_u8(), 0b01010010);
        byte.flip_bit(3);
        assert_eq!(byte.to_u8(), 0b01011010);
        byte.flip_bit(4);
        assert_eq!(byte.to_u8(), 0b01001010);
        byte.flip_bit(5);
        assert_eq!(byte.to_u8(), 0b01101010);
        byte.flip_bit(6);
        assert_eq!(byte.to_u8(), 0b00101010);
        byte.flip_bit(7);
        assert_eq!(byte.to_u8(), 0b10101010);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_flip_bit_out_of_bounds() {
        let mut byte = Byte::from_u8(0b00000000);
        byte.flip_bit(8);
    }

    #[test]
    fn test_flip_all_bits() {
        let mut byte = Byte::from_u8(0b11111111);
        byte.flip();
        assert_eq!(byte.to_u8(), 0b00000000);
    }

    #[test]
    fn test_flip_no_bits() {
        let mut byte = Byte::from_u8(0b00000000);
        byte.flip();
        assert_eq!(byte.to_u8(), 0b11111111);
    }

    #[test]
    fn test_flip_odd_bits() {
        let mut byte = Byte::from_u8(0b01010101);
        byte.flip();
        assert_eq!(byte.to_u8(), 0b10101010);
    }

    #[test]
    fn test_flip_even_bits() {
        let mut byte = Byte::from_u8(0b10101010);
        byte.flip();
        assert_eq!(byte.to_u8(), 0b01010101);
    }

    #[test]
    fn test_flip_alternating_bits() {
        let mut byte = Byte::from_u8(0b01010101);
        byte.flip();
        assert_eq!(byte.to_u8(), 0b10101010);
        byte.flip();
        assert_eq!(byte.to_u8(), 0b01010101);
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
