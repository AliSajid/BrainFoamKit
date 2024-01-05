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
// * http://www.apache.org/licenses/LICENSE-2.0
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
// * Permission is hereby granted, free of charge, to any person obtaining a
// * copy
// * of this software and associated documentation files (the "Software"), to
// * deal
// * in the Software without restriction, including without limitation the
// * rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in
// * all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// * FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// * THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

use std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    ops::Index,
};

use crate::Instruction;

/// Structure to hold the program.
///
/// A `Program` is a series if instructions stored in the program stack.
/// This struct allows us to conveniently read the program, modify it and save
/// it back.
///
/// # Examples
///
/// ## Loading a `Program` from a series of instructions
///
/// ```
/// use brainfoamkit_lib::{
///     Instruction,
///     Program,
/// };
///
/// let instructions = vec![
///     Instruction::IncrementPointer,
///     Instruction::IncrementValue,
///     Instruction::DecrementPointer,
///     Instruction::DecrementValue,
/// ];
/// let mut program = Program::from(instructions);
///
/// assert_eq!(program.length(), Some(4));
/// ```
///
/// ## Load a `Program` from a string
///
/// ```
/// // TODO: Verify this example
/// use brainfoamkit_lib::Program;
///
/// let program_string = ">>++<<--";
/// let program = Program::from(program_string);
///
/// assert_eq!(program.length(), Some(8));
/// ```
///
/// ## Get an instruction from a `Program`
///
/// ```
/// // TODO: Verify this example
/// use brainfoamkit_lib::{
///     Instruction,
///     Program,
/// };
///
/// let program_string = ">+<-";
///
/// let mut program = Program::from(program_string);
///
/// assert_eq!(
///     program.get_instruction(0),
///     Some(Instruction::IncrementPointer)
/// );
/// assert_eq!(
///     program.get_instruction(1),
///     Some(Instruction::IncrementValue)
/// );
/// assert_eq!(
///     program.get_instruction(2),
///     Some(Instruction::DecrementPointer)
/// );
/// assert_eq!(
///     program.get_instruction(3),
///     Some(Instruction::DecrementValue)
/// );
/// assert_eq!(program.get_instruction(4), None);
/// ```
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Program {
    /// The instructions for the program
    instructions: Vec<Instruction>,
}

impl Program {
    /// Get an instruction from a `Program` at a specific index
    ///
    /// This method gets an instruction from the program at a specific index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the instruction to get
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Instruction,
    ///     Program,
    /// };
    ///
    /// let instructions = ">>++<<--";
    /// let program = Program::from(instructions);
    ///
    /// assert_eq!(
    ///     program.get_instruction(0),
    ///     Some(Instruction::IncrementPointer)
    /// );
    /// assert_eq!(
    ///     program.get_instruction(1),
    ///     Some(Instruction::IncrementPointer)
    /// );
    /// assert_eq!(
    ///     program.get_instruction(2),
    ///     Some(Instruction::IncrementValue)
    /// );
    /// assert_eq!(
    ///     program.get_instruction(3),
    ///     Some(Instruction::IncrementValue)
    /// );
    /// assert_eq!(
    ///     program.get_instruction(4),
    ///     Some(Instruction::DecrementPointer)
    /// );
    /// assert_eq!(
    ///     program.get_instruction(5),
    ///     Some(Instruction::DecrementPointer)
    /// );
    /// assert_eq!(
    ///     program.get_instruction(6),
    ///     Some(Instruction::DecrementValue)
    /// );
    /// assert_eq!(
    ///     program.get_instruction(7),
    ///     Some(Instruction::DecrementValue)
    /// );
    /// assert_eq!(program.get_instruction(8), None);
    /// ```
    ///
    /// # Returns
    ///
    /// The `Instruction` at the given index
    ///
    /// # See Also
    ///
    /// * [`length()`](#method.length): Get the length of the program
    #[must_use]
    pub fn get_instruction(&self, index: usize) -> Option<Instruction> {
        self.length().and_then(|length| {
            if index >= length {
                None
            } else {
                Some(self.instructions[index])
            }
        })
    }

