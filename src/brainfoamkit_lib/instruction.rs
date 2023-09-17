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

/// Define the states for the interpreter
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    OutputValue,
    InputValue,
    JumpForward,
    JumpBackward,
    NoOp,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Instruction::IncrementPointer => write!(f, "INCPTR"),
            Instruction::DecrementPointer => write!(f, "DECPTR"),
            Instruction::IncrementValue => write!(f, "INCVAL"),
            Instruction::DecrementValue => write!(f, "DECVAL"),
            Instruction::OutputValue => write!(f, "OUTVAL"),
            Instruction::InputValue => write!(f, "INPVAL"),
            Instruction::JumpForward => write!(f, "JMPFWD"),
            Instruction::JumpBackward => write!(f, "JMPBCK"),
            Instruction::NoOp => write!(f, "NOOP"),
        }
    }
}

impl Instruction {
    /// Convert a char to an Instruction
    fn from_char(c: char) -> Instruction {
        match c {
            '>' => Instruction::IncrementPointer,
            '<' => Instruction::DecrementPointer,
            '+' => Instruction::IncrementValue,
            '-' => Instruction::DecrementValue,
            '.' => Instruction::OutputValue,
            ',' => Instruction::InputValue,
            '[' => Instruction::JumpForward,
            ']' => Instruction::JumpBackward,
            _ => Instruction::NoOp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from_char() {
        assert_eq!(Instruction::from_char('>'), Instruction::IncrementPointer);
        assert_eq!(Instruction::from_char('<'), Instruction::DecrementPointer);
        assert_eq!(Instruction::from_char('+'), Instruction::IncrementValue);
        assert_eq!(Instruction::from_char('-'), Instruction::DecrementValue);
        assert_eq!(Instruction::from_char('.'), Instruction::OutputValue);
        assert_eq!(Instruction::from_char(','), Instruction::InputValue);
        assert_eq!(Instruction::from_char('['), Instruction::JumpForward);
        assert_eq!(Instruction::from_char(']'), Instruction::JumpBackward);
        assert_eq!(Instruction::from_char(' '), Instruction::NoOp);
    }

    #[test]
    fn test_instruction_display() {
        assert_eq!(format!("{}", Instruction::IncrementPointer), "INCPTR");
        assert_eq!(format!("{}", Instruction::DecrementPointer), "DECPTR");
        assert_eq!(format!("{}", Instruction::IncrementValue), "INCVAL");
        assert_eq!(format!("{}", Instruction::DecrementValue), "DECVAL");
        assert_eq!(format!("{}", Instruction::OutputValue), "OUTVAL");
        assert_eq!(format!("{}", Instruction::InputValue), "INPVAL");
        assert_eq!(format!("{}", Instruction::JumpForward), "JMPFWD");
        assert_eq!(format!("{}", Instruction::JumpBackward), "JMPBCK");
        assert_eq!(format!("{}", Instruction::NoOp), " ");
    }
}
