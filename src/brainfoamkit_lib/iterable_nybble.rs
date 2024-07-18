// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
    Bit,
    Nybble,
};

/// An iterator over a nybble
///
/// This struct is a wrapper struct to generate an iterator
/// for `Nybble`. This allows us to map and/or loop over all the `Bit`s
/// in the `Nybble`.
///
/// This iterator iterates from the Least Significant Bit (LSB) to the
/// Most Significant Bit (MSB). This means that the first `Bit` returned
/// is the last `Bit` in the `Nybble` and the last `Bit` returned is the
/// first `Bit` in the `Nybble`.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::{
///     Bit,
///     IterableNybble,
///     Nybble,
/// };
///
/// let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
/// let mut iter = IterableNybble::new(&nybble);
///
/// assert_eq!(iter.next(), Some(Bit::zero()));
/// assert_eq!(iter.next(), Some(Bit::one()));
/// assert_eq!(iter.next(), Some(Bit::zero()));
/// assert_eq!(iter.next(), Some(Bit::one()));
/// assert_eq!(iter.next(), None);
/// ```
///
/// # See Also
///
/// * [`Nybble`](crate::Nybble)
/// * [`Bit`](crate::Bit)
/// * [`Byte`](crate::Byte)
/// * [`IterableByte`](crate::IterableByte)
pub struct IterableNybble<'a> {
    nybble:        &'a Nybble,
    current_index: u8,
}

impl<'a> IterableNybble<'a> {
    /// Create a new `IterableNybble` from a reference to a `Nybble`
    ///
    /// # Arguments
    ///
    /// * `nybble` - A reference to a `Nybble` to iterate over
    ///
    /// # Returns
    ///
    /// A new `IterableNybble` that can be used to iterate over the `Nybble`
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     IterableNybble,
    ///     Nybble,
    /// };
    ///
    /// let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let mut iter = IterableNybble::new(&nybble);
    ///
    /// for bit in iter {
    ///     println!("{}", bit);
    /// }
    /// ```
    #[must_use]
    pub const fn new(nybble: &'a Nybble) -> Self {
        Self {
            nybble,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for IterableNybble<'a> {
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
    ///     IterableNybble,
    ///     Nybble,
    /// };
    ///
    /// let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    ///
    /// let mut iter = IterableNybble::new(&nybble);
    ///
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.current_index;
        let next_index = current_index + 1;

        if next_index > 4 {
            self.current_index = 0;
            None
        } else {
            self.current_index = next_index;
            Some(self.nybble.get_bit(current_index))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bit;

    #[test]
    fn test_iterable_nybble() {
        let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
        let mut iter = nybble.iter();

        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), None);
    }
}
