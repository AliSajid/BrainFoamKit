// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
    Bit,
    Byte,
};

/// An iterator over a byte
///
/// This struct is a wrapper struct to generate an iterator
/// for `Byte`. This allows us to map and/or loop over all the `Bit`s
/// in the `Byte`.
///
/// This iterator iterates from the Least Significant Bit (LSB) to the
/// Most Significant Bit (MSB). This means that the first `Bit` returned
/// is the last `Bit` in the `Byte` and the last `Bit` returned is the
/// first `Bit` in the `Byte`.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::{
///     Bit,
///     Byte,
///     IterableByte,
/// };
///
/// let byte = Byte::from(0b1100_1010); // Dec: 10; Hex: 0xA; Oct: 0o12
/// let mut iter = IterableByte::new(&byte);
///
/// assert_eq!(iter.next(), Some(Bit::zero()));
/// assert_eq!(iter.next(), Some(Bit::one()));
/// assert_eq!(iter.next(), Some(Bit::zero()));
/// assert_eq!(iter.next(), Some(Bit::one()));
/// assert_eq!(iter.next(), Some(Bit::zero()));
/// assert_eq!(iter.next(), Some(Bit::zero()));
/// assert_eq!(iter.next(), Some(Bit::one()));
/// assert_eq!(iter.next(), Some(Bit::one()));
/// assert_eq!(iter.next(), None);
/// ```
///
/// # See Also
///
/// * [`Byte`](crate::Byte)
/// * [`Bit`](crate::Bit)
/// * [`Nybble`](crate::Nybble)
/// * [`IterableNybble`](crate::IterableNybble)
pub struct IterableByte<'a> {
    byte:          &'a Byte,
    current_index: u8,
}

impl<'a> IterableByte<'a> {
    /// Create a new `IterableByte` from a reference to a `Byte`
    ///
    /// # Arguments
    ///
    /// * `byte` - A reference to a `Byte` to iterate over
    ///
    /// # Returns
    ///
    /// A new `IterableNybble` that can be used to iterate over the `Nybble`
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Byte,
    ///     IterableByte,
    /// };
    ///
    /// let byte = Byte::from(0b1100_1010); // Dec: 202; Hex: 0xCA; Oct: 0o312
    /// let mut iter = IterableByte::new(&byte);
    ///
    /// for bit in iter {
    ///     println!("{}", bit);
    /// }
    /// ```
    #[must_use]
    pub const fn new(byte: &'a Byte) -> Self {
        Self {
            byte,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for IterableByte<'a> {
    /// The type of the element the iterator returns
    type Item = Bit;

    /// Advance the iterator and return the next element
    ///
    /// # Returns
    ///
    /// The next element in the iterator
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Bit,
    ///     Byte,
    ///     IterableByte,
    /// };
    ///
    /// let byte = Byte::from(0b1100_1010); // Dec: 202; Hex: 0xCA; Oct: 0o312
    ///
    /// let mut iter = IterableByte::new(&byte);
    ///
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.current_index;
        let next_index = current_index + 1;

        if next_index > 8 {
            self.current_index = 0;
            None
        } else {
            self.current_index = next_index;
            Some(self.byte.get_bit(current_index))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterable_nybble() {
        let byte = Byte::from(0b11001010); // Dec: 10; Hex: 0xA; Oct: 0o12
        let mut iter = byte.iter();

        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), None);
    }
}
