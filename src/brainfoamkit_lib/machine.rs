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

use crate::{Byte, Instruction, Program};

/// `BrainfoamKitMachine` is a struct representing a Virtual Machine capable of interpreting
/// a BrainFuck program and tracking its state.
///
/// # Fields
///
/// * `tape`: A vector of `Byte` values representing the memory of the machine. Each `Byte` in the vector is a cell in the memory tape.
/// * `pointer`: A `usize` value representing the current position of the memory pointer. The memory pointer points to a given cell in the memory tape.
/// * `program`: A `Program` instance representing the Brainfuck program that the machine is executing.
///
/// # Example
///
/// ```
/// use brainfoamkit_lib::BrainfoamKitMachine;
///
/// let machine = BrainfoamKitMachine::default();
/// ```
#[allow(clippy::module_name_repetitions)]
pub struct BrainfoamKitMachine {
    tape: Vec<Byte>,
    pointer: usize,
    program: Program,
}

#[allow(dead_code)]
#[allow(clippy::len_without_is_empty)] //FIXME - Add an `is_empty` method
impl BrainfoamKitMachine {
    /// Creates a new `BrainfoamKitMachine` with a specified length.
    ///
    /// This method initializes the `tape` and `program` fields with vectors of the specified length. The `tape` vector is initialized with default `Byte` values, and the `program` vector is initialized with `NoOp` instructions. The `pointer` field is initialized to `0`.
    ///
    /// # Arguments
    ///
    /// * `length`: The length of the `tape` and `program` vectors.
    ///
    /// # Returns
    ///
    /// A new `BrainfoamKitMachine` instance with the specified length.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{BrainfoamKitMachine, Instruction, Program};
    ///
    /// let length = 10;
    /// let machine = BrainfoamKitMachine::new(length);
    /// assert_eq!(machine.len(), length);
    /// assert_eq!(machine.pointer(), 0);
    /// ```
    pub fn new(length: usize) -> Self {
        let program = Program::from(vec![Instruction::NoOp; length]);
        Self {
            tape: vec![Byte::default(); length],
            pointer: 0,
            program,
        }
    }

    /// Loads a `Program` into the `BrainfoamKitMachine`.
    ///
    /// This method replaces the current `program` of the `BrainfoamKitMachine` with the specified `Program`.
    ///
    /// # Arguments
    ///
    /// * `program`: The `Program` to load into the `BrainfoamKitMachine`.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{BrainfoamKitMachine, Instruction, Program};
    ///
    /// let mut machine = BrainfoamKitMachine::new(10);
    /// let program = Program::from(vec![Instruction::IncrementPointer, Instruction::IncrementValue]);
    /// machine.load(program);
    /// assert_eq!(machine.get_instruction(), Some(Instruction::IncrementPointer));
    /// assert_eq!(machine.pointer(), 0);
    /// machine.execute_instruction();
    /// assert_eq!(machine.pointer(), 1);
    /// assert_eq!(machine.get_instruction(), Some(Instruction::IncrementValue));
    /// ```
    pub fn load(&mut self, program: Program) {
        self.program = program;
    }

