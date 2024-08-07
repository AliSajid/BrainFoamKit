// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{
    self,
    Display,
    Formatter,
};

/// All possible instructions that can be understood by the interpreter
///
/// This enum is at the heart of the interpreter. This enumerates
/// all eight instructions that a brainfuck program can be composed of
/// in addition to a ninth No-Op instruction
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::Instruction;
///
/// let incrptr = Instruction::from_char('>');
/// let decrptr = Instruction::from_char('<');
///
/// assert_eq!(incrptr, Instruction::IncrementPointer);
/// assert_eq!(decrptr, Instruction::DecrementPointer);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    /// Instruction to Increment the Pointer
    ///
    /// Internal representation of the `>` instruction.
    IncrementPointer,
    /// Instruction to Decrement the Pointer
    ///
    /// Internal representation of the `<` instruction.
    DecrementPointer,
    /// Instruction to Increment the Value stored in memory location
    ///
    /// Internal representation of the `+` instruction.
    IncrementValue,
    /// Instruction to Increment the Value stored in memory location
    ///
    /// Internal representation of the `-` instruction.
    DecrementValue,
    /// Instruction to Output the currently stored value to the external
    /// interface
    ///
    /// Internal representation of the `.` instruction.
    OutputValue,
    /// Instruction to Input the currently available value at the external
    /// interface
    ///
    /// Internal representation of the `,` instruction.
    InputValue,
    /// Instruction to Start a loop if the current value is non-zero
    ///
    /// Internal representation of the `[` instruction.
    JumpForward,
    /// Instruction to Restart a loop if the current value is non-zero
    ///
    /// Internal representation of the `]` instruction.
    JumpBackward,
    /// Instruction to do nothing
    ///
    /// This does not have a corresponding instruction in `BrainFuck`
    NoOp,
}

impl Instruction {
    /// Convert a char to an Instruction
    ///
    /// This method takes in a a single instruction (character
    /// and converts that into the instruction.
    ///
    /// This ignores any instructions not in the standard alphabet
    /// of `BrainFuck` and counts them as No-Ops
    ///
    /// The following table and [associated github repository](https://github.com/AliSajid/BrainFoamKit/tree/alpha/lang#brainfuck-syntax)
    /// show the entire syntax.
    ///
    /// | Symbol | Effect |
    /// | :------: | :------ |
    /// | `+` | Increment the value in the current memory cell|
    /// | `-` | Decrement the value in the current memory cell|
    /// | `>` | Move the memory pointer to the right|
    /// | `<` | Move the memory pointer to the left|
    /// | `[` | Begin a loop: continue if value in memory cell is nonzero|
    /// | `]` | End a loop: jump back to corresponding `[` if value in memory cell is nonzero|
    /// | `.` | Output the ASCII value in the current memory cell|
    /// | `,` | Read a single ASCII character and store it in the current memory cell|
    /// | _ | Everything else is a No-Up|
    ///
    /// # Argument
    ///
    /// * `c` - A single character from the `BrainFuck` list of command
    ///   characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::Instruction;
    ///
    /// let instruction = Instruction::from_char('+');
    ///
    /// assert_eq!(instruction, Instruction::IncrementValue)
    /// ```
    ///
    /// # Returns
    ///
    /// The appropriate variant of the `Instruction` enum
    ///
    /// # Notes
    ///
    /// This version of `Instruction` treats every character other than the
    /// eight specific characters as `NoOp`s
    #[must_use]
    pub const fn from_char(c: char) -> Self {
        match c {
            '>' => Self::IncrementPointer,
            '<' => Self::DecrementPointer,
            '+' => Self::IncrementValue,
            '-' => Self::DecrementValue,
            '.' => Self::OutputValue,
            ',' => Self::InputValue,
            '[' => Self::JumpForward,
            ']' => Self::JumpBackward,
            _ => Self::NoOp,
        }
    }
}

/// Convert an instruction to a String
///
/// This method converts a given instruction to a human-readable
/// instruction.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::{
///     Instruction,
///     Nybble,
/// };
///
/// let instruction = Instruction::from_char('>');
///
/// assert_eq!(instruction.to_string(), "INCPTR");
/// ```
///
/// # Returns
///
/// The instruction as a string.
///
/// # See Also
///
/// * [`from_char()`](#method.from_char): Creates a new Instruction from a
///   string.
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::IncrementPointer => write!(f, "INCPTR"),
            Self::DecrementPointer => write!(f, "DECPTR"),
            Self::IncrementValue => write!(f, "INCVAL"),
            Self::DecrementValue => write!(f, "DECVAL"),
            Self::OutputValue => write!(f, "OUTVAL"),
            Self::InputValue => write!(f, "INPVAL"),
            Self::JumpForward => write!(f, "JMPFWD"),
            Self::JumpBackward => write!(f, "JMPBCK"),
            Self::NoOp => write!(f, "NOOP"),
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
        assert_eq!(format!("{}", Instruction::NoOp), "NOOP");
    }
}
