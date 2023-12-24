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

use crate::{
    Program,
    VirtualMachine,
};

#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct VirtualMachineBuilder {
    program:   Option<Program>,
    tape_size: Option<usize>,
}

impl VirtualMachineBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            program:   None,
            tape_size: None,
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
    ///     VirtualMachineBuilder,
    /// };
    ///
    /// let program = Program::from_string("++++++[>++++++++++<-]>+++++.");
    /// let vm = VirtualMachineBuilder::new().program(program).build();
    ///
    /// assert_eq!(
    ///     vm.program(),
    ///     Program::from_string("++++++[>++++++++++<-]>+++++.")
    /// );
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
    /// use brainfoamkit_lib::VirtualMachineBuilder;
    ///
    /// let vm = VirtualMachineBuilder::new().tape_size(100).build();
    ///
    /// assert_eq!(vm.length(), 100);
    /// ```
    #[must_use]
    pub const fn tape_size(mut self, tape_size: usize) -> Self {
        self.tape_size = Some(tape_size);
        self
    }

    /// Build the virtual machine.
    ///
    /// # Returns
    ///
    /// * The virtual machine.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     Program,
    ///     VirtualMachineBuilder,
    /// };
    ///
    /// let program = Program::from_string("++++++[>++++++++++<-]>+++++.");
    /// let vm = VirtualMachineBuilder::new()
    ///     .program(program)
    ///     .tape_size(100)
    ///     .build();
    /// ```
    #[must_use]
    pub fn build(self) -> VirtualMachine {
        let program = self.program.unwrap_or_default();
        let tape_size = self.tape_size.unwrap_or(30000);
        VirtualMachine::new(tape_size, program, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let program = Program::from_string("++++++[>++++++++++<-]>+++++.");
        let vm = VirtualMachine::builder().program(program).build();
        assert_eq!(
            vm.program(),
            Program::from_string("++++++[>++++++++++<-]>+++++.")
        );
    }

    #[test]
    fn test_tape_size() {
        let vm = VirtualMachine::builder().tape_size(100).build();
        assert_eq!(vm.tape_size(), 100);
    }

    #[test]
    fn test_build() {
        let program = Program::from_string("++++++[>++++++++++<-]>+++++.");
        let vm = VirtualMachine::builder()
            .tape_size(100)
            .program(program)
            .build();
        assert_eq!(
            vm.program(),
            Program::from_string("++++++[>++++++++++<-]>+++++.")
        );
        assert_eq!(vm.tape_size(), 100);
    }
}
