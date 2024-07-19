// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
    vm_reader::VMReader,
    Byte,
    Instruction,
    Program,
    VirtualMachineBuilder,
};

/// `VirtualMachine` is a struct representing a Virtual Machine capable of
/// interpreting a `BrainFuck` program and tracking its state.
///
/// # Fields
///
/// * `tape`: A vector of `Byte` values representing the memory of the machine.
///   Each `Byte` in the vector is a cell in the memory tape.
/// * `program`: A `Program` instance representing the Brainfuck program that
///   the machine is executing.
/// * `memory_pointer`: A `usize` value representing the current position of the
///   memory pointer. The memory pointer points to a given cell in the memory
///   tape.
/// * `program_counter`: A `usize` that represents which instruction of the
///   `Program` is being executed right now.
///
/// # Example
///
/// ```
/// use brainfoamkit_lib::{
///     VMReader,
///     VirtualMachine,
/// };
///
/// let input_device = std::io::stdin();
/// let machine = VirtualMachine::builder().input_device(input_device).build();
/// ```
#[allow(clippy::module_name_repetitions)]
pub struct VirtualMachine<R>
where
    R: VMReader,
{
    tape:            Vec<Byte>,
    program:         Program,
    memory_pointer:  usize,
    program_counter: usize,
    input:           R,
    //    output: W,
}

