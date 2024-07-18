// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::{
    vm_reader::VMReader,
    Program,
    VirtualMachine,
};

/// `VirtualMachineBuilder` is a builder for the `VirtualMachine` struct.
///
/// This builder allows you to set the `program` and `tape_size` for a
/// `VirtualMachine` before building it. Both `program` and `tape_size` are
/// optional. If they're not provided, the `VirtualMachine` will be initialized
/// with default values.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::{
///     Program,
///     VMReader,
///     VirtualMachineBuilder,
/// };
/// let program = Program::default();
/// let input_device = std::io::stdin();
/// let vm = VirtualMachineBuilder::new()
///     .program(program)
///     .tape_size(1024)
///     .input_device(input_device)
///     .build()
///     .unwrap();
/// ```
#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct VirtualMachineBuilder<R>
where
    R: VMReader,
{
    /// The program that the `VirtualMachine` will execute. If not provided,
    /// the `VirtualMachine` will be initialized with a default program.
    program: Option<Program>,

    /// The size of the tape for the `VirtualMachine`. If not provided,
    /// the `VirtualMachine` will be initialized with a default tape size.
    tape_size: Option<usize>,

    /// The input device for the `VirtualMachine`. If not provided,
    /// the `VirtualMachine` will be initialized with a STDIN as the input
    /// device.
    input_device: Option<R>,
}

impl<R> VirtualMachineBuilder<R>
where
    R: VMReader,
{
    /// Creates a new `VirtualMachineBuilder` with empty values.
    ///
    /// This function returns a new `VirtualMachineBuilder` with `program` and
    /// `tape_size` set to `None`. These values can be set later using the
    /// builder's methods.
    ///
    /// # Returns
    ///
    /// A new `VirtualMachineBuilder` struct with empty values.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Stdin;
    ///
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachineBuilder,
    /// };
    ///
    /// let builder = VirtualMachineBuilder::<Stdin>::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            program:      None,
            tape_size:    None,
            input_device: None,
        }
    }

    /// Set the program to be run by the virtual machine.
    ///
    /// # Arguments
    ///
    /// * `program` - The program to be run by the virtual machine.
    ///
    /// # Returns
    ///
    /// * Builder by value with the program set.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Program,
    ///     VMReader,
    ///     VirtualMachineBuilder,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let program = Program::from("++++++[>++++++++++<-]>+++++.");
    /// let vm = VirtualMachineBuilder::new()
    ///     .input_device(input_device)
    ///     .program(program)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(vm.program(), Program::from("++++++[>++++++++++<-]>+++++."));
    /// ```
    #[must_use]
    pub fn program(mut self, program: Program) -> Self {
        self.program = Some(program);
        self
    }

    /// Set the size of the tape to be used by the virtual machine.
    /// The default size is 30,000.
    ///
    /// # Arguments
    ///
    /// * `tape_size` - The size of the tape to be used by the virtual machine.
    ///
    /// # Returns
    ///
    /// * Builder by value with the tape size set.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachineBuilder,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let vm = VirtualMachineBuilder::new()
    ///     .input_device(input_device)
    ///     .tape_size(100)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(vm.length(), 100);
    /// ```
    #[must_use]
    pub const fn tape_size(mut self, tape_size: usize) -> Self {
        self.tape_size = Some(tape_size);
        self
    }

    /// Set the input device to be used by the virtual machine.
    ///
    /// The default input device is `stdin`.
    ///
    /// # Arguments
    ///
    /// * `input_device` - The input device to be used by the virtual machine.
    ///
    /// # Returns
    ///
    /// * Builder by value with the input device set.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     VMReader,
    ///     VirtualMachineBuilder,
    /// };
    ///
    /// let input_device = std::io::stdin();
    ///
    /// let mut vm = VirtualMachineBuilder::new()
    ///     .input_device(input_device)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     vm.input_device().get_vmreader_type(),
    ///     brainfoamkit_lib::VMReaderType::Stdin
    /// );
    /// ```
    #[must_use]
    pub fn input_device(mut self, input_device: R) -> Self {
        self.input_device = Some(input_device);
        self
    }

    /// Build the virtual machine.
    ///
    /// # Returns
    ///
    /// * A `Result` containing either a `VirtualMachine` or an `Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Program,
    ///     VMReader,
    ///     VirtualMachineBuilder,
    /// };
    ///
    /// let input_device = std::io::stdin();
    /// let program = Program::from("++++++[>++++++++++<-]>+++++.");
    /// let vm = VirtualMachineBuilder::new()
    ///     .program(program)
    ///     .tape_size(100)
    ///     .input_device(input_device)
    ///     .build();
    /// ```
    ///
    /// # Errors
    ///
    /// * If the input device is not set, this function will return an error.
    pub fn build(self) -> Result<VirtualMachine<R>> {
        let program = self.program.unwrap_or_default();
        let tape_size = self.tape_size.unwrap_or(30000);
        let Some(input_device) = self.input_device else {
            return Err(anyhow::anyhow!("Input device not set."));
        };

        Ok(VirtualMachine::new(tape_size, program, 0, 0, input_device))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm_reader::{
        MockReader,
        VMReaderType,
    };

    #[test]
    fn test_program() {
        let program = Program::from("++++++[>++++++++++<-]>+++++.");
        let input_device = MockReader {
            data: std::io::Cursor::new("A".as_bytes().to_vec()),
        };
        let vm = VirtualMachine::builder()
            .input_device(input_device)
            .program(program)
            .build()
            .unwrap();
        assert_eq!(vm.program(), Program::from("++++++[>++++++++++<-]>+++++."));
    }

    #[test]
    fn test_tape_size() {
        let input_device = MockReader {
            data: std::io::Cursor::new("A".as_bytes().to_vec()),
        };
        let vm = VirtualMachine::builder()
            .input_device(input_device)
            .tape_size(100)
            .build()
            .unwrap();
        assert_eq!(vm.tape_size(), 100);
    }

    #[test]
    fn test_input_device() {
        let input_device = MockReader {
            data: std::io::Cursor::new("A".as_bytes().to_vec()),
        };
        let mut vm = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        assert_eq!(vm.input_device().get_vmreader_type(), VMReaderType::Mock);
    }

    #[test]
    fn test_build() {
        let program = Program::from("++++++[>++++++++++<-]>+++++.");
        let input_device = MockReader {
            data: std::io::Cursor::new("A".as_bytes().to_vec()),
        };
        let vm = VirtualMachine::builder()
            .input_device(input_device)
            .tape_size(100)
            .program(program)
            .build()
            .unwrap();
        assert_eq!(vm.program(), Program::from("++++++[>++++++++++<-]>+++++."));
        assert_eq!(vm.tape_size(), 100);
    }

    #[test]
    fn test_default() {
        let input_device = MockReader {
            data: std::io::Cursor::new("A".as_bytes().to_vec()),
        };
        let vm = VirtualMachine::builder()
            .input_device(input_device)
            .build()
            .unwrap();
        assert_eq!(vm.program(), Program::default());
        assert_eq!(vm.tape_size(), 30000);
    }
}
