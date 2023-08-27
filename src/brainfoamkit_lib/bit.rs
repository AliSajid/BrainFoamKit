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

use std::{
    fmt::{self, Display, Formatter},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// Representation of a single bit.
///
/// This Enum is the most basic building block of the `BrainfoamKit` library.
/// This encodes a single bit, which can be either a 0 or a 1.
/// I have implemented this as an Enum to ensure that the only possible values are 0 and 1.
/// Additionally, the variants are not public and can only be accessed through the `Bit::zero()` and `Bit::one()` constructor functions.
///
/// # Examples
///
/// ## Create with helper functions
///
/// ```
/// use brainfoamkit_lib::Bit;
///
/// let bit = Bit::zero();
/// assert_eq!(bit, Bit::Zero);
/// let bit = Bit::one();
/// assert_eq!(bit, Bit::One);
/// ```
/// ## Perform basic operations
///
/// ```
/// use brainfoamkit_lib::Bit;
///
/// let mut bit = Bit::zero();
/// bit.flip();
/// assert_eq!(bit, Bit::One);
/// let mut bit = Bit::one();
/// bit.flip();
/// assert_eq!(bit, Bit::Zero);
/// ```
///
/// ## Display the value
///
/// ```
/// use brainfoamkit_lib::Bit;
///
/// let bit = Bit::zero();
/// assert_eq!(format!("{}", bit), "0");
/// let bit = Bit::one();
/// assert_eq!(format!("{}", bit), "1");
/// ```
///
/// ## Convert to a u8
///
/// ```
/// use brainfoamkit_lib::Bit;
///
/// let bit = Bit::zero();
/// assert_eq!(bit.to_u8(), 0);
/// let bit = Bit::one();
/// assert_eq!(bit.to_u8(), 1);
/// ```
///
/// ## Perform logical operations
///
/// ```
/// use brainfoamkit_lib::Bit;
///
/// let bit = Bit::zero() & Bit::zero();
/// assert_eq!(bit, Bit::Zero);
/// let bit = Bit::zero() | Bit::one();
/// assert_eq!(bit, Bit::One);
/// let bit = Bit::one() ^ Bit::one();
/// assert_eq!(bit, Bit::Zero);
/// ```
///
/// # See Also
///
/// * [`Nybble`](crate::Nybble): A 4-bit value composed of 4 Bits.
/// * [`Byte`](crate::Byte): An 8-bit value composed of 8 Bits.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bit {
    /// The zero variant of the Bit Enum.
    /// Represents the value 0 or the Off state.
    Zero,
    /// The one variant of the Bit Enum.
    /// Represents the value 1 or the On state.
    One,
}

impl Bit {
    /// Constructs a new Bit with the value 0.
    ///
    /// This function returns a value of the `Bit` Enum with the `Bit::Zero` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::zero();
    /// assert_eq!(bit, Bit::Zero);
    /// assert_eq!(bit.to_string(), "0");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Bit with the `Bit::Zero` variant.
    ///
    /// # See Also
    ///
    /// * [`Bit::one()`](#method.one): Constructs a new Bit with the value 1.
    ///
    #[must_use]
    pub const fn zero() -> Self {
        Self::Zero
    }

    /// Constructs a new Bit with the value 1.
    ///
    /// This function returns a value of the `Bit` Enum with the `Bit::One` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::one();
    /// assert_eq!(bit, Bit::One);
    /// assert_eq!(bit.to_string(), "1");
    /// ```
    /// # Returns
    ///
    /// A new Bit with the `Bit::One` variant.
    ///
    /// # See Also
    ///
    /// * [`Bit::zero()`](#method.zero): Constructs a new Bit with the value 0.
    ///
    #[must_use]
    pub const fn one() -> Self {
        Self::One
    }

