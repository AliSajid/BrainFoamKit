// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    ops::{
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Not,
    },
};

use crate::{
    Bit,
    IterableNybble,
};

/// A Nybble is a 4-bit unsigned integer (u4).
///
/// This is a wrapper around four Bit instances. The least significant bit is
/// `bit_0` and the most significant bit is `bit_3`. This struct is used to
/// conveniently manipulate 4-bit values.
///
/// Note that bit instances are stored in reverse (LSB is first, MSB is last)
/// order, so the least significant bit is `bit_0` and the most significant bit
/// is `bit_3`. However, the [`new`](#method.new) method takes the bits in the
/// correct (MSB is first, LSB is last) order.
///
///
///
/// # Examples
///
/// ## Create a Nybble from primitive Bit values
///
/// ```
/// use brainfoamkit_lib::{
///     Bit,
///     Nybble,
/// };
///
/// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
/// assert_eq!(u8::from(&nybble), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
/// assert_eq!(nybble.to_string(), "0xA");
/// ```
///
/// ## Create a Nybble from a u8 value
///
/// ```
/// use brainfoamkit_lib::Nybble;
///
/// let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
/// assert_eq!(u8::from(&nybble), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
/// assert_eq!(nybble.to_string(), "0xA");
/// ```
///
/// ## Set and Unset bits to generate the desired Nybble
///
/// ```
/// use brainfoamkit_lib::Nybble;
///
/// let mut nybble = Nybble::default();
/// nybble.set_bit(0); // Nybble: 0b0001; Dec: 1; Hex: 0x1; Oct: 0o1
/// nybble.set_bit(2); // Nybble: 0b0101; Dec: 5; Hex: 0x5; Oct: 0o5
///
/// assert_eq!(u8::from(&nybble), 0b0101); // Dec: 5; Hex: 0x5; Oct: 0o5
/// assert_eq!(nybble.to_string(), "0x5");
/// ```
///
/// ## Flip the bits at a given index
///
/// ```
/// use brainfoamkit_lib::Nybble;
///
/// let mut nybble = Nybble::from(0b0101); // Dec: 5; Hex: 0x5; Oct: 0o5
/// nybble.flip_bit(0); // Nybble: 0b0100; Dec: 4; Hex: 0x4; Oct: 0o4
/// nybble.flip_bit(1); // Nybble: 0b0110; Dec: 6; Hex: 0x6; Oct: 0o6
///
/// assert_eq!(u8::from(&nybble), 0b0110); // Dec: 6; Hex: 0x6; Oct: 0o6
/// assert_eq!(nybble.to_string(), "0x6");
/// ```
///
/// # Panics
///
/// The methods [`set_bit()`](#method.set_bit),
/// [`unset_bit()`](#method.unset_bit) and [`get_bit()`](#method.get_bit) will
/// panic if the index is out of bounds.
///
/// # See Also
///
/// * [`Bit`](crate::Bit): The building block of a Nybble.
/// * [`Byte`](crate::Byte): A Byte is a collection of eight Bits.
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
    /// The least significant bit is `bit_0` and the most significant bit is
    /// `bit_3`.
    ///
    /// # Arguments
    ///
    /// * `first` - The most significant bit.
    /// * `second` - The second most significant bit.
    /// * `third` - The second least significant bit.
    /// * `fourth` - The least significant bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Bit,
    ///     Nybble,
    /// };
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero);
    /// assert_eq!(u8::from(&nybble), 10);
    /// assert_eq!(nybble.to_string(), "0xA");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble with the specified Bit values.
    ///
    /// # See Also
    ///
    /// * [`from_u8()`](#method.from_u8): Creates a new Nybble from a u8 value.
    /// * [`default()`](#method.default): Creates a new Nybble with default (all
    ///   [`Bit::Zero`](crate::Bit::Zero)) Bit values.
    #[must_use]
    pub const fn new(first: Bit, second: Bit, third: Bit, fourth: Bit) -> Self {
        Self {
            bit_0: fourth, // Least significant bit
            bit_1: third,
            bit_2: second,
            bit_3: first, // Most Significant Bit
        }
    }

    /// Sets the Bit value at the specified index.
    ///
    /// This method is used "Set" the bit value at a given index.
    /// This means that that bit value is set to 1.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and
    /// the most significant bit is at index 3.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the Bit value to set.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Bit,
    ///     Nybble,
    /// };
    ///
    /// let mut nybble = Nybble::default();
    /// nybble.set_bit(0);
    /// nybble.set_bit(2);
    /// assert_eq!(u8::from(&nybble), 0b0101); // Dec: 5; Hex: 0x5; Oct: 0o5
    /// assert_eq!(nybble.to_string(), "0x5");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will [set](crate::Bit#method.set) the Bit value at the
    /// specified index.
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # See Also
    ///
    /// * [`unset_bit()`](#method.unset_bit): Unsets the Bit value at the
    ///   specified index.
    /// * [`get_bit()`](#method.get_bit): Gets the Bit value at the specified
    ///   index.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified
    ///   index.
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
    /// The index is zero-based, so the least significant bit is at index 0 and
    /// the most significant bit is at index 3.
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
    /// let mut nybble = Nybble::default(); // Nybble: 0b0000; Dec: 0; Hex: 0x0; Oct: 0o0
    /// nybble.set_bit(0); // Nybble: 0b0001; Dec: 1; Hex: 0x1; Oct: 0o1
    /// nybble.set_bit(2); // Nybble: 0b0101; Dec: 5; Hex: 0x5; Oct: 0o5
    /// assert_eq!(u8::from(&nybble), 0b0101); // Dec: 5; Hex: 0x5; Oct: 0o5
    /// assert_eq!(nybble.to_string(), "0x5");
    /// nybble.unset_bit(0); // Nybble: 0b0100; Dec: 4; Hex: 0x4; Oct: 0o4
    /// assert_eq!(u8::from(&nybble), 4); // Dec: 4; Hex: 0x4; Oct: 0o4
    /// assert_eq!(nybble.to_string(), "0x4");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will [unset](crate::Bit#method.unset) the Bit value at the
    /// specified index.
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # See Also
    ///
    /// * [`set_bit()`](#method.set_bit): Sets the Bit value at the specified
    ///   index.
    /// * [`get_bit()`](#method.get_bit): Gets the Bit value at the specified
    ///   index.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified
    ///   index.
    pub fn unset_bit(&mut self, index: u8) {
        match index {
            0 => self.bit_0.unset(),
            1 => self.bit_1.unset(),
            2 => self.bit_2.unset(),
            3 => self.bit_3.unset(),
            _ => panic!("Index out of bounds"),
        }
    }

    /// Get the Bit value at the specified index.
    ///
    /// This method is used to get the bit value at a given index.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and
    /// the most significant bit is at index 3.
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
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero); // Dec: 10; Hex: 0xA; Oct: 0o12
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
    /// * [`set_bit()`](#method.set_bit): Sets the Bit value at the specified
    ///   index.
    /// * [`unset_bit()`](#method.unset_bit): Unsets the Bit value at the
    ///   specified index.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified
    ///   index.
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

    /// Get a reference to the Bit value at the specified index.
    ///
    /// This method is used to get a reference to the bit value at a given
    /// index.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and
    /// the most significant bit is at index 3.
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
    /// let one = Bit::one();
    /// let zero = Bit::zero();
    ///
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble.get_bit_ref(0), &zero);
    /// assert_eq!(nybble.get_bit_ref(1), &one);
    /// assert_eq!(nybble.get_bit_ref(2), &zero);
    /// assert_eq!(nybble.get_bit_ref(3), &one);
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # Returns
    ///
    /// A reference to the Bit value at the specified index.
    ///
    /// # See Also
    ///
    /// * [`set_bit()`](#method.set_bit): Sets the Bit value at the specified
    ///   index.
    /// * [`unset_bit()`](#method.unset_bit): Unsets the Bit value at the
    ///   specified index.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified
    ///   index.
    #[must_use]
    pub fn get_bit_ref(&self, index: u8) -> &Bit {
        match index {
            0 => &self.bit_0,
            1 => &self.bit_1,
            2 => &self.bit_2,
            3 => &self.bit_3,
            _ => panic!("Index out of bounds"),
        }
    }

    /// Flips the Bit value at the specified index.
    ///
    /// This method is used to flip the bit value at a given index.
    /// This means that that bit value is set to the opposite of its current
    /// value.
    ///
    /// The index is zero-based, so the least significant bit is at index 0 and
    /// the most significant bit is at index 3.
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
    /// let mut nybble = Nybble::default(); // Nybble: 0b0000; Dec: 0; Hex: 0x0; Oct: 0o0
    /// nybble.set_bit(0); // Nybble: 0b0001; Dec: 1; Hex: 0x1; Oct: 0o1
    /// nybble.set_bit(2); // Nybble: 0b0101; Dec: 5; Hex: 0x5; Oct: 0o5
    ///
    /// assert_eq!(u8::from(&nybble), 0b0101); // Dec: 5; Hex: 0x5; Oct: 0o5
    /// assert_eq!(nybble.to_string(), "0x5");
    ///
    /// nybble.flip_bit(0); // Nybble: 0b0100; Dec: 4; Hex: 0x4; Oct: 0o4
    /// nybble.flip_bit(1); // Nybble: 0b0110; Dec: 6; Hex: 0x6; Oct: 0o6
    /// nybble.flip_bit(2); // Nybble: 0b0010; Dec: 2; Hex: 0x2; Oct: 0o2
    /// nybble.flip_bit(3); // Nybble: 0b1010; Dec: 10; Hex: 0xA; Oct: 0o12
    ///
    /// assert_eq!(u8::from(&nybble), 10);
    /// assert_eq!(nybble.to_string(), "0xA");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the index is out of bounds.
    ///
    /// # Side Effects
    ///
    /// This method will [flip](crate::Bit#method.flip) the Bit value at the
    /// specified index.
    ///
    /// # See Also
    ///
    /// * [`set_bit()`](#method.set_bit): Sets the Bit value at the specified
    ///   index.
    /// * [`unset_bit()`](#method.unset_bit): Unsets the Bit value at the
    ///   specified index.
    /// * [`get_bit()`](#method.get_bit): Gets the Bit value at the specified
    ///   index.
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
    /// This means that all of the bit values are set to the opposite of their
    /// current values. This can be used to find the two's complement of a
    /// Nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    /// use brainfoamkit_lib::Bit;
    ///
    /// let mut nybble = Nybble::default(); // Nybble: 0b0000; Dec: 0; Hex: 0x0; Oct: 0o0
    ///
    /// nybble.set_bit(0); // Nybble: 0b0001; Dec: 1; Hex: 0x1; Oct: 0o1
    /// nybble.set_bit(2); // Nybble: 0b0101; Dec: 5; Hex: 0x5; Oct: 0o5
    ///
    /// assert_eq!(u8::from(&nybble), 0b0101); // Dec: 5; Hex: 0x5; Oct: 0o5
    /// assert_eq!(nybble.to_string(), "0x5");
    ///
    /// nybble.flip(); // Nybble: 0b1010; Dec: 10; Hex: 0xA; Oct: 0o12
    ///
    /// assert_eq!(u8::from(&nybble), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble.to_string(), "0xA");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will [flip](crate::Bit#method.flip) all of the Bit values in
    /// the Nybble.
    ///
    /// # See Also
    ///
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified
    ///   index.
    pub fn flip(&mut self) {
        self.bit_0.flip();
        self.bit_1.flip();
        self.bit_2.flip();
        self.bit_3.flip();
    }

    /// Increment the Nybble with rollover overflow
    ///
    /// This method increments the value stored in the Nybble.
    /// This has a rollover for overflow. This means that if we increment past
    /// the maximum value (15), we will go back to 0.
    ///
    /// # Examples
    ///
    /// ## Regular Use
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let mut nybble = Nybble::default(); // Nybble: 0b0000; Dec: 0; Hex: 0x0; Oct: 0o0
    ///
    /// nybble.increment();
    /// assert_eq!(u8::from(&nybble), 1);
    /// assert_eq!(nybble.to_string(), "0x1");
    ///
    /// nybble.increment();
    /// assert_eq!(u8::from(&nybble), 2);
    /// assert_eq!(nybble.to_string(), "0x2");
    /// ```
    ///
    /// ## Overflow Use
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let mut nybble = Nybble::from(15); // Nybble: 0b1111; Dec: 15; Hex: 0xF; Oct: 0o17
    ///
    /// nybble.increment();
    /// assert_eq!(u8::from(&nybble), 0);
    /// assert_eq!(nybble.to_string(), "0x0");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method increments the value stored in the Nybble.
    ///
    /// # See Also
    ///
    /// * [`decrement()`](#method.decrement): Decrements the value stored in the
    ///   Nybble.
    /// * [`flip()`](#method.flip): Flips all of the Bit values in the Nybble.
    #[allow(clippy::cast_possible_truncation)]
    pub fn increment(&mut self) {
        // Find the first Bit::Zero from the right
        let zero = self.iter().position(|bit| bit == Bit::Zero);

        if let Some(index) = zero {
            for i in 0..=index as u8 {
                self.flip_bit(i);
            }
        } else {
            self.flip();
        }
    }

    /// Decrement the Nybble with no rollover
    ///
    /// This method decrements the value stored in the Nybble.
    /// This has no rollover for underflow. This means that if we decrement past
    /// the minimum value (0), we will stay at 0.
    ///
    /// # Examples
    ///
    /// ## Regular Use
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let mut nybble = Nybble::from(2); // Nybble: 0b0010; Dec: 2; Hex: 0x2; Oct: 0o2
    ///
    /// nybble.decrement();
    /// assert_eq!(u8::from(&nybble), 1);
    /// assert_eq!(nybble.to_string(), "0x1");
    ///
    /// nybble.decrement();
    /// assert_eq!(u8::from(&nybble), 0);
    /// assert_eq!(nybble.to_string(), "0x0");
    /// ```
    ///
    /// ## Underflow Use
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let mut nybble = Nybble::default(); // Nybble: 0b0000; Dec: 0; Hex: 0x0; Oct: 0o0
    ///
    /// nybble.decrement();
    /// assert_eq!(u8::from(&nybble), 0);
    /// assert_eq!(nybble.to_string(), "0x0");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method decrements the value stored in the Nybble.
    ///
    /// # See Also
    ///
    /// * [`increment()`](#method.increment): Increments the value stored in the
    ///   Nybble.
    /// * [`flip()`](#method.flip): Flips all of the Bit values in the Nybble.
    #[allow(clippy::cast_possible_truncation)]
    pub fn decrement(&mut self) {
        // Find the first Bit::One bit from the right
        let one = self.iter().position(|bit| bit == Bit::One);

        if let Some(index) = one {
            for i in 0..=index as u8 {
                self.flip_bit(i);
            }
        }
    }

    /// Create an iterator over the Nybble.
    /// This allows the use of the `for` loop on the Nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    ///
    /// for bit in nybble.iter() {
    ///     println!("{}", bit);
    /// }
    /// ```
    #[must_use]
    pub const fn iter(&self) -> IterableNybble {
        IterableNybble::new(self)
    }
}