#[allow(dead_code)]
#[allow(clippy::len_without_is_empty)]
impl<R> VirtualMachine<R>
where
    R: VMReader,
{
    pub(crate) fn new(
        tape_size: usize,
        program: Program,
        memory_pointer: usize,
        program_counter: usize,
        input: R,
    ) -> Self {
        // FIXME - Remove `memory_pointer` and `program_counter` from the constructor
        // since they should always be set to 0 on initialization.

        Self {
            tape: vec![Byte::default(); tape_size],
            program,
            memory_pointer,
            program_counter,
            input,
        }
    }

    /// Return the length of the "memory" or the `tape_size` of the
    /// `VirtualMachine`.
    ///
    /// This method is an alias for the [`length`](#method.length) method.
    ///
    /// # Returns
    ///
    /// A `usize` value representing the length of the `VirtualMachine`.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .tape_size(10)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(machine.length(), 10);
    /// ```
    ///
    /// # See Also
    ///
    /// * [`length`](#method.length)
    /// * [`memory_pointer`](#method.memory_pointer)
    /// * [`program_counter`](#method.program_counter)
    #[must_use]
    pub(crate) fn tape_size(&self) -> usize {
        self.length()
    }

    /// Return the `Program` of the `VirtualMachine`.
    ///
    /// This method returns the `Program` of the `VirtualMachine`.
    ///
    /// # Returns
    ///
    /// A `Program` instance representing the `Program` of the `VirtualMachine`.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Program,
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(machine.program(), Program::default());
    /// ```
    #[must_use]
    pub fn program(&self) -> Program {
        self.program.clone()
    }

    /// Create a new instance of `VirtualMachine` using `VirtualMachineBuilder`.
    ///
    /// This method provides a convenient way to create a new instance of
    /// `VirtualMachine` using `VirtualMachineBuilder`. This method returns
    /// a `VirtualMachineBuilder` instance that can be used to configure the
    /// `VirtualMachine` before building it.
    ///
    /// # Returns
    ///
    /// A `VirtualMachineBuilder` instance that can be used to configure the
    /// `VirtualMachine` before building it.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let input_device = std::io::stdin();
    ///
    /// let machine = VirtualMachine::builder().input_device(input_device).build();
    /// ```
    ///
    /// # See Also
    ///
    /// * [`VirtualMachineBuilder`](struct.VirtualMachineBuilder.html)
    #[must_use]
    pub const fn builder() -> VirtualMachineBuilder<R> {
        VirtualMachineBuilder::<R>::new()
    }

    /// Returns the length of the `tape` inside the `VirtualMachine`.
    ///
    /// This method returns the length of the `tape` vector of the
    /// `VirtualMachine`.
    ///
    /// # Returns
    ///
    /// A `usize` value representing the length of the `VirtualMachine`.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .tape_size(10)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(machine.length(), 10);
    /// ```
    #[must_use]
    pub fn length(&self) -> usize {
        self.tape.len()
    }

    /// Returns the current position of the memory pointer.
    ///
    /// This method returns the current position of the memory pointer in the
    /// `VirtualMachine`.
    ///
    /// # Returns
    ///
    /// A `usize` value representing the current position of the memory pointer.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(machine.memory_pointer(), 0);
    /// ```
    #[must_use]
    pub const fn memory_pointer(&self) -> usize {
        self.memory_pointer
    }

    /// Returns the current position of the program counter.
    ///
    /// This method returns the current position of the program counter in the
    /// `VirtualMachine`.
    ///
    /// # Returns
    ///
    /// A `usize` value representing the current position of the program
    /// counter.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(machine.program_counter(), 0);
    /// ```
    #[must_use]
    pub const fn program_counter(&self) -> usize {
        self.program_counter
    }

    /// returns the current input device of the `VirtualMachine`.
    ///
    /// This method returns the current input device of the `VirtualMachine`.
    /// This allows for testing and type checking of the input device.
    ///
    /// # Returns
    ///
    /// A reference to the current input device of the
    /// `VirtualMachine`.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     MockReader,
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let input_device = MockReader {
    ///     data: std::io::Cursor::new("A".as_bytes().to_vec()),
    /// };
    /// let mut machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(machine.input_device().read().unwrap(), 65);
    /// ```
    ///
    /// # See Also
    ///
    /// * [`VMReader`](trait.VMReader.html)
    /// * [`VirtualMachineBuilder`](struct.VirtualMachineBuilder.html)
    #[must_use]
    pub fn input_device(&mut self) -> &mut R {
        &mut self.input
    }

    /// Returns the current instruction of the `VirtualMachine`.
    ///
    /// This method returns the instruction at the current position of the
    /// program counter in the program. If the program counter is out of
    /// bounds of the program, this method returns `None`.
    ///
    /// # Returns
    ///
    /// An `Option` that contains the current instruction if the program counter
    /// is within the bounds of the program, or `None` if the program
    /// counter is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Instruction,
    ///     Program,
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let program = Program::from(vec![
    ///     Instruction::IncrementPointer,
    ///     Instruction::IncrementValue,
    /// ]);
    /// let input_device = std::io::stdin();
    /// let mut machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .program(program)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(
    ///     machine.get_instruction(),
    ///     Some(Instruction::IncrementPointer)
    /// );
    /// machine.execute_instruction();
    /// assert_eq!(machine.get_instruction(), Some(Instruction::IncrementValue));
    /// machine.execute_instruction();
    /// assert_eq!(machine.get_instruction(), None);
    /// ```
    #[must_use]
    pub fn get_instruction(&self) -> Option<Instruction> {
        self.program.get_instruction(self.program_counter)
    }

    /// Executes the current instruction of the `VirtualMachine`.
    ///
    /// This method executes the instruction at the current position of the
    /// memory pointer in the program. If the memory pointer is out of bounds of
    /// the program, this method does nothing.
    ///
    /// # Example
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Instruction,
    ///     Program,
    ///     VMReader,
    ///     VirtualMachine,
    /// };
    ///
    /// let program = Program::from(vec![
    ///     Instruction::IncrementPointer,
    ///     Instruction::IncrementValue,
    /// ]);
    /// let input_device = std::io::stdin();
    /// let mut machine = VirtualMachine::builder()
    ///     .input_device(input_device)
    ///     .program(program)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(machine.memory_pointer(), 0);
    /// machine.execute_instruction();
    /// assert_eq!(machine.memory_pointer(), 1);
    /// machine.execute_instruction();
    /// assert_eq!(machine.memory_pointer(), 1);
    /// ```
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
        self.program_counter += 1;
    }

    fn increment_pointer(&mut self) {
        let next = self.memory_pointer.checked_add(1);
        if let Some(next) = next {
            self.memory_pointer = next;
        } else {
            self.memory_pointer = 0;
        }
    }

    fn decrement_pointer(&mut self) {
        let next = self.memory_pointer.checked_sub(1);
        if let Some(next) = next {
            self.memory_pointer = next;
        } else {
            self.memory_pointer = self.tape.len() - 1;
        }
    }

    fn increment_value(&mut self) {
        self.tape[self.memory_pointer].increment();
    }

    fn decrement_value(&mut self) {
        self.tape[self.memory_pointer].decrement();
    }

    fn output_value(&self) {
        todo!("Implement output_value")
    }

    fn input_value(&mut self) {
        let input = self.input.read();
        if let Ok(input) = input {
            self.tape[self.memory_pointer] = Byte::from(input);
        }
    }

    fn jump_forward(&self) {
        todo!("Implement jump_forward")
    }

    fn jump_backward(&self) {
        todo!("Implement jump_backward")
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use crate::vm_reader::MockReader;

    #[test]
    fn test_machine_get_instruction() {
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
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let machine = VirtualMachine::builder()
            .input_device(input_device)
            .program(program)
            .build()
            .unwrap();
        assert_eq!(
            machine.get_instruction(),
            Some(Instruction::IncrementPointer)
        );
    }

    #[test]
    fn test_machine_execute_instruction() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let program = Program::from(vec![
            Instruction::IncrementPointer,
            Instruction::IncrementValue,
            Instruction::DecrementValue,
            Instruction::DecrementPointer,
        ]);
        let mut machine = VirtualMachine::builder()
            .input_device(input_device)
            .program(program)
            .build()
            .unwrap();

        machine.execute_instruction();
        assert_eq!(
            machine.memory_pointer(),
            1,
            "Memory pointer should be incremented"
        );
        assert_eq!(
            machine.program_counter(),
            1,
            "Program counter should be incremented"
        );

        machine.execute_instruction();
        assert_eq!(
            machine.tape[1],
            Byte::from(0b0000_0001),
            "Value at memory pointer should be incremented"
        );
        assert_eq!(
            machine.memory_pointer(),
            1,
            "Memory pointer should not be changed"
        );
        assert_eq!(
            machine.program_counter(),
            2,
            "Program counter should be incremented"
        );

        machine.execute_instruction();
        assert_eq!(
            machine.tape[1],
            Byte::from(0),
            "Value at memory pointer should be decremented"
        );
        assert_eq!(
            machine.memory_pointer(),
            1,
            "Memory pointer should not be decremented"
        );
        assert_eq!(
            machine.program_counter(),
            3,
            "Program counter should be incremented"
        );

        machine.execute_instruction();
        assert_eq!(
            machine.memory_pointer(),
            0,
            "Memory pointer should be decremented"
        );
        assert_eq!(
            machine.program_counter(),
            4,
            "Program counter should be incremented"
        );
    }

    #[test]
    fn test_memory_pointer() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        assert_eq!(
            machine.memory_pointer(),
            0,
            "Memory pointer should be initialized to 0"
        );
    }

    #[test]
    fn test_program_counter() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        assert_eq!(
            machine.program_counter(),
            0,
            "Program counter should be initialized to 0"
        );
    }

    #[test]
    fn test_increment_pointer() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let mut machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        machine.increment_pointer();
        assert_eq!(
            machine.memory_pointer(),
            1,
            "Memory pointer should be incremented"
        );
    }

    #[test]
    fn test_decrement_pointer() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let mut machine = VirtualMachine::builder()
            .input_device(input_device)
            .tape_size(100)
            .build()
            .unwrap();
        machine.decrement_pointer();
        assert_eq!(
            machine.memory_pointer(),
            99,
            "Memory pointer should be decremented"
        );
    }

    #[test]
    fn test_increment_value() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let mut machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        let increment_result = Byte::from(1);

        machine.increment_value();
        assert_eq!(
            machine.tape[0], increment_result,
            "Value at memory pointer should be incremented"
        );
    }

    #[test]
    fn test_decrement_value() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let mut machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        machine.tape[0] = Byte::from(1);
        machine.decrement_value();
        assert_eq!(
            machine.tape[0],
            Byte::from(0),
            "Value at memory pointer should be decremented"
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_output_value() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        machine.output_value();
    }

    #[test]
    fn test_valid_input_value() {
        let data = vec![65]; // A's ASCII value is 65
        let input_device = MockReader {
            data: Cursor::new(data),
        };
        let mut machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();

        machine.input_value();

        assert_eq!(
            machine.tape[0],
            Byte::from(65),
            "Value at memory pointer should be set to the input value"
        );
    }

    #[test]
    fn test_invalid_input_value() {
        let data = vec![129]; // 129 is not a valid ASCII value
        let input_device = MockReader {
            data: Cursor::new(data),
        };
        let mut machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();

        machine.input_value();

        assert_eq!(
            machine.tape[0],
            Byte::from(0),
            "Value at memory pointer should not be set to the input value"
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_jump_forward() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        machine.jump_forward();
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_jump_backward() {
        let input_device = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let machine = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        machine.jump_backward();
    }
}