    /// Flips the value of the Bit.
    ///
    /// This function flips the value of the Bit.
    /// This means that if the Bit is `Bit::Zero` it will become `Bit::One` and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut bit = Bit::zero();
    /// bit.flip();
    /// assert_eq!(bit, Bit::One);
    /// let mut bit = Bit::one();
    /// bit.flip();
    /// assert_eq!(bit, Bit::Zero);
    /// ```
    ///
    /// # Side Effects
    ///
    /// The value of the Bit is flipped.
    ///
    /// # See Also
    ///
    /// * [`Bit::set()`](#method.set): Sets the value of the Bit to 1.
    /// * [`Bit::unset()`](#method.unset): Sets the value of the Bit to 0.
    /// * [`Bit::is_set()`](#method.is_set): Checks if the value of the Bit is 1.
    /// * [`Bit::is_unset()`](#method.is_unset): Checks if the value of the Bit is 0.
    ///
    pub fn flip(&mut self) {
        *self = match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }

    /// Converts the Bit to a u8.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::zero();
    /// assert_eq!(bit.to_u8(), 0);
    /// let bit = Bit::one();
    /// assert_eq!(bit.to_u8(), 1);
    /// ```
    ///
    /// # Returns
    ///
    /// The value of the Bit as a u8.
    ///
    /// # See Also
    ///
    /// * [`Bit::to_string()`](#method.to_string): Converts the Bit to a string.
    ///
    #[must_use]
    pub const fn to_u8(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }

    /// Set the bit
    ///
    /// This function *sets* the bit.
    /// This means that the value of the bit is set to 1.
    /// This function ignores the current value of the bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut bit = Bit::zero();
    /// bit.set();
    /// assert_eq!(bit, Bit::One);
    /// ```
    ///
    /// # Side Effects
    ///
    /// The value of the Bit is set to 1.
    ///
    /// # Notes
    ///
    /// This is equivalent to calling `bit.flip()` on a `Bit::Zero`.
    ///
    /// # See Also
    ///
    /// * [`Bit::flip()`](#method.flip): Flips the value of the Bit.
    /// * [`Bit::unset()`](#method.unset): Sets the value of the Bit to 0.
    /// * [`Bit::is_set()`](#method.is_set): Checks if the value of the Bit is 1.
    /// * [`Bit::is_unset()`](#method.is_unset): Checks if the value of the Bit is 0.
    ///
    pub fn set(&mut self) {
        *self = Self::One;
    }

    /// Unset the bit
    ///
    /// This function unsets the bit.
    /// This means that the value of the bit is set to 0.
    /// This function ignores the current value of the bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut bit = Bit::one();
    /// bit.unset();
    /// assert_eq!(bit, Bit::Zero);
    /// ```
    ///
    /// # Side Effects
    ///
    /// The value of the Bit is set to 0.
    ///
    /// # Notes
    ///
    /// This is equivalent to calling `bit.flip()` on a `Bit::One`.
    ///
    /// # See Also
    ///
    /// * [`Bit::flip()`](#method.flip): Flips the value of the Bit.
    /// * [`Bit::set()`](#method.set): Sets the value of the Bit to 1.
    /// * [`Bit::is_set()`](#method.is_set): Checks if the value of the Bit is 1.
    /// * [`Bit::is_unset()`](#method.is_unset): Checks if the value of the Bit is 0.
    ///
    pub fn unset(&mut self) {
        *self = Self::Zero;
    }

    /// Check if the bit is set
    ///
    /// This function checks if the bit is set (i.e. has the value of Bit::One).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::one();
    /// assert!(bit.is_set());
    /// ```
    ///
    /// # Returns
    ///
    /// A boolean indicating if the bit is set.
    ///
    /// # See Also
    ///
    /// * [`Bit::is_unset()`](#method.is_unset): Checks if the bit is unset.
    /// * [`Bit::set()`](#method.set): Sets the value of the Bit to 1.
    /// * [`Bit::unset()`](#method.unset): Sets the value of the Bit to 0.
    ///
    #[must_use]
    pub fn is_set(&self) -> bool {
        *self == Self::One
    }