impl Display for Nybble {
    /// Converts the Nybble to a string.
    ///
    /// This method converts the Nybble to a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    ///
    /// assert_eq!(nybble.to_string(), "0xA");
    /// ```
    ///
    /// # Returns
    ///
    /// The Nybble as a string.
    ///
    /// # See Also
    ///
    /// * [`to_u8()`](#method.to_u8): Converts the Nybble to an 8-bit unsigned
    ///   integer (u8).
    /// * [`from_u8()`](#method.from_u8): Creates a new Nybble from a u8 value.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let number: u8 = self.into();
        write!(f, "{number:#03X}")
    }
}

impl Default for Nybble {
    /// Create a _default_ (empty) Nybble.
    ///
    /// Creates a new Nybble with default (all [`Bit::Zero`](crate::Bit::Zero))
    /// Bit values.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble = Nybble::default();
    ///
    /// assert_eq!(u8::from(&nybble), 0b0000); // Nybble: 0b0000; Dec: 0; Hex: 0x0; Oct: 0o0
    ///
    /// assert_eq!(nybble.to_string(), "0x0");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble with default (all [`Bit::Zero`](crate::Bit::Zero)) Bit
    /// values.
    ///
    /// # See Also
    ///
    /// * [`new()`](#method.new): Creates a new Nybble with the specified Bit
    ///   values.
    /// * [`from_u8()`](#method.from_u8): Creates a new Nybble from a u8 value.
    /// * [`to_u8()`](#method.to_u8): Converts the Nybble to an 8-bit unsigned
    ///   integer (u8).
    fn default() -> Self {
        Self::new(Bit::zero(), Bit::zero(), Bit::zero(), Bit::zero())
    }
}

