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

use crate::Instruction;

use std::fmt::{self, Display, Formatter};

/// Structure to hold the program.
///
/// A `Program` is a series if instructions stored in the program stack.
/// This struct allows us to conveniently read the program, modify it and save it back.
///
/// # Examples
///
/// ## Empty Program
///
/// ```
/// use brainfoamkit_lib::Program;
///
/// let program = Program::new();
///
/// assert_eq!(program.get_program_length(), None);
/// assert_eq!(program.get_instruction_counter(), None);
/// ```
///
/// ## Loading a `Program` from a series of instructions
///
/// ```
/// use brainfoamkit_lib::Program;
/// use brainfoamkit_lib::Instruction;
///
/// let instructions = vec![Instruction::IncrementPointer, Instruction::IncrementValue, Instruction::DecrementPointer, Instruction::DecrementValue];
/// let mut program = Program::new();
///
/// program.load(instructions);
///
/// assert_eq!(program.get_program_length(), Some(4));
/// ```
///
/// ## Load a `Program` from a string
///
/// ```
/// use brainfoamkit_lib::Program;
///
/// let program_string = ">>++<<--";
/// let mut program = Program::new();
///
/// program.load_from_string(program_string);
///
/// assert_eq!(program.get_program_length(), Some(8));
/// ```
///
/// ## Get an instruction from a `Program`
///
/// ```
/// use brainfoamkit_lib::Program;
/// use brainfoamkit_lib::Instruction;
///
/// let program_string = ">+<-";
///
/// let mut program = Program::new();
///
/// program.load_from_string(program_string);
///
/// assert_eq!(program.get_instruction(), Some(Instruction::IncrementPointer));
/// program.increment_instruction_counter();
/// assert_eq!(program.get_instruction(), Some(Instruction::IncrementValue));
/// program.increment_instruction_counter();
/// assert_eq!(program.get_instruction(), Some(Instruction::DecrementPointer));
/// program.increment_instruction_counter();
/// assert_eq!(program.get_instruction(), Some(Instruction::DecrementValue));
/// program.increment_instruction_counter();
/// assert_eq!(program.get_instruction(), None);
/// ```
pub struct Program {
    /// The instructions for the program
    instructions: Vec<Instruction>,
    /// The current instruction counter
    instruction_counter: Option<usize>,
}

impl Program {
    /// Create a new `Program`
    ///
    /// This method creates a new `Program` with no instructions.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    ///
    /// let program = Program::new();
    ///
    /// assert_eq!(program.get_program_length(), None);
    /// assert_eq!(program.get_instruction_counter(), None);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `Program`
    ///
    /// # See Also
    ///
    /// * [`load()`](#method.load): Load a `Program` from a series of instructions
    /// * [`load_from_string()`](#method.load_from_string): Load a `Program` from a string
    /// * [`get_instruction()`](#method.get_instruction): Get an instruction from a `Program`
    /// * [`get_instruction_counter()`](#method.get_instruction_counter): Get the current instruction counter
    /// * [`increment_instruction_counter()`](#method.increment_instruction_counter): Increment the instruction counter
    /// * [`get_program_length()`](#method.get_program_length): Get the length of the program
    /// * [`push()`](#method.push): Push an instruction onto the program
    /// * [`find_matching_bracket()`](#method.find_matching_bracket): Find the matching bracket
    #[must_use]
    pub const fn new() -> Self {
        Self {
            instructions: Vec::new(),
            instruction_counter: None,
        }
    }