    /// Find the matching `JumpBackward` instruction for the given `JumpForward`
    /// instruction
    ///
    /// This method allows us to identify the boundaries of a given loop.
    /// It will return the index of the matching `JumpBackward` instruction for
    /// the given `JumpForward` instruction. It returns `None` if no
    /// matching `JumpBackward` instruction is found or the instruction
    /// at the given index is not a `JumpForward` instruction.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Instruction,
    ///     Program,
    /// };
    ///
    /// let instructions = "[[]]";
    /// let mut program = Program::from(instructions);
    ///
    /// assert_eq!(program.find_matching_bracket(0), Some(3));
    /// assert_eq!(program.find_matching_bracket(1), Some(2));
    /// ```
    ///
    /// # Returns
    ///
    /// The index of the matching bracket
    ///
    /// # See Also
    ///
    /// * [`length()`](#method.length): Get the length of the program
    /// * [`get_instruction()`](#method.get_instruction): Get an instruction
    ///   from a `Program`
    #[must_use]
    pub fn find_matching_bracket(&self, index: usize) -> Option<usize> {
        match self.get_instruction(index) {
            Some(Instruction::JumpForward) => {
                let mut bracket_counter = 0;
                let mut index = index;

                loop {
                    match self.instructions.get(index) {
                        Some(Instruction::JumpForward) => bracket_counter += 1,
                        Some(Instruction::JumpBackward) => bracket_counter -= 1,
                        _ => (),
                    }

                    if bracket_counter == 0 {
                        break;
                    }

                    index += 1;
                }

                Some(index)
            }
            _ => None,
        }
    }

    /// Get the length of the program
    ///
    /// This method returns the length of the program.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    ///
    /// let program_string = ">>++<<--";
    /// let program = Program::from(program_string);
    ///
    /// assert_eq!(program.length(), Some(8));
    /// ```
    ///
    /// # Returns
    ///
    /// The length of the program
    #[must_use]
    pub fn length(&self) -> Option<usize> {
        if self.instructions.is_empty() {
            None
        } else {
            Some(self.instructions.len())
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::from(vec![Instruction::NoOp; 10])
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (index, instruction) in self.instructions.iter().enumerate() {
            // Index should be zero padded to 4 digits
            writeln!(f, "{index:04}: {instruction}")?;
        }
        Ok(())
    }
}

impl Index<usize> for Program {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        &self.instructions[index]
    }
}

impl From<&str> for Program {
    /// Load a `Program` from a string
    ///
    /// This method loads a `Program` from a string.
    ///
    /// # Arguments
    ///
    /// * `program` - A string containing the program to load
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    ///
    /// let program_string = ">>++<<--";
    /// let program = Program::from(program_string);
    ///
    /// assert_eq!(program.length(), Some(8));
    /// ```
    ///
    /// # See Also
    ///
    /// * [`from()`](#method.from): Create a new `Program` from a series of
    ///   instructions
    fn from(program: &str) -> Self {
        let mut instructions = Vec::new();

        for c in program.chars() {
            instructions.push(Instruction::from_char(c));
        }

        Self { instructions }
    }
}