impl From<u8> for Nybble {
    /// Creates a new Nybble from a u8 value.
    ///
    /// This method takes a u8 value as an argument and creates a new Nybble
    /// truncating to only the least significant four bits.
    ///
    /// # Arguments
    ///
    /// * `n` - The u8 value to create the Nybble from.
    ///
    /// # Examples
    ///
    /// ## A valid value for a Nybble
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble = Nybble::from(5);
    /// assert_eq!(u8::from(&nybble), 5);
    /// assert_eq!(nybble.to_string(), "0x5");
    /// ```
    ///
    /// ## A value too large for a Nybble
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble = Nybble::from(16);
    /// assert_eq!(u8::from(&nybble), 0);
    /// assert_eq!(nybble.to_string(), "0x0");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble from the specified u8 value, or an all
    /// [`Bit::One`](crate::Bit::One) Nybble if the value is larger than 15.
    ///
    /// # See Also
    ///
    /// * [`new()`](#method.new): Creates a new Nybble with the specified Bit
    ///   values.
    /// * [`default()`](#method.default): Creates a new Nybble with default (all
    ///   [`Bit::Zero`](crate::Bit::Zero)) Bit values.
    fn from(n: u8) -> Self {
        let n = n & 0b0000_1111;

        // Create a new Nybble instance with default Bit values
        let mut nybble = Self::default();

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
}

impl From<&Nybble> for u8 {
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
    /// let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let result = u8::from(&nybble); // Dec: 10; Hex: 0xA; Oct: 0o12
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
    fn from(nybble: &Nybble) -> Self {
        let mut n = 0;

        for i in 0..4 {
            if nybble.get_bit(i) == Bit::One {
                n |= 1 << i;
            }
        }

        n
    }
}

impl Not for Nybble {
    // The output type of Not is Nybble as the operation is symmetric
    type Output = Self;

