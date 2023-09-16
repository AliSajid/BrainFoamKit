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

/// Define the states for the interpreter
enum Instruction {
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
            Instruction::IncrementPointer => write!(f, ">"),
            Instruction::DecrementPointer => write!(f, "<"),
            Instruction::IncrementValue => write!(f, "+"),
            Instruction::DecrementValue => write!(f, "-"),
            Instruction::OutputValue => write!(f, "."),
            Instruction::InputValue => write!(f, ","),
            Instruction::JumpForward => write!(f, "["),
            Instruction::JumpBackward => write!(f, "]"),
            Instruction::NoOp => write!(f, " "),
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
        assert_eq!(format!("{}", Instruction::IncrementPointer), ">");
        assert_eq!(format!("{}", Instruction::DecrementPointer), "<");
        assert_eq!(format!("{}", Instruction::IncrementValue), "+");
        assert_eq!(format!("{}", Instruction::DecrementValue), "-");
        assert_eq!(format!("{}", Instruction::OutputValue), ".");
        assert_eq!(format!("{}", Instruction::InputValue), ",");
        assert_eq!(format!("{}", Instruction::JumpForward), "[");
        assert_eq!(format!("{}", Instruction::JumpBackward), "]");
        assert_eq!(format!("{}", Instruction::NoOp), " ");
    }
}