impl From<Vec<Instruction>> for Program {
    /// Create a new `Program` from a series of instructions
    ///
    /// This method creates a new `Program` from a series of instructions.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A vector of `Instruction`s to load into the `Program`
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Instruction,
    ///     Program,
    /// };
    ///
    /// let instructions = vec![
    ///     Instruction::IncrementPointer,
    ///     Instruction::IncrementValue,
    ///     Instruction::DecrementPointer,
    ///     Instruction::DecrementValue,
    /// ];
    /// let program: Program = Program::from(instructions);
    ///
    /// assert_eq!(program.length(), Some(4));
    /// ```
    ///
    /// # See Also
    ///
    /// * [`from()`](#method.from): Load a `Program` from a string
    fn from(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_from() {
        let instructions = vec![Instruction::NoOp];
        let program = Program::from(instructions);

        assert_eq!(program.instructions.len(), 1);
        assert_eq!(program.length(), Some(1));
    }

    #[test]
    fn test_program_load_from() {
        let instructions = ">>++<<--";
        let program = Program::from(instructions);

        assert_eq!(program.instructions.len(), 8);
        assert_eq!(program.length(), Some(8));
    }

    #[test]
    fn test_program_length() {
        let program = Program::from(">>++<<--");
        assert_eq!(program.length(), Some(8));

        let program = Program::from("");
        assert_eq!(program.length(), None);
    }

    #[test]
    fn test_program_default() {
        let program = Program::default();

        assert_eq!(program.instructions.len(), 10);
        assert_eq!(program.length(), Some(10));
    }

    #[test]
    fn test_program_display() {
        let instructions = vec![Instruction::NoOp];
        let program = Program::from(instructions);

        assert_eq!(program.to_string(), "0000: NOOP\n");

        let instructions = vec![Instruction::NoOp, Instruction::NoOp];
        let program = Program::from(instructions);
        assert_eq!(program.to_string(), "0000: NOOP\n0001: NOOP\n");
    }

    #[test]
    fn test_program_find_matching_bracket() {
        let instructions = "[]";
        let program = Program::from(instructions);

        assert_eq!(program.find_matching_bracket(0), Some(1));
    }

    #[test]
    fn test_program_find_matching_bracket_nested() {
        let instructions = "[[]]";
        let program = Program::from(instructions);

        assert_eq!(program.find_matching_bracket(0), Some(3));
    }

    // #[test]
    // fn test_find_matching_bracket_no_match() {
    //     let instructions = "[";
    //     let program = Program::from(instructions);

    //     assert_eq!(program.find_matching_bracket(0), None);
    // }

    #[test]
    fn test_find_matching_bracket_not_jump_forward() {
        let instructions = "]";
        let program = Program::from(instructions);

        assert_eq!(program.find_matching_bracket(0), None);
    }

    #[test]
    fn test_get_instruction() {
        let instructions = vec![
            Instruction::IncrementPointer,
            Instruction::IncrementPointer,
            Instruction::IncrementValue,
            Instruction::IncrementValue,
            Instruction::DecrementPointer,
            Instruction::DecrementPointer,
            Instruction::DecrementValue,
            Instruction::DecrementValue,
        ];
        let program = Program::from(instructions);

        assert_eq!(
            program.get_instruction(0),
            Some(Instruction::IncrementPointer)
        );
        assert_eq!(
            program.get_instruction(1),
            Some(Instruction::IncrementPointer)
        );
        assert_eq!(
            program.get_instruction(2),
            Some(Instruction::IncrementValue)
        );
        assert_eq!(
            program.get_instruction(3),
            Some(Instruction::IncrementValue)
        );
        assert_eq!(
            program.get_instruction(4),
            Some(Instruction::DecrementPointer)
        );
        assert_eq!(
            program.get_instruction(5),
            Some(Instruction::DecrementPointer)
        );
        assert_eq!(
            program.get_instruction(6),
            Some(Instruction::DecrementValue)
        );
        assert_eq!(
            program.get_instruction(7),
            Some(Instruction::DecrementValue)
        );
        assert_eq!(program.get_instruction(8), None);
    }

    #[test]
    fn test_find_matching_bracket() {
        let instructions = vec![
            Instruction::JumpForward,
            Instruction::JumpForward,
            Instruction::JumpBackward,
            Instruction::JumpBackward,
        ];
        let program = Program::from(instructions);

        assert_eq!(program.find_matching_bracket(0), Some(3));
        assert_eq!(program.find_matching_bracket(1), Some(2));
        assert_eq!(program.find_matching_bracket(2), None);
        assert_eq!(program.find_matching_bracket(3), None);
    }

    #[test]
    fn test_default() {
        let program = Program::default();
        assert_eq!(program.length(), Some(10));
        assert_eq!(program.get_instruction(0), Some(Instruction::NoOp));
        assert_eq!(program.get_instruction(9), Some(Instruction::NoOp));
    }

    #[test]
    fn test_index() {
        let program = Program::from(">>++<<--");

        assert_eq!(program[0], Instruction::IncrementPointer);
        assert_eq!(program[1], Instruction::IncrementPointer);
        assert_eq!(program[2], Instruction::IncrementValue);
        assert_eq!(program[3], Instruction::IncrementValue);
        assert_eq!(program[4], Instruction::DecrementPointer);
        assert_eq!(program[5], Instruction::DecrementPointer);
        assert_eq!(program[6], Instruction::DecrementValue);
        assert_eq!(program[7], Instruction::DecrementValue);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_out_of_bounds() {
        let program = Program::from(">>++<<--");
        let _ = program[8];
    }
}