    /// Perform the Not operation on the Nybble.
    ///
    /// This method performs the Not operation on the Nybble.
    /// This also allows the use of the `!` operator on the Nybble.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    ///
    /// assert_eq!(u8::from(&nybble), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble.to_string(), "0xA");
    ///
    /// let nybble = !nybble;
    ///
    /// assert_eq!(u8::from(&nybble), 0b0101); // Dec: 5; Hex: 0x5; Oct: 0o5
    /// assert_eq!(nybble.to_string(), "0x5");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble with the Bit values flipped.
    ///
    /// # See Also
    ///
    /// * [`flip()`](#method.flip): Flips all of the Bit values in the Nybble.
    /// * [`flip_bit()`](#method.flip_bit): Flips the Bit value at the specified
    ///   index.
    /// * [`bitand()`](#method.bitand): Performs the Bitwise And operation on
    ///   two Nybbles.
    /// * [`bitor()`](#method.bitor): Performs the Bitwise Or operation on two
    ///   Nybbles.
    /// * [`bitxor()`](#method.bitxor): Performs the Bitwise Xor operation on
    ///   two Nybbles.
    /// * [`bitand_assign()`](#method.bitand_assign): Performs the Bitwise And
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitor_assign()`](#method.bitor_assign): Performs the Bitwise Or
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitxor_assign()`](#method.bitxor_assign): Performs the Bitwise Xor
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    fn not(self) -> Self::Output {
        let mut nybble = self;
        nybble.flip();
        nybble
    }
}

impl BitAnd for Nybble {
    // The output type of BitAnd is Nybble as the operation is symmetric
    type Output = Self;

