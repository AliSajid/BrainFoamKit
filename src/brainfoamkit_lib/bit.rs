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
/// Additionally, the the variants are not public and can only be accessed through the `Bit::zero()` and `Bit::one()` constructor functions.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::Bit;
///
/// let bit = Bit::zero();
/// assert_eq!(bit, Bit::Zero);
/// let bit = Bit::one();
/// assert_eq!(bit, Bit::One);
/// ```
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
/// ```
/// use brainfoamkit_lib::Bit;
///
/// let bit = Bit::zero();
/// assert_eq!(format!("{}", bit), "0");
/// let bit = Bit::one();
/// assert_eq!(format!("{}", bit), "1");
/// ```
///
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
    #[must_use]
    pub const fn zero() -> Self {
        Self::Zero
    }

    /// Constructs a new Bit with the value 1.
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
    #[must_use]
    pub const fn one() -> Self {
        Self::One
    }

    /// Flips the value of the Bit.
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
    #[must_use]
    pub const fn to_u8(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }

    /// Set the bit
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
    /// * [`Bit::flip()`](#method.flip)
    /// * [`Bit::unset()`](#method.unset)
    /// * [`Bit::is_set()`](#method.is_set)
    /// * [`Bit::is_unset()`](#method.is_unset)
    pub fn set(&mut self) {
        *self = Self::One;
    }

    /// Unset the bit
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
    /// * [`Bit::flip()`](#method.flip)
    /// * [`Bit::set()`](#method.set)
    /// * [`Bit::is_set()`](#method.is_set)
    /// * [`Bit::is_unset()`](#method.is_unset)
    ///
    pub fn unset(&mut self) {
        *self = Self::Zero;
    }

    /// Check if the bit is set
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
    /// * [`Bit::is_unset()`](#method.is_unset)
    /// * [`Bit::set()`](#method.set)
    /// * [`Bit::unset()`](#method.unset)
    #[must_use]
    pub fn is_set(&self) -> bool {
        *self == Self::One
    }

    /// Check if the bit is unset
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
    /// * [`Bit::is_set()`](#method.is_set)
    /// * [`Bit::set()`](#method.set)
    /// * [`Bit::unset()`](#method.unset)
    #[must_use]
    pub fn is_unset(&self) -> bool {
        *self == Self::Zero
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::One => write!(f, "1"),
        }
    }
}

impl Default for Bit {
    fn default() -> Self {
        Self::zero()
    }
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, Self::Zero) => Self::Zero,
            _ => Self::One,
        }
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::One, Self::One) => Self::One,
            _ => Self::Zero,
        }
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, Self::One) | (Self::One, Self::Zero) => Self::One,
            _ => Self::Zero,
        }
    }
}

impl BitXorAssign for Bit {
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