    /// Load a `Program` from a series of instructions
    ///
    /// This method loads a `Program` from a series of instructions.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A vector of `Instruction`s to load into the `Program`
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instructions = vec![Instruction::IncrementPointer, Instruction::IncrementValue, Instruction::DecrementPointer, Instruction::DecrementValue];
    /// let mut program = Program::new();
    ///
    /// program.load(instructions);
    ///
    /// assert_eq!(program.get_program_length(), Some(4));
    /// ```
    ///
    /// # See Also
    ///
    /// * [`new()`](#method.new): Create a new `Program`
    /// * [`load_from_string()`](#method.load_from_string): Load a `Program` from a string
    pub fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
        self.instruction_counter = Some(0);
    }

    /// Get the current instruction from a `Program`
    ///
    /// This method gets the current instruction from the program as indicated by the instruction counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instructions = vec![Instruction::IncrementPointer, Instruction::IncrementValue, Instruction::DecrementPointer, Instruction::DecrementValue];
    /// let mut program = Program::new();
    ///
    /// program.load(instructions);
    ///
    /// assert_eq!(program.get_instruction(), Some(Instruction::IncrementPointer));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction(), Some(Instruction::IncrementValue));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction(), Some(Instruction::DecrementPointer));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction(), Some(Instruction::DecrementValue));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction(), None);
    /// ```
    ///
    /// # Returns
    ///
    /// The current instruction from the program
    ///
    /// # See Also
    ///
    /// * [`new()`](#method.new): Create a new `Program`
    /// * [`load()`](#method.load): Load a `Program` from a series of instructions
    /// * [`load_from_string()`](#method.load_from_string): Load a `Program` from a string
    /// * [`get_instruction_counter()`](#method.get_instruction_counter): Get the current instruction counter
    #[must_use]
    pub fn get_instruction(&self) -> Option<Instruction> {
        self.instruction_counter
            .and_then(|i| self.get_instruction_at(i))
    }

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
    /// use brainfoamkit_lib::Program;
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instructions = ">>++<<--";
    /// let mut program = Program::new();
    ///
    /// program.load_from_string(instructions);
    ///
    /// assert_eq!(program.get_instruction_at(0), Some(Instruction::IncrementPointer));
    /// assert_eq!(program.get_instruction_at(1), Some(Instruction::IncrementPointer));
    /// assert_eq!(program.get_instruction_at(2), Some(Instruction::IncrementValue));
    /// assert_eq!(program.get_instruction_at(3), Some(Instruction::IncrementValue));
    /// assert_eq!(program.get_instruction_at(4), Some(Instruction::DecrementPointer));
    /// assert_eq!(program.get_instruction_at(5), Some(Instruction::DecrementPointer));
    /// assert_eq!(program.get_instruction_at(6), Some(Instruction::DecrementValue));
    /// assert_eq!(program.get_instruction_at(7), Some(Instruction::DecrementValue));
    /// assert_eq!(program.get_instruction_at(8), None);
    /// ```
    ///
    /// # Returns
    ///
    /// The `Instruction` at the given index
    ///
    /// # See Also
    ///
    /// * [`get_instruction()`](#method.get_instruction): Get an instruction from a `Program`
    /// * [`get_program_length()`](#method.get_program_length): Get the length of the program
    /// * [`push()`](#method.push): Push an instruction onto the program
    #[must_use]
    pub fn get_instruction_at(&self, index: usize) -> Option<Instruction> {
        if index >= self.instructions.len() {
            return None;
        }
        Some(self.instructions[index])
    }

    /// Find the matching bracket
    ///
    /// This method finds the matching bracket for the current instruction.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instructions = "[[]]";
    /// let mut program = Program::new();
    ///
    /// program.load_from_string(instructions);
    ///
    /// assert_eq!(program.find_matching_bracket(), Some(3));
    /// ```
    ///
    /// # Returns
    ///
    /// The index of the matching bracket
    ///
    /// # See Also
    ///
    /// * [`get_instruction_counter()`](#method.get_instruction_counter): Get the current instruction counter
    /// * [`get_program_length()`](#method.get_program_length): Get the length of the program
    /// * [`push()`](#method.push): Push an instruction onto the program
    /// * [`get_instruction()`](#method.get_instruction): Get an instruction from a `Program`
    #[must_use]
    pub fn find_matching_bracket(&self) -> Option<usize> {
        let mut bracket_counter = 0;
        let mut index = self.instruction_counter.unwrap_or(0);

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
    /// let mut program = Program::new();
    ///
    /// program.load_from_string(program_string);
    ///
    /// assert_eq!(program.get_program_length(), Some(8));
    /// ```
    ///
    /// # See Also
    ///
    /// * [`new()`](#method.new): Create a new `Program`
    /// * [`load()`](#method.load): Load a `Program` from a series of instructions
    pub fn load_from_string(&mut self, program: &str) {
        let mut instructions = Vec::new();

        for c in program.chars() {
            instructions.push(Instruction::from_char(c));
        }

        self.load(instructions);
    }

    /// Get the current value of the instruction counter
    ///
    /// This method gets the current value of the instruction counter.
    ///
    /// # Examples
    ///
    /// ## New program
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    ///
    /// let program = Program::new();
    ///
    /// assert_eq!(program.get_instruction_counter(), None);
    ///
    /// ```
    /// ## Program with executed instructions
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instructions = vec![Instruction::IncrementPointer, Instruction::IncrementValue, Instruction::DecrementPointer, Instruction::DecrementValue];
    ///
    /// let mut program = Program::new();
    ///
    /// program.load(instructions);
    ///
    /// assert_eq!(program.get_instruction_counter(), Some(1));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), Some(2));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), Some(3));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), Some(4));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), None);
    /// ```
    ///
    /// # Returns
    ///
    /// The current value of the instruction counter
    ///
    /// # See Also
    ///
    /// * [`set_instruction_counter()`](#method.set_instruction_counter): Set the current instruction counter
    /// * [`increment_instruction_counter()`](#method.increment_instruction_counter): Increment the instruction counter
    /// * [`get_program_length()`](#method.get_program_length): Get the length of the program
    #[must_use]
    pub fn get_instruction_counter(&self) -> Option<usize> {
        self.instruction_counter
            .map(|instruction_counter| instruction_counter + 1)
    }

    /// Set the current value of the instruction counter
    ///
    /// This method sets the current value of the instruction counter.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the instruction counter to
    ///
    /// # Examples
    ///
    /// ## New program
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    ///
    /// let mut program = Program::new();
    /// program.set_instruction_counter(1);
    /// assert_eq!(program.get_instruction_counter(), None);
    ///
    /// ```
    /// ## Program with executed instructions
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instructions = vec![Instruction::IncrementPointer, Instruction::IncrementValue, Instruction::DecrementPointer, Instruction::DecrementValue];
    ///
    /// let mut program = Program::new();
    ///
    /// program.load(instructions);
    ///
    /// assert_eq!(program.get_instruction_counter(), Some(1));
    /// program.set_instruction_counter(3);
    /// assert_eq!(program.get_instruction_counter(), Some(3));
    /// program.set_instruction_counter(2);
    /// assert_eq!(program.get_instruction_counter(), Some(2));
    /// program.set_instruction_counter(6); // Out of bounds. Will ignore
    /// assert_eq!(program.get_instruction_counter(), Some(2));
    /// ```
    ///
    /// # Side Effects
    ///
    /// This method will set the instruction counter to the given value.
    ///
    /// # See Also
    ///
    /// * [`get_instruction_counter()`](#method.get_instruction_counter): Get the current instruction counter
    /// * [`increment_instruction_counter()`](#method.increment_instruction_counter): Increment the instruction counter
    /// * [`get_program_length()`](#method.get_program_length): Get the length of the program
    pub fn set_instruction_counter(&mut self, value: usize) {
        if let Some(_counter) = self.instruction_counter {
            if value < self.instructions.len() {
                self.instruction_counter = Some(value - 1);
            }
        }
    }

    /// Increment the instruction counter
    ///
    /// This method increments the instruction counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Program;
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instructions = vec![Instruction::IncrementPointer, Instruction::IncrementValue, Instruction::DecrementPointer, Instruction::DecrementValue];
    ///
    /// let mut program = Program::new();
    /// program.load(instructions);
    ///
    /// assert_eq!(program.get_instruction_counter(), Some(1));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), Some(2));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), Some(3));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), Some(4));
    /// program.increment_instruction_counter();
    /// assert_eq!(program.get_instruction_counter(), None);
    /// ```
    ///
    /// # See Also
    ///
    /// * [`get_instruction_counter()`](#method.get_instruction_counter): Get the current instruction counter
    /// * [`set_instruction_counter()`](#method.set_instruction_counter): Set the current instruction counter
    /// * [`get_program_length()`](#method.get_program_length): Get the length of the program
    /// * [`push()`](#method.push): Push an instruction onto the program
    pub fn increment_instruction_counter(&mut self) {
        if let Some(counter) = self.instruction_counter {
            if (counter + 1) >= self.instructions.len() {
                self.instruction_counter = None;
            } else {
                self.instruction_counter = Some(counter + 1);
            }
        }
    }

    #[must_use]
    pub fn get_program_length(&self) -> Option<usize> {
        if self.instructions.is_empty() {
            None
        } else {
            Some(self.instructions.len())
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);

        match self.instruction_counter {
            None => self.instruction_counter = Some(0),
            Some(_) => {}
        };
    }
}