    /// Perform the Bitwise And operation on two Nybbles.
    ///
    /// This method performs the Bitwise And operation on two Nybbles.
    /// This also allows the use of the `&` operator on two Nybbles.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise And operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble_1 = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let nybble_2 = Nybble::from(0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble_1.to_string(), "0xA");
    ///
    /// assert_eq!(u8::from(&nybble_2), 0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    /// assert_eq!(nybble_2.to_string(), "0xC");
    ///
    /// let nybble = nybble_1 & nybble_2;
    ///
    /// assert_eq!(u8::from(&nybble), 0b1000); // Dec: 8; Hex: 0x8; Oct: 0o10
    /// assert_eq!(nybble.to_string(), "0x8");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble with the Bitwise And operation performed on the two
    /// Nybbles.
    ///
    /// # See Also
    ///
    /// * [`bitor()`](#method.bitor): Performs the Bitwise Or operation on two
    ///   Nybbles.
    /// * [`bitxor()`](#method.bitxor): Performs the Bitwise Xor operation on
    ///   two Nybbles.
    /// * [`bitand_assign()`](#method.bitand_assign): Performs the Bitwise And
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitor_assign()`](#method.bitor_assign): Performs the Bitwise Or
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitxor_assign()`](#method.bitxor_assign): Performs the Bitwise Xor
    ///   operation on two Nybbles and assigns the result to the first Nybble.
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
    /// Perform the Bitwise And operation on two Nybbles and assigns the result
    /// to the first Nybble.
    ///
    /// This method performs the Bitwise And operation on two Nybbles and
    /// assigns the result to the first Nybble.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise And operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let mut nybble_1 = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let nybble_2 = Nybble::from(0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble_1.to_string(), "0xA");
    ///
    /// assert_eq!(u8::from(&nybble_2), 0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    /// assert_eq!(nybble_2.to_string(), "0xC");
    ///
    /// nybble_1 &= nybble_2;
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1000); // Dec: 8; Hex: 0x8; Oct: 0o10
    /// assert_eq!(nybble_1.to_string(), "0x8");
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will perform the Bitwise And operation on two Nybbles and
    /// assign the result to the first Nybble.
    ///
    /// # See Also
    ///
    /// * [`bitand()`](#method.bitand): Performs the Bitwise And operation on
    ///   two Nybbles.
    /// * [`bitor()`](#method.bitor): Performs the Bitwise Or operation on two
    ///   Nybbles.
    /// * [`bitxor()`](#method.bitxor): Performs the Bitwise Xor operation on
    ///   two Nybbles.
    /// * [`bitor_assign()`](#method.bitor_assign): Performs the Bitwise Or
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitxor_assign()`](#method.bitxor_assign): Performs the Bitwise Xor
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    fn bitand_assign(&mut self, rhs: Self) {
        self.bit_0 &= rhs.bit_0;
        self.bit_1 &= rhs.bit_1;
        self.bit_2 &= rhs.bit_2;
        self.bit_3 &= rhs.bit_3;
    }
}

impl BitOr for Nybble {
    // The output type of BitOr is Nybble as the operation is symmetric
    type Output = Self;

    /// Perform the Bitwise Or operation on two Nybbles.
    ///
    /// This method performs the Bitwise Or operation on two Nybbles.
    /// This also allows the use of the `|` operator on two Nybbles.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Or operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble_1 = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let nybble_2 = Nybble::from(0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble_1.to_string(), "0xA");
    ///
    /// assert_eq!(u8::from(&nybble_2), 0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// let nybble = nybble_1 | nybble_2;
    ///
    /// assert_eq!(u8::from(&nybble), 0b1110); // Dec: 14; Hex: 0xE; Oct: 0o16
    /// assert_eq!(nybble.to_string(), "0xE");
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble with the Bitwise Or operation performed on the two Nybbles.
    ///
    /// # See Also
    ///
    /// * [`bitand()`](#method.bitand): Performs the Bitwise And operation on
    ///   two Nybbles.
    /// * [`bitxor()`](#method.bitxor): Performs the Bitwise Xor operation on
    ///   two Nybbles.
    /// * [`bitand_assign()`](#method.bitand_assign): Performs the Bitwise And
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitor_assign()`](#method.bitor_assign): Performs the Bitwise Or
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitxor_assign()`](#method.bitxor_assign): Performs the Bitwise Xor
    ///   operation on two Nybbles and assigns the result to the first Nybble.
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
    /// Perform the Bitwise Or operation on two Nybbles and assigns the result
    /// to the first Nybble.
    ///
    /// This method performs the Bitwise Or operation on two Nybbles and assigns
    /// the result to the first Nybble. This also allows the use of the `|=`
    /// operator on two Nybbles.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Or operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let mut nybble_1 = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let nybble_2 = Nybble::from(0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble_1.to_string(), "0xA");
    ///
    /// assert_eq!(u8::from(&nybble_2), 0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    /// assert_eq!(nybble_2.to_string(), "0xC");
    ///
    /// nybble_1 |= nybble_2;
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1110); // Dec: 14; Hex: 0xE; Oct: 0o16
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will perform the Bitwise Or operation on two Nybbles and
    /// assign the result to the first Nybble.
    ///
    /// # See Also
    ///
    /// * [`bitand()`](#method.bitand): Performs the Bitwise And operation on
    ///   two Nybbles.
    /// * [`bitor()`](#method.bitor): Performs the Bitwise Or operation on two
    ///   Nybbles.
    /// * [`bitxor()`](#method.bitxor): Performs the Bitwise Xor operation on
    ///   two Nybbles.
    /// * [`bitand_assign()`](#method.bitand_assign): Performs the Bitwise And
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitxor_assign()`](#method.bitxor_assign): Performs the Bitwise Xor
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    fn bitor_assign(&mut self, rhs: Self) {
        self.bit_0 |= rhs.bit_0;
        self.bit_1 |= rhs.bit_1;
        self.bit_2 |= rhs.bit_2;
        self.bit_3 |= rhs.bit_3;
    }
}