    /// Check if the bit is unset
    ///
    /// This function checks if the bit is unset (i.e. has the value of Bit::Zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::zero();
    /// assert!(bit.is_unset());
    /// ```
    ///
    /// # Returns
    ///
    /// A boolean indicating if the bit is unset.
    ///
    /// # See Also
    ///
    /// * [`Bit::is_set()`](#method.is_set): Checks if the bit is set.
    /// * [`Bit::set()`](#method.set): Sets the value of the Bit to 1.
    /// * [`Bit::unset()`](#method.unset): Sets the value of the Bit to 0.
    ///
    #[must_use]
    pub fn is_unset(&self) -> bool {
        *self == Self::Zero
    }
}

impl Display for Bit {
    /// Display the value of the Bit.
    ///
    /// This function displays the value of the Bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::zero();
    /// assert_eq!(format!("{}", bit), "0");
    ///
    /// let bit = Bit::one();
    /// assert_eq!(format!("{}", bit), "1");
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// A string containing the value of the Bit.
    ///
    /// # See Also
    ///
    /// * [`Bit::to_string()`](#method.to_string): Converts the Bit to a string.
    ///
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
        }
    }
}

impl Default for Bit {
    /// Create a new Bit with the value 0.
    ///
    /// This function returns a new Bit with the value 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::default();
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// A new Bit with the value 0.
    ///
    /// # See Also
    ///
    /// * [`Bit::zero()`](#method.zero): Constructs a new Bit with the value 0.
    /// * [`Bit::one()`](#method.one): Constructs a new Bit with the value 1.
    fn default() -> Self {
        Self::zero()
    }
}

impl Not for Bit {
    // The return type of the `not` function is Bit since the only possible values are 0 and 1.
    type Output = Self;

    /// Perform a logical NOT on the Bit.
    ///
    /// This function performs a logical NOT on the Bit.
    /// This means that if the Bit is `Bit::Zero` it will become `Bit::One` and vice versa.
    /// This function also allows you to use the `!` operator on the Bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = !Bit::zero();
    /// assert_eq!(bit, Bit::One);
    ///
    /// let bit = !Bit::one();
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// A new Bit with the value of the logical NOT of the Bit.
    ///
    /// # See Also
    ///
    /// * [`Bit::flip()`](#method.flip): Flips the value of the Bit.
    /// * [`Bit::set()`](#method.set): Sets the value of the Bit to 1.
    /// * [`Bit::unset()`](#method.unset): Sets the value of the Bit to 0.
    /// * [`Bit::bitand()`](#method.bitand): Performs a logical AND on the Bit.
    /// * [`Bit::bitor()`](#method.bitor): Performs a logical OR on the Bit.
    /// * [`Bit::bitxor()`](#method.bitxor): Performs a logical XOR on the Bit.
    /// * [`Bit::bitandassign()`](#method.bitandassign): Performs a logical AND on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitorassign()`](#method.bitorassign): Performs a logical OR on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitxorassign()`](#method.bitxorassign): Performs a logical XOR on the Bit and assigns the result to the Bit.
    ///
    fn not(self) -> Self::Output {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
}

impl BitOr for Bit {
    // The return type of the `bitor` function is a Bit since the `Or` operation is symmetric.
    type Output = Self;

    /// Perform a logical OR on the Bit.
    ///
    /// This function performs a logical OR on the Bit.
    /// This means that if either of the Bits is `Bit::One` the result will be `Bit::One`, otherwise the result will be `Bit::Zero`.
    /// This function also allows you to use the `|` operator on the Bit.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other Bit to perform the logical OR with.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::zero() | Bit::zero();
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// let bit = Bit::zero() | Bit::one();
    /// assert_eq!(bit, Bit::One);
    ///
    /// let bit = Bit::one() | Bit::zero();
    /// assert_eq!(bit, Bit::One);
    ///
    /// let bit = Bit::one() | Bit::one();
    /// assert_eq!(bit, Bit::One);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// A new Bit with the value of the logical OR of the two Bits.
    ///
    /// # See Also
    ///
    /// * [`Bit::not()`](#method.not): Performs a logical NOT on the Bit.
    /// * [`Bit::bitand()`](#method.bitand): Performs a logical AND on the Bit.
    /// * [`Bit::bitxor()`](#method.bitxor): Performs a logical XOR on the Bit.
    /// * [`Bit::bitandassign()`](#method.bitandassign): Performs a logical AND on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitorassign()`](#method.bitorassign): Performs a logical OR on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitxorassign()`](#method.bitxorassign): Performs a logical XOR on the Bit and assigns the result to the Bit.
    ///
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, Self::Zero) => Self::Zero,
            _ => Self::One,
        }
    }
}