impl Default for Program {
    fn default() -> Self {
        let mut program = Self::new();
        program.load(vec![Instruction::NoOp]);
        program
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (index, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "{index}: {instruction}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_new() {
        let program = Program::new();

        assert_eq!(program.instructions.len(), 0);
        assert_eq!(program.instruction_counter, None);
    }

    #[test]
    fn test_program_load() {
        let mut program = Program::new();
        let instructions = vec![Instruction::NoOp];

        program.load(instructions);

        assert_eq!(program.instructions.len(), 1);
        assert_eq!(program.instruction_counter, Some(0));
    }

    #[test]
    fn test_program_get_instruction() {
        let mut program = Program::new();
        assert_eq!(program.get_instruction(), None);

        let instructions = vec![Instruction::NoOp];
        program.load(instructions);
        assert_eq!(program.get_instruction(), Some(Instruction::NoOp));

        program.increment_instruction_counter();
        assert_eq!(program.get_instruction(), None);
    }

    #[test]
    fn test_program_load_from_string() {
        let mut program = Program::new();
        let instructions = ">>++<<--";

        program.load_from_string(instructions);

        assert_eq!(program.instructions.len(), 8);
        assert_eq!(program.get_program_length(), Some(8));
        assert_eq!(program.instruction_counter, Some(0));
    }

    #[test]
    fn test_program_get_instruction_counter() {
        let mut program = Program::new();
        assert_eq!(program.get_instruction_counter(), None);

        program.push(Instruction::NoOp);
        assert_eq!(program.get_instruction_counter(), Some(1));

        program.push(Instruction::NoOp);
        assert_eq!(program.get_instruction_counter(), Some(1));
    }

    #[test]
    fn test_program_increment_instruction_counter() {
        let mut program = Program::default();
        program.push(Instruction::NoOp);
        program.push(Instruction::NoOp);

        assert_eq!(program.instructions.len(), 3);

        assert_eq!(program.instruction_counter, Some(0));

        program.increment_instruction_counter();
        assert_eq!(program.instruction_counter, Some(1));

        program.increment_instruction_counter();
        assert_eq!(program.instruction_counter, Some(2));

        program.increment_instruction_counter();
        assert_eq!(program.get_instruction_counter(), None);
    }

    #[test]
    fn test_program_get_program_length() {
        let mut program = Program::new();
        assert_eq!(program.get_program_length(), None);

        let instructions = vec![Instruction::NoOp];

        program.load(instructions);
        assert_eq!(program.get_program_length(), Some(1));

        program.push(Instruction::NoOp);
        assert_eq!(program.get_program_length(), Some(2));
    }

    #[test]
    fn test_program_default() {
        let program = Program::default();

        assert_eq!(program.instructions.len(), 1);
        assert_eq!(program.get_program_length(), Some(1));
        assert_eq!(program.instruction_counter, Some(0));
        assert_eq!(program.get_instruction_counter(), Some(1));
        assert_eq!(program.get_instruction(), Some(Instruction::NoOp));
    }

    #[test]
    fn test_program_display() {
        let mut program = Program::new();
        let instructions = vec![Instruction::NoOp];

        program.load(instructions);
        assert_eq!(program.to_string(), "0: NOOP\n");

        program.push(Instruction::NoOp);
        assert_eq!(program.to_string(), "0: NOOP\n1: NOOP\n");
    }

    #[test]
    fn test_program_push_new() {
        let mut program = Program::new();

        program.push(Instruction::NoOp);

        assert_eq!(program.instructions.len(), 1);
        assert_eq!(program.instruction_counter, Some(0));
    }

    #[test]
    fn test_program_push() {
        let mut program = Program::new();
        program.load_from_string(">>++<<--");
        program.push(Instruction::NoOp);

        assert_eq!(program.instructions.len(), 9);
    }

    #[test]
    fn test_program_find_matching_bracket() {
        let mut program = Program::new();
        let instructions = "[]";

        program.load_from_string(instructions);

        assert_eq!(program.find_matching_bracket(), Some(1));
    }

    #[test]
    fn test_program_find_matching_bracket_nested() {
        let mut program = Program::new();
        let instructions = "[[]]";

        program.load_from_string(instructions);

        assert_eq!(program.find_matching_bracket(), Some(3));
    }
}
