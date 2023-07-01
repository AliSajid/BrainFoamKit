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

use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    One,
    Zero,
}

#[allow(dead_code)]
impl Bit {
    fn new() -> Self {
        Bit::Zero
    }

    fn flip(&mut self) {
        *self = match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Bit::Zero => write!(f, "0"),
            Bit::One => write!(f, "1"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Byte {
    bits: [Bit; 8],
}

impl Byte {
    fn new() -> Self {
        Self {
            bits: [Bit::Zero; 8],
        }
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            self.bits[0],
            self.bits[1],
            self.bits[2],
            self.bits[3],
            self.bits[4],
            self.bits[5],
            self.bits[6],
            self.bits[7],
        )
    }
}

#[allow(dead_code)]
struct Tape {
    bytes: Vec<Byte>,
}

impl Tape {
    fn new(length: usize) -> Self {
        Self {
            // Create a vector of bytes with the specified length
            bytes: vec![Byte::new(); length],
        }
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new(30000)
    }
}

#[allow(dead_code)]
struct Machine {
    tape: Tape,
    pointer: usize,
}

mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_byte() {
        let byte = Byte::new();
        assert_eq!(byte.to_string(), "00000000");
    }

    #[test]
    fn test_bit() {
        let mut bit = Bit::new();
        assert_eq!(bit.to_string(), "0");
        bit.flip();
        assert_eq!(bit.to_string(), "1");
    }

    #[test]
    fn test_tape() {
        let tape = Tape::new(10);
        assert_eq!(tape.bytes.len(), 10);
    }
}