impl BitOrAssign for Bit {
    /// Perform a logical OR on the Bit and assign the result to the Bit.
    ///
    /// This function performs a logical OR on the Bit and assigns the result to the Bit.
    /// This also allows you to use the `|=` operator on the Bit.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other Bit to perform the logical OR with.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut bit = Bit::zero();
    ///
    /// bit |= Bit::zero();
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// bit |= Bit::one();
    /// assert_eq!(bit, Bit::One);
    ///
    /// bit |= Bit::zero();
    ///
    /// assert_eq!(bit, Bit::One);
    ///
    /// ```
    ///
    /// # Side Effects
    ///
    /// The value of the Bit is updated to the result of the logical OR.
    ///
    /// # See Also
    ///
    /// * [`Bit::not()`](#method.not): Performs a logical NOT on the Bit.
    /// * [`Bit::bitand()`](#method.bitand): Performs a logical AND on the Bit.
    /// * [`Bit::bitxor()`](#method.bitxor): Performs a logical XOR on the Bit.
    /// * [`Bit::bitandassign()`](#method.bitandassign): Performs a logical AND on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitorassign()`](#method.bitorassign): Performs a logical OR on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitxorassign()`](#method.bitxorassign): Performs a logical XOR on the Bit and assigns the result to the Bit.
    ///
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for Bit {
    // The return type of the `bitand` function is a Bit since the `And` operation is symmetric.
    type Output = Self;

    /// Perform a logical AND on the Bit.
    ///
    /// This function performs a logical AND on the Bit.
    /// This means that if both of the Bits are `Bit::One` the result will be `Bit::One`, otherwise the result will be `Bit::Zero`.
    /// This function also allows you to use the `&` operator on the Bit.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other Bit to perform the logical AND with.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::zero() & Bit::zero();
    ///
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// let bit = Bit::zero() & Bit::one();
    ///
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// let bit = Bit::one() & Bit::zero();
    ///
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// let bit = Bit::one() & Bit::one();
    ///
    /// assert_eq!(bit, Bit::One);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// A new Bit with the value of the logical AND of the two Bits.
    ///
    /// # See Also
    ///
    /// * [`Bit::not()`](#method.not): Performs a logical NOT on the Bit.
    /// * [`Bit::bitor()`](#method.bitor): Performs a logical OR on the Bit.
    /// * [`Bit::bitxor()`](#method.bitxor): Performs a logical XOR on the Bit.
    /// * [`Bit::bitandassign()`](#method.bitandassign): Performs a logical AND on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitorassign()`](#method.bitorassign): Performs a logical OR on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitxorassign()`](#method.bitxorassign): Performs a logical XOR on the Bit and assigns the result to the Bit.
    ///
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::One, Self::One) => Self::One,
            _ => Self::Zero,
        }
    }
}

