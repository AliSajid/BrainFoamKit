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

use crate::Nybble;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Byte {
    low_nybble: Nybble,
    high_nybble: Nybble,
}

impl Byte {
    pub fn new() -> Self {
        Self {
            low_nybble: Nybble::new(),
            high_nybble: Nybble::new(),
        }
    }

    pub fn to_u8(&self) -> u8 {
        (self.high_nybble.to_u8() << 4) | self.low_nybble.to_u8()
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "0x{:X}{:X}",
            self.high_nybble.to_u8(),
            self.low_nybble.to_u8()
        )
    }
}

impl Default for Byte {
    fn default() -> Self {
        Byte::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_new() {
        let byte = Byte::new();
        assert_eq!(byte.low_nybble, Nybble::new());
        assert_eq!(byte.high_nybble, Nybble::new());
    }

    #[test]
    fn test_byte_to_u8() {
        let mut byte = Byte::new();
        byte.low_nybble = Nybble::from_u8(0b1010);
        byte.high_nybble = Nybble::from_u8(0b0101);
        assert_eq!(byte.to_u8(), 0b01011010);
    }

    #[test]
    fn test_byte_display() {
        let mut byte = Byte::new();
        byte.low_nybble = Nybble::from_u8(0b1010);
        byte.high_nybble = Nybble::from_u8(0b0101);
        assert_eq!(format!("{}", byte), "0x5A");
    }
}
