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
use crate::Nybble;

/// An iterator over a nybble
///
/// This struct is a wrapper struct to generate an iterator
/// for `Nybble`. This allows us to map and/or loop over all the `BIT`s
/// in the `Nybble`.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::Nybble;
/// use brainfoamkit_lib::IterableNybble;
///
/// let nybble = Nybble::from_u8(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
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
///
pub struct IterableNybble<'a> {
    nybble: &'a Nybble,
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
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::IterableNybble;
    ///
    /// let nybble = Nybble::from_u8(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let mut iter = IterableNybble::new(&nybble);
    ///
    /// for bit in iter {
    ///    println!("{}", bit);
    /// }
    /// ```
    ///
    pub fn new(nybble: &'a Nybble) -> Self {
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
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::IterableNybble;
    ///
    /// let nybble = Nybble::from_u8(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    ///
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), Some(Bit::zero()));
    /// assert_eq!(iter.next(), Some(Bit::one()));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
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

    #[test]
    fn test_iterable_nybble() {
        let nybble = Nybble::from_u8(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
        let mut iter = nybble.iter();

        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), None);
    }
}
