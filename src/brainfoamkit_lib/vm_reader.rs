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
    fs::File,
    io::{
        Cursor,
        Read,
        Stdin,
    },
};

use anyhow::{
    anyhow,
    Result,
};

/// Allowable types of `VMReader`
///
/// This enum is used to determine the type of `VMReader` that is being used.
///
/// The currently supported types are:
///
/// * Stdin - The standard input device as implemented by the [std::io::Stdin struct](https://doc.rust-lang.org/std/io/struct.Stdin.html)
/// * File - A file as implemented by the [std::fs::File struct](https://doc.rust-lang.org/std/fs/struct.File.html)
/// * Mock - A mock reader as implemented by the [`MockReader`
///   struct](struct.MockReader.html)
/// * Unknown - The default type of `VMReader`
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::VMReaderType;
///
/// let stdin = std::io::stdin();
/// let file = std::fs::File::open("test.bf");
/// let mock = brainfoamkit_lib::MockReader {
///     data: std::io::Cursor::new("A".as_bytes().to_vec()),
/// };
///
/// assert_eq!(stdin.get_type(), VMReaderType::Stdin);
/// assert_eq!(file.get_type(), VMReaderType::File);
/// assert_eq!(mock.get_type(), VMReaderType::Mock);
/// ```
///
/// # See Also
///
/// * [`VMReader`](trait.VMReader.html)
/// * [`MockReader`](struct.MockReader.html)
/// * [Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html)
/// * [File](https://doc.rust-lang.org/std/fs/struct.File.html)
#[derive(Debug, PartialEq, Eq)]
pub enum VMReaderType {
    /// The standard input device as implemented by the [std::io::Stdin struct](https://doc.rust-lang.org/std/io/struct.Stdin.html)
    Stdin,
    /// A file as implemented by the [std::fs::File struct](https://doc.rust-lang.org/std/fs/struct.File.html)
    File,
    /// A mock reader as implemented by the [MockReader
    /// struct](struct.MockReader.html)
    Mock,
    /// The default type of VMReader
    Unknown,
}
/// The `VMReader` trait
///
/// This trait is used to implement a `Reader` for the `VirtualMachine`. This
/// allows us to abstract over several different types of `Reader`s, including
/// `StdIn` and `File`. This trait is also implemented for the `MockReader`
/// struct, which is used for testing.
///
/// This is a restricted trait, meaning that it will only be implemented for
/// specific types. This is done to ensure that the `VMReader` is only
/// implemented for types that are valid for the `VirtualMachine`. The valid
/// types for `VMReader` are listed in the
/// [`VMReaderType`](enum.VMReaderType.html) enum.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::VMReader;
///
/// let stdin = std::io::stdin();
/// let file = std::fs::File::open("test.bf");
/// let mock = brainfoamkit_lib::MockReader {
///     data: std::io::Cursor::new("A".as_bytes().to_vec()),
/// };
///
/// assert_eq!(stdin.get_type(), brainfoamkit_lib::VMReaderType::Stdin);
/// assert_eq!(file.get_type(), brainfoamkit_lib::VMReaderType::File);
/// assert_eq!(mock.get_type(), brainfoamkit_lib::VMReaderType::Mock);
/// ```
///
/// # See Also
///
/// * [`VMReaderType`](enum.VMReaderType.html)
/// * [`MockReader`](struct.MockReader.html)
/// * [Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html)
/// * [File](https://doc.rust-lang.org/std/fs/struct.File.html)
pub trait VMReader {
    /// Read a single byte from the reader
    ///
    /// This function reads a single byte from the reader and returns it as a
    /// `u8` for use by the `VirtualMachine`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the byte read from the reader is
    /// not within the ASCII range.
    fn read(&mut self) -> Result<u8> {
        Ok(0)
    }

    /// Get the type of the reader
    ///
    /// This function returns the type of the reader as a `VMReaderType` enum.
    ///
    /// The currently supported types are:
    ///
    /// * Stdin - The standard input device as implemented by the [std::io::Stdin struct](https://doc.rust-lang.org/std/io/struct.Stdin.html)
    /// * File - A file as implemented by the [std::fs::File struct](https://doc.rust-lang.org/std/fs/struct.File.html)
    /// * Mock - A mock reader as implemented by the [`MockReader`
    ///   struct](struct.MockReader.html)
    /// * Unknown - The default type of `VMReader`
    ///
    /// The default type of `VMReader` is `Unknown`, and is used when the type
    /// of the reader is not set.
    fn get_vmreader_type(&self) -> VMReaderType {
        VMReaderType::Unknown
    }
}