    /// Returns the length of the `BrainfoamKitMachine`.
    ///
    /// This method returns the length of the `tape` and `program` vectors of the `BrainfoamKitMachine`.
    ///
    /// # Returns
    ///
    /// A `usize` value representing the length of the `BrainfoamKitMachine`.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::BrainfoamKitMachine;
    ///
    /// let machine = BrainfoamKitMachine::new(10);
    /// assert_eq!(machine.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.tape.len()
    }

    /// Returns the current position of the memory pointer.
    ///
    /// This method returns the current position of the memory pointer in the `BrainfoamKitMachine`.
    ///
    /// # Returns
    ///
    /// A `usize` value representing the current position of the memory pointer.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::BrainfoamKitMachine;
    ///
    /// let machine = BrainfoamKitMachine::new(10);
    /// assert_eq!(machine.pointer(), 0);
    /// ```

    pub fn pointer(&self) -> usize {
        self.pointer
    }

    /// Returns the current instruction of the `BrainfoamKitMachine`.
    ///
    /// This method returns the instruction at the current position of the memory pointer in the program. If the memory pointer is out of bounds of the program, this method returns `None`.
    ///
    /// # Returns
    ///
    /// An `Option` that contains the current instruction if the memory pointer is within the bounds of the program, or `None` if the memory pointer is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{BrainfoamKitMachine, Instruction, Program};
    ///
    /// let mut machine = BrainfoamKitMachine::new(10);
    /// let program = Program::from(vec![Instruction::IncrementPointer, Instruction::IncrementValue]);
    /// machine.load(program);
    /// assert_eq!(machine.get_instruction(), Some(Instruction::IncrementPointer));
    /// assert_eq!(machine.get_instruction(), Some(Instruction::IncrementValue));
    /// assert_eq!(machine.get_instruction(), None);
    /// ```
    pub fn get_instruction(&self) -> Option<Instruction> {
        match self.program.length() {
            Some(length) if self.pointer < length => Some(self.program[self.pointer]),
            _ => None,
        }
    }

    /// Executes the current instruction of the `BrainfoamKitMachine`.
    ///
    /// This method executes the instruction at the current position of the memory pointer in the program. If the memory pointer is out of bounds of the program, this method does nothing.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{BrainfoamKitMachine, Instruction, Program};
    ///
    /// let mut machine = BrainfoamKitMachine::new(10);
    /// let program = Program::from(vec![Instruction::IncrementPointer, Instruction::IncrementValue]);
    /// machine.load(program);
    /// assert_eq!(machine.pointer(), 0);
    /// machine.execute_instruction();
    /// assert_eq!(machine.pointer(), 1);
    /// machine.execute_instruction();
    /// assert_eq!(machine.pointer(), 1);
    /// ```
    ///
    pub fn execute_instruction(&mut self) {
        let current_instruction = self.get_instruction().unwrap_or(Instruction::NoOp);
        match current_instruction {
            Instruction::IncrementPointer => self.increment_pointer(),
            Instruction::DecrementPointer => self.decrement_pointer(),
            Instruction::IncrementValue => self.increment_value(),
            Instruction::DecrementValue => self.decrement_value(),
            Instruction::OutputValue => self.output_value(),
            Instruction::InputValue => self.input_value(),
            Instruction::JumpForward => self.jump_forward(),
            Instruction::JumpBackward => self.jump_backward(),
            Instruction::NoOp => {}
        }
    }

    fn increment_pointer(&mut self) {
        self.pointer += 1;
    }

    fn decrement_pointer(&mut self) {
        self.pointer -= 1;
    }

    fn increment_value(&mut self) {
        let mut value = self.tape[self.pointer];
        value.increment();
    }

    fn decrement_value(&mut self) {
        let mut value = self.tape[self.pointer];
        value.decrement();
    }

    fn output_value(&mut self) {
        todo!("Implement output_value")
    }

    fn input_value(&mut self) {
        todo!("Implement input_value")
    }

    fn jump_forward(&mut self) {
        todo!("Implement jump_forward")
    }

    fn jump_backward(&mut self) {
        todo!("Implement jump_backward")
    }
}

impl Default for BrainfoamKitMachine {
    fn default() -> Self {
        Self::new(30000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_new() {
        let machine: BrainfoamKitMachine = BrainfoamKitMachine::new(30000);
        assert_eq!(machine.tape.len(), 30000);
        assert_eq!(machine.pointer, 0);
    }

    #[test]
    fn test_machine_load() {
        let mut machine = BrainfoamKitMachine::new(30000);
        let instructions = vec![
            Instruction::IncrementPointer,
            Instruction::DecrementPointer,
            Instruction::IncrementValue,
            Instruction::DecrementValue,
            Instruction::OutputValue,
            Instruction::InputValue,
            Instruction::JumpForward,
            Instruction::JumpBackward,
            Instruction::NoOp,
        ];
        let program = Program::from(instructions);
        machine.load(program);
        assert_eq!(machine.program.length(), Some(9));
    }

    #[test]
    fn test_machine_get_instruction() {
        let mut machine = BrainfoamKitMachine::new(30000);
        let instructions = vec![
            Instruction::IncrementPointer,
            Instruction::DecrementPointer,
            Instruction::IncrementValue,
            Instruction::DecrementValue,
            Instruction::OutputValue,
            Instruction::InputValue,
            Instruction::JumpForward,
            Instruction::JumpBackward,
            Instruction::NoOp,
        ];
        let program = Program::from(instructions);
        machine.load(program);
        assert_eq!(
            machine.get_instruction(),
            Some(Instruction::IncrementPointer)
        );
        machine.pointer = 9;
        assert_eq!(machine.get_instruction(), None);
    }

    #[test]
    fn test_machine_execute_instruction() {
        let mut machine = BrainfoamKitMachine::new(30000);
        let instructions = vec![
            Instruction::IncrementPointer,
            Instruction::DecrementPointer,
            Instruction::IncrementValue,
            Instruction::DecrementValue,
            Instruction::OutputValue,
            Instruction::InputValue,
            Instruction::JumpForward,
            Instruction::JumpBackward,
            Instruction::NoOp,
        ];
        let program = Program::from(instructions);
        machine.load(program);
        machine.execute_instruction();
        assert_eq!(machine.pointer, 1);
        machine.execute_instruction();
        assert_eq!(machine.pointer, 0);
        machine.execute_instruction();
        assert_eq!(machine.tape[0], Byte::default());
        machine.execute_instruction();
        assert_eq!(machine.tape[0], Byte::default());
        machine.execute_instruction();
        machine.execute_instruction();
        machine.execute_instruction();
        machine.execute_instruction();
        machine.execute_instruction();
        machine.execute_instruction();
        machine.execute_instruction();
    }
}