impl BitAndAssign for Bit {
    /// Perform a logical AND on the Bit and assign the result to the Bit.
    ///
    /// This function performs a logical AND on the Bit and assigns the result to the Bit.
    /// This also allows you to use the `&=` operator on the Bit.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other Bit to perform the logical AND with.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut bit = Bit::zero();
    ///
    /// bit &= Bit::zero();
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// ```
    ///
    /// # Side Effects
    ///
    /// The value of the Bit is updated to the result of the logical AND.
    ///
    /// # See Also
    ///
    /// * [`Bit::not()`](#method.not): Performs a logical NOT on the Bit.
    /// * [`Bit::bitor()`](#method.bitor): Performs a logical OR on the Bit.
    /// * [`Bit::bitxor()`](#method.bitxor): Performs a logical XOR on the Bit.
    /// * [`Bit::bitandassign()`](#method.bitandassign): Performs a logical AND on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitorassign()`](#method.bitorassign): Performs a logical OR on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitxorassign()`](#method.bitxorassign): Performs a logical XOR on the Bit and assigns the result to the Bit.
    ///
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitXor for Bit {
    // The return type of the `bitxor` function is a Bit since the `Xor` operation is symmetric.
    type Output = Self;

    /// Perform a logical XOR on the Bit.
    ///
    /// This function performs a logical XOR on the Bit.
    /// This means that if the Bits are different the result will be `Bit::One`, otherwise the result will be `Bit::Zero`.
    /// This function also allows you to use the `^` operator on the Bit.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other Bit to perform the logical XOR with.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let bit = Bit::zero() ^ Bit::zero();
    ///
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// let bit = Bit::zero() ^ Bit::one();
    ///
    /// assert_eq!(bit, Bit::One);
    /// ```
    ///
    /// # Returns
    ///
    /// A new Bit with the value of the logical XOR of the two Bits.
    ///
    /// # See Also
    ///
    /// * [`Bit::not()`](#method.not): Performs a logical NOT on the Bit.
    /// * [`Bit::bitor()`](#method.bitor): Performs a logical OR on the Bit.
    /// * [`Bit::bitand()`](#method.bitand): Performs a logical AND on the Bit.
    /// * [`Bit::bitandassign()`](#method.bitandassign): Performs a logical AND on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitorassign()`](#method.bitorassign): Performs a logical OR on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitxorassign()`](#method.bitxorassign): Performs a logical XOR on the Bit and assigns the result to the Bit.
    ///
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, Self::One) | (Self::One, Self::Zero) => Self::One,
            _ => Self::Zero,
        }
    }
}