/// The `MockReader` struct
///
/// This struct is used to implement a mock `Reader` for the `VirtualMachine`.
/// This allows for us to test the `VirtualMachine` without having to use
/// `Stdin` or `File` as the `Reader`.
///
/// This struct is used for testing purposes only, and should not be used in
/// production code.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::VMReader;
///
/// let mock = brainfoamkit_lib::MockReader {
///     data: std::io::Cursor::new("A".as_bytes().to_vec()),
/// };
///
/// assert_eq!(mock.read().unwrap(), 65);
/// ```
///
/// # See Also
///
/// * [`VMReader`](trait.VMReader.html)
/// * [`VMReaderType`](enum.VMReaderType.html)
/// * [Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html)
/// * [File](https://doc.rust-lang.org/std/fs/struct.File.html)
#[derive(Debug, Default)]
pub struct MockReader {
    pub data: Cursor<Vec<u8>>,
}

/// The implementation of the `VMReader` trait for the `MockReader` struct
impl VMReader for MockReader {
    /// Read a single byte from the mock reader
    ///
    /// This function reads a single byte from the mock reader and returns it as
    /// a `u8` for use by the `VirtualMachine`. The mock reader is
    /// implemented using a `Cursor<Vec<u8>>`, which is used to store the data
    /// that is read from the mock reader.
    ///
    /// # Errors
    ///
    /// This function will return an error if the byte read from the mock reader
    /// is not within the ASCII range.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::VMReader;
    ///
    /// let mock = brainfoamkit_lib::MockReader {
    ///     data: std::io::Cursor::new("A".as_bytes().to_vec()),
    /// };
    ///
    /// assert_eq!(mock.read().unwrap(), 65);
    /// assert_eq!(mock.get_type(), brainfoamkit_lib::VMReaderType::Mock);
    /// ```
    fn read(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        self.data.read_exact(&mut buffer)?;

        if buffer[0] <= 128 {
            Ok(buffer[0])
        } else {
            Err(anyhow!("Byte is not within the ASCII range"))
        }
    }

    fn get_vmreader_type(&self) -> VMReaderType {
        VMReaderType::Mock
    }
}

/// The implementation of the `VMReader` trait for the `Stdin` struct
impl VMReader for Stdin {
    /// Read a single byte from STDIN
    ///
    /// This function reads a single byte from STDIN and returns it as a `u8`
    /// for use by the `VirtualMachine`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the byte read from STDIN is not
    /// within the ASCII range.
    fn read(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)?;

        if buffer[0] <= 128 {
            Ok(buffer[0])
        } else {
            Err(anyhow!("Byte is not within the ASCII range"))
        }
    }

    fn get_vmreader_type(&self) -> VMReaderType {
        VMReaderType::Stdin
    }
}

/// The implementation of the `VMReader` trait for the `File` struct
impl VMReader for File {
    /// Read a single byte from a file
    ///
    /// This function reads a single byte from a file and returns it as a `u8`
    /// for use by the `VirtualMachine`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the byte read from the file is not
    /// within the ASCII range.
    fn read(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)?;

        if buffer[0] <= 128 {
            Ok(buffer[0])
        } else {
            Err(anyhow!("Byte is not within the ASCII range"))
        }
    }

    fn get_vmreader_type(&self) -> VMReaderType {
        VMReaderType::File
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{
            Cursor,
            Write,
        },
    };

    use tempdir::TempDir;

    use super::*;

    struct DefaultReader;

    impl VMReader for DefaultReader {}

    #[test]
    fn test_default_trait() {
        let mut reader = DefaultReader;
        let read_value = reader.read().unwrap();
        assert_eq!(read_value, 0);
        assert_eq!(reader.get_vmreader_type(), VMReaderType::Unknown);
    }

    #[test]
    fn test_read_from_stdin() {
        let mut stdin = Cursor::new("A".as_bytes());
        let mut buffer = [0u8; 1];
        stdin.read_exact(&mut buffer).unwrap();
        assert_eq!(buffer[0], 65);
    }

    #[test]
    fn test_read_from_file() {
        let temp_dir = TempDir::new("test").unwrap();
        let file_path = temp_dir.path().join("test.bf");
        let mut file = File::create(&file_path).unwrap();
        file.write_all("A".as_bytes()).unwrap();

        let mut file = File::open(file_path).unwrap();
        let read_value = VMReader::read(&mut file).unwrap();
        assert_eq!(read_value, 65);
    }

    #[test]
    fn test_read_from_mock() {
        let mut mock = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let read_value = mock.read().unwrap();
        assert_eq!(read_value, 65);
    }

    #[test]
    fn test_get_type() {
        let stdin = std::io::stdin();
        let temp_dir = TempDir::new("test").unwrap();
        let file_path = temp_dir.path().join("test.bf");
        let file = File::create(file_path).unwrap();
        let mock = MockReader {
            data: Cursor::new("A".as_bytes().to_vec()),
        };
        let default = DefaultReader;

        assert_eq!(stdin.get_vmreader_type(), VMReaderType::Stdin);
        assert_eq!(file.get_vmreader_type(), VMReaderType::File);
        assert_eq!(mock.get_vmreader_type(), VMReaderType::Mock);
        assert_eq!(default.get_vmreader_type(), VMReaderType::Unknown);
    }
}