impl BitXor for Nybble {
    // The output type of BitXor is Nybble as the operation is symmetric
    type Output = Self;

    /// Perform the Bitwise Xor operation on two Nybbles.
    ///
    /// This method performs the Bitwise Xor operation on two Nybbles.
    /// This also allows the use of the `^` operator on two Nybbles.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Xor operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let nybble_1 = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let nybble_2 = Nybble::from(0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble_1.to_string(), "0xA");
    ///
    /// assert_eq!(u8::from(&nybble_2), 0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// let nybble = nybble_1 ^ nybble_2;
    ///
    /// assert_eq!(u8::from(&nybble), 0b0110); // Dec: 6; Hex: 0x6; Oct: 0o6
    /// ```
    ///
    /// # Returns
    ///
    /// A new Nybble with the Bitwise Xor operation performed on the two
    /// Nybbles.
    ///
    /// # See Also
    ///
    /// * [`bitand()`](#method.bitand): Performs the Bitwise And operation on
    ///   two Nybbles.
    /// * [`bitor()`](#method.bitor): Performs the Bitwise Or operation on two
    ///   Nybbles.
    /// * [`bitand_assign()`](#method.bitand_assign): Performs the Bitwise And
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitor_assign()`](#method.bitor_assign): Performs the Bitwise Or
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitxor_assign()`](#method.bitxor_assign): Performs the Bitwise Xor
    ///   operation on two Nybbles and assigns the result to the first Nybble.
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
    /// Perform the Bitwise Xor operation on two Nybbles and assigns the result
    /// to the first Nybble.
    ///
    /// This method performs the Bitwise Xor operation on two Nybbles and
    /// assigns the result to the first Nybble. This also allows the use of
    /// the `^=` operator on two Nybbles.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the Bitwise Xor operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Nybble;
    ///
    /// let mut nybble_1 = Nybble::from(0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// let nybble_2 = Nybble::from(0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b1010); // Dec: 10; Hex: 0xA; Oct: 0o12
    /// assert_eq!(nybble_1.to_string(), "0xA");
    ///
    /// assert_eq!(u8::from(&nybble_2), 0b1100); // Dec: 12; Hex: 0xC; Oct: 0o14
    /// assert_eq!(nybble_2.to_string(), "0xC");
    ///
    /// nybble_1 ^= nybble_2;
    ///
    /// assert_eq!(u8::from(&nybble_1), 0b0110); // Dec: 6; Hex: 0x6; Oct: 0o6
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will perform the Bitwise Xor operation on two Nybbles and
    /// assign the result to the first Nybble.
    ///
    /// # See Also
    ///
    /// * [`bitand()`](#method.bitand): Performs the Bitwise And operation on
    ///   two Nybbles.
    /// * [`bitor()`](#method.bitor): Performs the Bitwise Or operation on two
    ///   Nybbles.
    /// * [`bitxor()`](#method.bitxor): Performs the Bitwise Xor operation on
    ///   two Nybbles.
    /// * [`bitand_assign()`](#method.bitand_assign): Performs the Bitwise And
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    /// * [`bitor_assign()`](#method.bitor_assign): Performs the Bitwise Or
    ///   operation on two Nybbles and assigns the result to the first Nybble.
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bit_0 ^= rhs.bit_0;
        self.bit_1 ^= rhs.bit_1;
        self.bit_2 ^= rhs.bit_2;
        self.bit_3 ^= rhs.bit_3;
    }
}

