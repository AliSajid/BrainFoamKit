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
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nybble {
    bit_0: Bit,
    bit_1: Bit,
    bit_2: Bit,
    bit_3: Bit,
}

impl Nybble {
    pub fn new() -> Self {
        Self {
            bit_0: Bit::zero(),
            bit_1: Bit::zero(),
            bit_2: Bit::zero(),
            bit_3: Bit::zero(),
        }
    }

    pub fn from_u8(n: u8) -> Self {
        let mut nybble = Self::new();
        let mut n = n;
        if n >= 8 {
            nybble.bit_3.flip();
            n -= 8;
        }
        if n >= 4 {
            nybble.bit_2.flip();
            n -= 4;
        }
        if n >= 2 {
            nybble.bit_1.flip();
            n -= 2;
        }
        if n >= 1 {
            nybble.bit_0.flip();
        }
        nybble
    }

    pub fn to_u8(&self) -> u8 {
        let mut n = 0;
        if self.bit_3 == Bit::One {
            n += 8;
        }
        if self.bit_2 == Bit::One {
            n += 4;
        }
        if self.bit_1 == Bit::One {
            n += 2;
        }
        if self.bit_0 == Bit::One {
            n += 1;
        }
        n
    }

    pub fn get_bit(&self, index: usize) -> Bit {
        match index {
            0 => self.bit_0,
            1 => self.bit_1,
            2 => self.bit_2,
            3 => self.bit_3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Display for Nybble {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#04X}", self.to_u8())
    }
}

impl Default for Nybble {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nybble_zero() {
        let nybble = Nybble::new();
        assert_eq!(nybble.to_u8(), 0);
        assert_eq!(nybble.to_string(), "0x00");
        assert_eq!(Nybble::from_u8(0), nybble);
    }

    #[test]
    fn test_nybble_one() {
        let nybble = Nybble::from_u8(1);
        assert_eq!(nybble.to_u8(), 1);
        assert_eq!(nybble.to_string(), "0x01");
        assert_eq!(Nybble::from_u8(1), nybble);
    }

    #[test]
    fn test_nybble_two() {
        let nybble = Nybble::from_u8(2);
        assert_eq!(nybble.to_u8(), 2);
        assert_eq!(nybble.to_string(), "0x02");
        assert_eq!(Nybble::from_u8(2), nybble);
    }

    #[test]
    fn test_nybble_three() {
        let nybble = Nybble::from_u8(3);
        assert_eq!(nybble.to_u8(), 3);
        assert_eq!(nybble.to_string(), "0x03");
        assert_eq!(Nybble::from_u8(3), nybble);
    }

    #[test]
    fn test_nybble_four() {
        let nybble = Nybble::from_u8(4);
        assert_eq!(nybble.to_u8(), 4);
        assert_eq!(nybble.to_string(), "0x04");
        assert_eq!(Nybble::from_u8(4), nybble);
    }

    #[test]
    fn test_nybble_five() {
        let nybble = Nybble::from_u8(5);
        assert_eq!(nybble.to_u8(), 5);
        assert_eq!(nybble.to_string(), "0x05");
        assert_eq!(Nybble::from_u8(5), nybble);
    }

    #[test]
    fn test_nybble_six() {
        let nybble = Nybble::from_u8(6);
        assert_eq!(nybble.to_u8(), 6);
        assert_eq!(nybble.to_string(), "0x06");
        assert_eq!(Nybble::from_u8(6), nybble);
    }

    #[test]
    fn test_nybble_seven() {
        let nybble = Nybble::from_u8(7);
        assert_eq!(nybble.to_u8(), 7);
        assert_eq!(nybble.to_string(), "0x07");
        assert_eq!(Nybble::from_u8(7), nybble);
    }

    #[test]
    fn test_nybble_eight() {
        let nybble = Nybble::from_u8(8);
        assert_eq!(nybble.to_u8(), 8);
        assert_eq!(nybble.to_string(), "0x08");
        assert_eq!(Nybble::from_u8(8), nybble);
    }

    #[test]
    fn test_nybble_nine() {
        let nybble = Nybble::from_u8(9);
        assert_eq!(nybble.to_u8(), 9);
        assert_eq!(nybble.to_string(), "0x09");
        assert_eq!(Nybble::from_u8(9), nybble);
    }

    #[test]
    fn test_nybble_ten() {
        let nybble = Nybble::from_u8(10);
        assert_eq!(nybble.to_u8(), 10);
        assert_eq!(nybble.to_string(), "0x0A");
        assert_eq!(Nybble::from_u8(10), nybble);
    }

    #[test]
    fn test_nybble_eleven() {
        let nybble = Nybble::from_u8(11);
        assert_eq!(nybble.to_u8(), 11);
        assert_eq!(nybble.to_string(), "0x0B");
        assert_eq!(Nybble::from_u8(11), nybble);
    }

    #[test]
    fn test_nybble_twelve() {
        let nybble = Nybble::from_u8(12);
        assert_eq!(nybble.to_u8(), 12);
        assert_eq!(nybble.to_string(), "0x0C");
        assert_eq!(Nybble::from_u8(12), nybble);
    }

    #[test]
    fn test_nybble_thirteen() {
        let nybble = Nybble::from_u8(13);
        assert_eq!(nybble.to_u8(), 13);
        assert_eq!(nybble.to_string(), "0x0D");
        assert_eq!(Nybble::from_u8(13), nybble);
    }

    #[test]
    fn test_nybble_fourteen() {
        let nybble = Nybble::from_u8(14);
        assert_eq!(nybble.to_u8(), 14);
        assert_eq!(nybble.to_string(), "0x0E");
        assert_eq!(Nybble::from_u8(14), nybble);
    }

    #[test]
    fn test_nybble_fifteen() {
        let nybble = Nybble::from_u8(15);
        assert_eq!(nybble.to_u8(), 15);
        assert_eq!(nybble.to_string(), "0x0F");
        assert_eq!(Nybble::from_u8(15), nybble);
    }

    #[test]
    fn test_get_bit() {
        let nybble = Nybble::from_u8(0b1010);
        assert_eq!(nybble.get_bit(0), Bit::one());
        assert_eq!(nybble.get_bit(1), Bit::zero());
        assert_eq!(nybble.get_bit(2), Bit::one());
        assert_eq!(nybble.get_bit(3), Bit::zero());
    }
}