impl BitXorAssign for Bit {
    /// Perform a logical XOR on the Bit and assign the result to the Bit.
    ///
    /// This function performs a logical XOR on the Bit and assigns the result to the Bit.
    /// This also allows you to use the `^=` operator on the Bit.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The other Bit to perform the logical XOR with.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut bit = Bit::zero();
    ///
    /// bit ^= Bit::zero();
    ///
    /// assert_eq!(bit, Bit::Zero);
    ///
    /// bit ^= Bit::one();
    ///
    /// assert_eq!(bit, Bit::One);
    /// ```
    ///
    /// # Side Effects
    ///
    /// The value of the Bit is updated to the result of the logical XOR.
    ///
    /// # See Also
    ///
    /// * [`Bit::not()`](#method.not): Performs a logical NOT on the Bit.
    /// * [`Bit::bitor()`](#method.bitor): Performs a logical OR on the Bit.
    /// * [`Bit::bitand()`](#method.bitand): Performs a logical AND on the Bit.
    /// * [`Bit::bitxor()`](#method.bitxor): Performs a logical XOR on the Bit.
    /// * [`Bit::bitandassign()`](#method.bitandassign): Performs a logical AND on the Bit and assigns the result to the Bit.
    /// * [`Bit::bitorassign()`](#method.bitorassign): Performs a logical OR on the Bit and assigns the result to the Bit.
    ///
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let bit = Bit::zero();
        assert_eq!(bit, Bit::Zero);
        assert_eq!(bit.to_string(), "0");
    }

    #[test]
    fn test_one() {
        let bit = Bit::one();
        assert_eq!(bit, Bit::One);
        assert_eq!(bit.to_string(), "1");
    }

    #[test]
    fn test_flip_zero() {
        let mut bit = Bit::zero();
        bit.flip();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_flip_one() {
        let mut bit = Bit::one();
        bit.flip();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_set_zero() {
        let mut bit = Bit::zero();
        bit.set();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_set_one() {
        let mut bit = Bit::one();
        bit.set();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_unset_zero() {
        let mut bit = Bit::zero();
        bit.unset();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_unset_one() {
        let mut bit = Bit::one();
        bit.unset();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_to_u8_zero() {
        let bit = Bit::zero();
        assert_eq!(bit.to_u8(), 0);
    }

    #[test]
    fn test_to_u8_one() {
        let bit = Bit::one();
        assert_eq!(bit.to_u8(), 1);
    }

    #[test]
    fn test_display_zero() {
        let bit = Bit::zero();
        assert_eq!(format!("{}", bit), "0");
    }

    #[test]
    fn test_display_one() {
        let bit = Bit::one();
        assert_eq!(format!("{}", bit), "1");
    }

    #[test]
    fn test_default() {
        let bit = Bit::default();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_not_zero() {
        let bit = !Bit::zero();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_not_one() {
        let bit = !Bit::one();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitor_zero_zero() {
        let bit = Bit::zero() | Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitor_zero_one() {
        let bit = Bit::zero() | Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitor_one_zero() {
        let bit = Bit::one() | Bit::zero();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitor_one_one() {
        let bit = Bit::one() | Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitorassign_zero_zero() {
        let mut bit = Bit::zero();
        bit |= Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitorassign_zero_one() {
        let mut bit = Bit::zero();
        bit |= Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitorassign_one_zero() {
        let mut bit = Bit::one();
        bit |= Bit::zero();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitorassign_one_one() {
        let mut bit = Bit::one();
        bit |= Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitand_zero_zero() {
        let bit = Bit::zero() & Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitand_zero_one() {
        let bit = Bit::zero() & Bit::one();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitand_one_zero() {
        let bit = Bit::one() & Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitand_one_one() {
        let bit = Bit::one() & Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitandassign_zero_zero() {
        let mut bit = Bit::zero();
        bit &= Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitandassign_zero_one() {
        let mut bit = Bit::zero();
        bit &= Bit::one();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitandassign_one_zero() {
        let mut bit = Bit::one();
        bit &= Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitandassign_one_one() {
        let mut bit = Bit::one();
        bit &= Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitxor_zero_zero() {
        let bit = Bit::zero() ^ Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitxor_zero_one() {
        let bit = Bit::zero() ^ Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitxor_one_zero() {
        let bit = Bit::one() ^ Bit::zero();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitxor_one_one() {
        let bit = Bit::one() ^ Bit::one();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitxorassign_zero_zero() {
        let mut bit = Bit::zero();
        bit ^= Bit::zero();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_bitxorassign_zero_one() {
        let mut bit = Bit::zero();
        bit ^= Bit::one();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitxorassign_one_zero() {
        let mut bit = Bit::one();
        bit ^= Bit::zero();
        assert_eq!(bit, Bit::One);
    }

    #[test]
    fn test_bitxorassign_one_one() {
        let mut bit = Bit::one();
        bit ^= Bit::one();
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_is_set_zero() {
        let bit = Bit::zero();
        assert!(!bit.is_set());
    }

    #[test]
    fn test_is_set_one() {
        let bit = Bit::one();
        assert!(bit.is_set());
    }

    #[test]
    fn test_is_unset_zero() {
        let bit = Bit::zero();
        assert!(bit.is_unset());
    }

    #[test]
    fn test_is_unset_one() {
        let bit = Bit::one();
        assert!(!bit.is_unset());
    }

    #[test]
    fn test_is_set_after_set() {
        let mut bit = Bit::zero();
        bit.set();
        assert!(bit.is_set());
    }

    #[test]
    fn test_is_unset_after_set() {
        let mut bit = Bit::zero();
        bit.set();
        assert!(!bit.is_unset());
    }

    #[test]
    fn test_is_set_after_unset() {
        let mut bit = Bit::one();
        bit.unset();
        assert!(!bit.is_set());
    }

    #[test]
    fn test_is_unset_after_unset() {
        let mut bit = Bit::one();
        bit.unset();
        assert!(bit.is_unset());
    }
}