/// `IntoIterator` implementation for a reference to a `Nybble`.
///
/// This implementation allows a `Nybble` reference to be converted into an
/// iterator. The iterator will yield `Bit` items.
impl<'a> IntoIterator for &'a Nybble {
    /// The type of the iterator that will be returned. It's an `IterableNybble`
    /// with the same lifetime as the `Nybble` reference.
    type IntoIter = IterableNybble<'a>;
    /// The type of the items that will be returned when iterating over the
    /// `Nybble` reference. In this case, it's a `Bit`.
    type Item = Bit;

    /// Converts the `Nybble` reference into an `IterableNybble` iterator.
    ///
    /// # Returns
    ///
    /// An `IterableNybble` iterator with the same lifetime as the `Nybble`
    /// reference.
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let nybble = Nybble::from(10);
        assert_eq!(u8::from(&nybble), 0b1010);
    }

    #[test]
    fn test_from_u8_zero() {
        let nybble = Nybble::from(0);
        assert_eq!(u8::from(&nybble), 0);
    }

    #[test]
    fn test_from_u8_all_ones() {
        let nybble = Nybble::from(0b1111);
        assert_eq!(u8::from(&nybble), 0b1111);
    }

    #[test]
    fn test_from_u8_high_bits() {
        let nybble = Nybble::from(0b10101010);
        assert_eq!(u8::from(&nybble), 0b1010);
    }

    #[test]
    fn test_get_bit() {
        let nybble = Nybble::from(12);
        assert_eq!(nybble.get_bit(0), Bit::zero());
        assert_eq!(nybble.get_bit(1), Bit::zero());
        assert_eq!(nybble.get_bit(2), Bit::one());
        assert_eq!(nybble.get_bit(3), Bit::one());
    }

    #[test]
    #[allow(unused_variables)]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_bit_oob() {
        let nybble = Nybble::from(12);
        let p = nybble.get_bit(4);
    }

    #[test]
    fn test_set_bit() {
        let mut nybble = Nybble::default();
        nybble.set_bit(0);
        nybble.set_bit(1);
        nybble.set_bit(2);
        nybble.set_bit(3);
        assert_eq!(u8::from(&nybble), 15);
        assert_eq!(nybble.to_string(), "0xF");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_set_bit_oob() {
        let mut nybble = Nybble::from(12);
        nybble.set_bit(4);
    }

    #[test]
    fn test_unset_bit() {
        let mut nybble = Nybble::from(15);
        nybble.unset_bit(0);
        nybble.unset_bit(1);
        nybble.unset_bit(2);
        nybble.unset_bit(3);
        assert_eq!(u8::from(&nybble), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_unset_bit_oob() {
        let mut nybble = Nybble::from(12);
        nybble.unset_bit(4);
    }

    #[test]
    fn test_flip_bit() {
        let mut nybble = Nybble::from(15);
        nybble.flip_bit(0);
        nybble.flip_bit(1);
        nybble.flip_bit(2);
        nybble.flip_bit(3);
        assert_eq!(u8::from(&nybble), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_flip_bit_oob() {
        let mut nybble = Nybble::from(12);
        nybble.flip_bit(4);
    }

    #[test]
    fn test_flip() {
        let mut nybble = Nybble::from(15);
        nybble.flip();
        assert_eq!(u8::from(&nybble), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    fn test_not() {
        let nybble = Nybble::from(15);
        let nybble_not = !nybble;
        assert_eq!(u8::from(&nybble_not), 0);
        assert_eq!(nybble_not.to_string(), "0x0");
    }

    #[test]
    fn test_and() {
        let nybble_1 = Nybble::from(0b1010);
        let nybble_2 = Nybble::from(0b1100);
        let nybble_3 = nybble_1 & nybble_2;
        assert_eq!(u8::from(&nybble_3), 0b1000);
        assert_eq!(nybble_3.to_string(), "0x8");
    }

    #[test]
    fn test_and_assign() {
        let mut nybble_1 = Nybble::from(0b1010);
        let nybble_2 = Nybble::from(0b1100);
        nybble_1 &= nybble_2;
        assert_eq!(u8::from(&nybble_1), 0b1000);
        assert_eq!(nybble_1.to_string(), "0x8");
    }

    #[test]
    fn test_or() {
        let nybble_1 = Nybble::from(0b1010);
        let nybble_2 = Nybble::from(0b1100);
        let nybble_3 = nybble_1 | nybble_2;
        assert_eq!(u8::from(&nybble_3), 0b1110);
        assert_eq!(nybble_3.to_string(), "0xE");
    }

    #[test]
    fn test_or_assign() {
        let mut nybble_1 = Nybble::from(0b1010);
        let nybble_2 = Nybble::from(0b1100);
        nybble_1 |= nybble_2;
        assert_eq!(u8::from(&nybble_1), 0b1110);
        assert_eq!(nybble_1.to_string(), "0xE");
    }

    #[test]
    fn test_xor() {
        let nybble_1 = Nybble::from(0b1010);
        let nybble_2 = Nybble::from(0b1100);
        let nybble_3 = nybble_1 ^ nybble_2;
        assert_eq!(u8::from(&nybble_3), 0b0110);
        assert_eq!(nybble_3.to_string(), "0x6");
    }

    #[test]
    fn test_xor_assign() {
        let mut nybble_1 = Nybble::from(0b1010);
        let nybble_2 = Nybble::from(0b1100);
        nybble_1 ^= nybble_2;
        assert_eq!(u8::from(&nybble_1), 0b0110);
        assert_eq!(nybble_1.to_string(), "0x6");
    }

    #[test]
    fn test_display() {
        let nybble = Nybble::from(10);
        assert_eq!(format!("{}", nybble), "0xA");
    }

    #[test]
    fn test_iterator() {
        let nybble = Nybble::from(10);
        let mut iter = nybble.iter();
        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), Some(Bit::zero()));
        assert_eq!(iter.next(), Some(Bit::one()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_increment() {
        let mut nybble = Nybble::from(10);
        nybble.increment();
        assert_eq!(u8::from(&nybble), 11);
        assert_eq!(nybble.to_string(), "0xB");
    }

    #[test]
    fn test_decrement() {
        let mut nybble = Nybble::from(10);
        nybble.decrement();
        assert_eq!(u8::from(&nybble), 9);
        assert_eq!(nybble.to_string(), "0x9");
    }

    #[test]
    fn test_increment_boundary() {
        let mut nybble = Nybble::from(15);
        nybble.increment();
        assert_eq!(u8::from(&nybble), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    fn test_decrement_boundary() {
        let mut nybble = Nybble::from(0);
        nybble.decrement();
        assert_eq!(u8::from(&nybble), 0);
        assert_eq!(nybble.to_string(), "0x0");
    }

    #[test]
    fn test_into_iter() {
        let byte = Nybble::from(0b1010); // Assuming Byte::from exists
        let mut iter = (&byte).into_iter();

        // Assuming Bit is an enum with variants Zero and One
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::One));
        assert_eq!(iter.next(), None); // Ensure the iterator is exhausted
    }

    #[test]
    fn test_into_iter_empty_byte() {
        let byte = Nybble::from(0b0000); // Assuming Byte::from exists
        let mut iter = (&byte).into_iter();

        // Assuming Bit is an enum with variants Zero and One
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), Some(Bit::Zero));
        assert_eq!(iter.next(), None); // Ensure the iterator is exhausted
    }

    #[test]
    fn test_get_bit_ref() {
        let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero); // Dec: 10; Hex: 0xA; Oct: 0o12

        // Test that the 'get_bit_ref' method returns the correct Bit reference for a
        // given index
        assert_eq!(
            nybble.get_bit_ref(0),
            &Bit::Zero,
            "Bit at index 0 should be Zero"
        );
        assert_eq!(
            nybble.get_bit_ref(1),
            &Bit::One,
            "Bit at index 1 should be One"
        );
        assert_eq!(
            nybble.get_bit_ref(2),
            &Bit::Zero,
            "Bit at index 2 should be Zero"
        );
        assert_eq!(
            nybble.get_bit_ref(3),
            &Bit::One,
            "Bit at index 3 should be One"
        );
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_get_bit_ref_out_of_bounds() {
        let nybble = Nybble::new(Bit::One, Bit::Zero, Bit::One, Bit::Zero); // Dec: 10; Hex: 0xA; Oct: 0o12
        let _ = nybble.get_bit_ref(4); // This should panic
    }
}
