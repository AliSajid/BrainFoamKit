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

use std::collections::HashMap;

use crate::{
    AsciiChar,
    Byte,
};

/// Represents a table of ASCII characters.
///
/// The table is implemented as a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
/// of [`Byte`](struct.Byte.html) values to [`AsciiChar`](struct.AsciiChar.html)
///
/// It maps the valid ASCII [`Byte`](struct.Byte.html) values to their
/// corresponding [`AsciiChar`](struct.AsciiChar.html) values.
///
/// # Examples
///
/// ```
/// use brainfoamkit_lib::{
///     AsciiTable,
///     Byte,
/// };
///
/// let ascii_table = AsciiTable::new();
///
/// assert_eq!(
///     ascii_table.get(Byte::from_u8(0)).unwrap().character_code(),
///     "CNUL"
/// );
/// assert_eq!(
///     ascii_table.get(Byte::from_u8(1)).unwrap().character_code(),
///     "CSOH"
/// );
/// assert_eq!(
///     ascii_table.get(Byte::from_u8(2)).unwrap().character_code(),
///     "CSTX"
/// );
/// ```
///
/// # References
///
/// * [ASCII](https://en.wikipedia.org/wiki/ASCII)
/// * [ASCII Table](https://www.asciitable.com/)
/// * [ASCII Table and Description](https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html)
pub struct AsciiTable {
    table: HashMap<Byte, AsciiChar>,
}

impl AsciiTable {
    /// Create a new [`AsciiTable`](struct.AsciiTable.html) instance.
    ///
    /// This will create a new [`AsciiTable`](struct.AsciiTable.html) instance
    /// with all the valid ASCII characters pre-populated.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiTable,
    ///     Byte,
    /// };
    ///
    /// let ascii_table = AsciiTable::new();
    ///
    /// assert_eq!(
    ///     ascii_table.get(Byte::from_u8(0)).unwrap().character_code(),
    ///     "CNUL"
    /// );
    /// ```
    ///
    /// # References
    ///
    /// * [ASCII](https://en.wikipedia.org/wiki/ASCII)
    /// * [ASCII Table](https://www.asciitable.com/)
    /// * [ASCII Table and Description](https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html)
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn new() -> Self {
        let mut table = HashMap::new();

        {
            table.insert(
                Byte::from_u8(0),
                AsciiChar::new(Byte::from_u8(0), "CNUL", "Null character", "\\000"),
            );

            table.insert(
                Byte::from_u8(1),
                AsciiChar::new(Byte::from_u8(1), "CSOH", "Start of heading", "\\001"),
            );
            table.insert(
                Byte::from_u8(2),
                AsciiChar::new(Byte::from_u8(2), "CSTX", "Start of text", "\\002"),
            );
            table.insert(
                Byte::from_u8(3),
                AsciiChar::new(Byte::from_u8(3), "CETX", "End of text", "\\003"),
            );
            table.insert(
                Byte::from_u8(4),
                AsciiChar::new(Byte::from_u8(4), "CEOT", "End of transmission", "\\004"),
            );
            table.insert(
                Byte::from_u8(5),
                AsciiChar::new(Byte::from_u8(5), "CENQ", "Enquiry", "\\005"),
            );
            table.insert(
                Byte::from_u8(6),
                AsciiChar::new(Byte::from_u8(6), "CACK", "Acknowledge", "\\006"),
            );
            table.insert(
                Byte::from_u8(7),
                AsciiChar::new(Byte::from_u8(7), "CBEL", "Bell", "\\007"),
            );
            table.insert(
                Byte::from_u8(8),
                AsciiChar::new(Byte::from_u8(8), "CBS", "Backspace", "\\008"),
            );
            table.insert(
                Byte::from_u8(9),
                AsciiChar::new(Byte::from_u8(9), "CTAB", "Horizontal tab", "\\009"),
            );
            table.insert(
                Byte::from_u8(10),
                AsciiChar::new(Byte::from_u8(10), "CLF", "Line feed", "\\010"),
            );
            table.insert(
                Byte::from_u8(11),
                AsciiChar::new(Byte::from_u8(11), "CVT", "Vertical tab", "\\011"),
            );
            table.insert(
                Byte::from_u8(12),
                AsciiChar::new(Byte::from_u8(12), "CFF", "Form feed", "\\012"),
            );
            table.insert(
                Byte::from_u8(13),
                AsciiChar::new(Byte::from_u8(13), "CCR", "Carriage return", "\\013"),
            );
            table.insert(
                Byte::from_u8(14),
                AsciiChar::new(Byte::from_u8(14), "CSO", "Shift out", "\\014"),
            );
            table.insert(
                Byte::from_u8(15),
                AsciiChar::new(Byte::from_u8(15), "CSI", "Shift in", "\\015"),
            );
            table.insert(
                Byte::from_u8(16),
                AsciiChar::new(Byte::from_u8(16), "CDLE", "Data link escape", "\\016"),
            );
            table.insert(
                Byte::from_u8(17),
                AsciiChar::new(Byte::from_u8(17), "CDC1", "Device control 1", "\\017"),
            );
            table.insert(
                Byte::from_u8(18),
                AsciiChar::new(Byte::from_u8(18), "CDC2", "Device control 2", "\\018"),
            );
            table.insert(
                Byte::from_u8(19),
                AsciiChar::new(Byte::from_u8(19), "CDC3", "Device control 3", "\\019"),
            );
            table.insert(
                Byte::from_u8(20),
                AsciiChar::new(Byte::from_u8(20), "CDC4", "Device control 4", "\\020"),
            );
            table.insert(
                Byte::from_u8(21),
                AsciiChar::new(Byte::from_u8(21), "CNAK", "Negative acknowledge", "\\021"),
            );
            table.insert(
                Byte::from_u8(22),
                AsciiChar::new(Byte::from_u8(22), "CSYN", "Synchronous idle", "\\022"),
            );
            table.insert(
                Byte::from_u8(23),
                AsciiChar::new(
                    Byte::from_u8(23),
                    "CETB",
                    "End of transmission block",
                    "\\023",
                ),
            );
            table.insert(
                Byte::from_u8(24),
                AsciiChar::new(Byte::from_u8(24), "CCAN", "Cancel", "\\024"),
            );
            table.insert(
                Byte::from_u8(25),
                AsciiChar::new(Byte::from_u8(25), "CEM", "End of medium", "\\025"),
            );
            table.insert(
                Byte::from_u8(26),
                AsciiChar::new(Byte::from_u8(26), "CSUB", "Substitute", "\\026"),
            );
            table.insert(
                Byte::from_u8(27),
                AsciiChar::new(Byte::from_u8(27), "CESC", "Escape", "\\027"),
            );
            table.insert(
                Byte::from_u8(28),
                AsciiChar::new(Byte::from_u8(28), "CFS", "File separator", "\\028"),
            );
            table.insert(
                Byte::from_u8(29),
                AsciiChar::new(Byte::from_u8(29), "CGS", "Group separator", "\\029"),
            );
            table.insert(
                Byte::from_u8(30),
                AsciiChar::new(Byte::from_u8(30), "CRS", "Record separator", "\\030"),
            );
            table.insert(
                Byte::from_u8(31),
                AsciiChar::new(Byte::from_u8(31), "CUS", "Unit separator", "\\031"),
            );
            table.insert(
                Byte::from_u8(32),
                AsciiChar::new(Byte::from_u8(32), "WSP", "Space", " "),
            );
            table.insert(
                Byte::from_u8(33),
                AsciiChar::new(Byte::from_u8(33), "SBANG", "Exclamation mark", "!"),
            );
            table.insert(
                Byte::from_u8(34),
                AsciiChar::new(Byte::from_u8(34), "SDBLQ", "Double quote", "\""),
            );
            table.insert(
                Byte::from_u8(35),
                AsciiChar::new(Byte::from_u8(35), "SHASH", "Hash", "#"),
            );
            table.insert(
                Byte::from_u8(36),
                AsciiChar::new(Byte::from_u8(36), "SDOLL", "Dollar sign", "$"),
            );
            table.insert(
                Byte::from_u8(37),
                AsciiChar::new(Byte::from_u8(37), "SPERC", "Percent", "%"),
            );
            table.insert(
                Byte::from_u8(38),
                AsciiChar::new(Byte::from_u8(38), "SAMP", "Ampersand", "&"),
            );
            table.insert(
                Byte::from_u8(39),
                AsciiChar::new(Byte::from_u8(39), "SSQT", "Single quote", "'"),
            );
            table.insert(
                Byte::from_u8(40),
                AsciiChar::new(Byte::from_u8(40), "SOPAR", "Open parenthesis", "("),
            );
            table.insert(
                Byte::from_u8(41),
                AsciiChar::new(Byte::from_u8(41), "SCPAR", "Close parenthesis", ")"),
            );
            table.insert(
                Byte::from_u8(42),
                AsciiChar::new(Byte::from_u8(42), "SSTAR", "Asterisk", "*"),
            );
            table.insert(
                Byte::from_u8(43),
                AsciiChar::new(Byte::from_u8(43), "SPLUS", "Plus", "+"),
            );
            table.insert(
                Byte::from_u8(44),
                AsciiChar::new(Byte::from_u8(44), "SCOM", "Comma", ","),
            );
            table.insert(
                Byte::from_u8(45),
                AsciiChar::new(Byte::from_u8(45), "SDASH", "Dash", "-"),
            );
            table.insert(
                Byte::from_u8(46),
                AsciiChar::new(Byte::from_u8(46), "SDOT", "Period", "."),
            );
            table.insert(
                Byte::from_u8(47),
                AsciiChar::new(Byte::from_u8(47), "SSLASH", "Slash", "/"),
            );
            table.insert(
                Byte::from_u8(48),
                AsciiChar::new(Byte::from_u8(48), "DIG0", "Zero", "0"),
            );
            table.insert(
                Byte::from_u8(49),
                AsciiChar::new(Byte::from_u8(49), "DIG1", "One", "1"),
            );
            table.insert(
                Byte::from_u8(50),
                AsciiChar::new(Byte::from_u8(50), "DIG2", "Two", "2"),
            );
            table.insert(
                Byte::from_u8(51),
                AsciiChar::new(Byte::from_u8(51), "DIG3", "Three", "3"),
            );
            table.insert(
                Byte::from_u8(52),
                AsciiChar::new(Byte::from_u8(52), "DIG4", "Four", "4"),
            );
            table.insert(
                Byte::from_u8(53),
                AsciiChar::new(Byte::from_u8(53), "DIG5", "Five", "5"),
            );
            table.insert(
                Byte::from_u8(54),
                AsciiChar::new(Byte::from_u8(54), "DIG6", "Six", "6"),
            );
            table.insert(
                Byte::from_u8(55),
                AsciiChar::new(Byte::from_u8(55), "DIG7", "Seven", "7"),
            );
            table.insert(
                Byte::from_u8(56),
                AsciiChar::new(Byte::from_u8(56), "DIG8", "Eight", "8"),
            );
            table.insert(
                Byte::from_u8(57),
                AsciiChar::new(Byte::from_u8(57), "DIG9", "Nine", "9"),
            );
            table.insert(
                Byte::from_u8(58),
                AsciiChar::new(Byte::from_u8(58), "SCOL", "Colon", ":"),
            );
            table.insert(
                Byte::from_u8(59),
                AsciiChar::new(Byte::from_u8(59), "SSCL", "Semicolon", ";"),
            );
            table.insert(
                Byte::from_u8(60),
                AsciiChar::new(Byte::from_u8(60), "SLT", "Less than", "<"),
            );
            table.insert(
                Byte::from_u8(61),
                AsciiChar::new(Byte::from_u8(61), "SEQ", "Equals", "="),
            );
            table.insert(
                Byte::from_u8(62),
                AsciiChar::new(Byte::from_u8(62), "SGT", "Greater than", ">"),
            );
            table.insert(
                Byte::from_u8(63),
                AsciiChar::new(Byte::from_u8(63), "SQUES", "Question mark", "?"),
            );
            table.insert(
                Byte::from_u8(64),
                AsciiChar::new(Byte::from_u8(64), "SAT", "At sign", "@"),
            );
            table.insert(
                Byte::from_u8(65),
                AsciiChar::new(Byte::from_u8(65), "UCLA", "Uppercase Letter A", "A"),
            );
            table.insert(
                Byte::from_u8(66),
                AsciiChar::new(Byte::from_u8(66), "UCLB", "Uppercase Letter B", "B"),
            );
            table.insert(
                Byte::from_u8(67),
                AsciiChar::new(Byte::from_u8(67), "UCLC", "Uppercase Letter C", "C"),
            );
            table.insert(
                Byte::from_u8(68),
                AsciiChar::new(Byte::from_u8(68), "UCLD", "Uppercase Letter D", "D"),
            );
            table.insert(
                Byte::from_u8(69),
                AsciiChar::new(Byte::from_u8(69), "UCLE", "Uppercase Letter E", "E"),
            );
            table.insert(
                Byte::from_u8(70),
                AsciiChar::new(Byte::from_u8(70), "UCLF", "Uppercase Letter F", "F"),
            );
            table.insert(
                Byte::from_u8(71),
                AsciiChar::new(Byte::from_u8(71), "UCLG", "Uppercase Letter G", "G"),
            );
            table.insert(
                Byte::from_u8(72),
                AsciiChar::new(Byte::from_u8(72), "UCLH", "Uppercase Letter H", "H"),
            );
            table.insert(
                Byte::from_u8(73),
                AsciiChar::new(Byte::from_u8(73), "UCLI", "Uppercase Letter I", "I"),
            );
            table.insert(
                Byte::from_u8(74),
                AsciiChar::new(Byte::from_u8(74), "UCLJ", "Uppercase Letter J", "J"),
            );
            table.insert(
                Byte::from_u8(75),
                AsciiChar::new(Byte::from_u8(75), "UCLK", "Uppercase Letter K", "K"),
            );
            table.insert(
                Byte::from_u8(76),
                AsciiChar::new(Byte::from_u8(76), "UCLL", "Uppercase Letter L", "L"),
            );
            table.insert(
                Byte::from_u8(77),
                AsciiChar::new(Byte::from_u8(77), "UCLM", "Uppercase Letter M", "M"),
            );
            table.insert(
                Byte::from_u8(78),
                AsciiChar::new(Byte::from_u8(78), "UCLN", "Uppercase Letter N", "N"),
            );
            table.insert(
                Byte::from_u8(79),
                AsciiChar::new(Byte::from_u8(79), "UCLO", "Uppercase Letter O", "O"),
            );
            table.insert(
                Byte::from_u8(80),
                AsciiChar::new(Byte::from_u8(80), "UCLP", "Uppercase Letter P", "P"),
            );
            table.insert(
                Byte::from_u8(81),
                AsciiChar::new(Byte::from_u8(81), "UCLQ", "Uppercase Letter Q", "Q"),
            );
            table.insert(
                Byte::from_u8(82),
                AsciiChar::new(Byte::from_u8(82), "UCLR", "Uppercase Letter R", "R"),
            );
            table.insert(
                Byte::from_u8(83),
                AsciiChar::new(Byte::from_u8(83), "UCLS", "Uppercase Letter S", "S"),
            );
            table.insert(
                Byte::from_u8(84),
                AsciiChar::new(Byte::from_u8(84), "UCLT", "Uppercase Letter T", "T"),
            );
            table.insert(
                Byte::from_u8(85),
                AsciiChar::new(Byte::from_u8(85), "UCLU", "Uppercase Letter U", "U"),
            );
            table.insert(
                Byte::from_u8(86),
                AsciiChar::new(Byte::from_u8(86), "UCLV", "Uppercase Letter V", "V"),
            );
            table.insert(
                Byte::from_u8(87),
                AsciiChar::new(Byte::from_u8(87), "UCLW", "Uppercase Letter W", "W"),
            );
            table.insert(
                Byte::from_u8(88),
                AsciiChar::new(Byte::from_u8(88), "UCLX", "Uppercase Letter X", "X"),
            );
            table.insert(
                Byte::from_u8(89),
                AsciiChar::new(Byte::from_u8(89), "UCLY", "Uppercase Letter Y", "Y"),
            );
            table.insert(
                Byte::from_u8(90),
                AsciiChar::new(Byte::from_u8(90), "UCLZ", "Uppercase Letter Z", "Z"),
            );
            table.insert(
                Byte::from_u8(91),
                AsciiChar::new(Byte::from_u8(91), "SOSB", "Open square bracket", "["),
            );
            table.insert(
                Byte::from_u8(92),
                AsciiChar::new(Byte::from_u8(92), "SBKS", "Backslash", "\\"),
            );
            table.insert(
                Byte::from_u8(93),
                AsciiChar::new(Byte::from_u8(93), "SCSB", "Close square bracket", "]"),
            );
            table.insert(
                Byte::from_u8(94),
                AsciiChar::new(Byte::from_u8(94), "SCAR", "Caret", "^"),
            );
            table.insert(
                Byte::from_u8(95),
                AsciiChar::new(Byte::from_u8(95), "SUSC", "Underscore", "_"),
            );
            table.insert(
                Byte::from_u8(96),
                AsciiChar::new(Byte::from_u8(96), "SBTK", "Backtick", "`"),
            );
            table.insert(
                Byte::from_u8(97),
                AsciiChar::new(Byte::from_u8(97), "LCLA", "Lowercase Letter a", "a"),
            );
            table.insert(
                Byte::from_u8(98),
                AsciiChar::new(Byte::from_u8(98), "LCLB", "Lowercase Letter b", "b"),
            );
            table.insert(
                Byte::from_u8(99),
                AsciiChar::new(Byte::from_u8(99), "LCLC", "Lowercase Letter c", "c"),
            );
            table.insert(
                Byte::from_u8(100),
                AsciiChar::new(Byte::from_u8(100), "LCLD", "Lowercase Letter d", "d"),
            );
            table.insert(
                Byte::from_u8(101),
                AsciiChar::new(Byte::from_u8(101), "LCLE", "Lowercase Letter e", "e"),
            );
            table.insert(
                Byte::from_u8(102),
                AsciiChar::new(Byte::from_u8(102), "LCLF", "Lowercase Letter f", "f"),
            );
            table.insert(
                Byte::from_u8(103),
                AsciiChar::new(Byte::from_u8(103), "LCLG", "Lowercase Letter g", "g"),
            );
            table.insert(
                Byte::from_u8(104),
                AsciiChar::new(Byte::from_u8(104), "LCLH", "Lowercase Letter h", "h"),
            );
            table.insert(
                Byte::from_u8(105),
                AsciiChar::new(Byte::from_u8(105), "LCLI", "Lowercase Letter i", "i"),
            );
            table.insert(
                Byte::from_u8(106),
                AsciiChar::new(Byte::from_u8(106), "LCLJ", "Lowercase Letter j", "j"),
            );
            table.insert(
                Byte::from_u8(107),
                AsciiChar::new(Byte::from_u8(107), "LCLK", "Lowercase Letter k", "k"),
            );
            table.insert(
                Byte::from_u8(108),
                AsciiChar::new(Byte::from_u8(108), "LCLL", "Lowercase Letter l", "l"),
            );
            table.insert(
                Byte::from_u8(109),
                AsciiChar::new(Byte::from_u8(109), "LCLM", "Lowercase Letter m", "m"),
            );
            table.insert(
                Byte::from_u8(110),
                AsciiChar::new(Byte::from_u8(110), "LCLN", "Lowercase Letter n", "n"),
            );
            table.insert(
                Byte::from_u8(111),
                AsciiChar::new(Byte::from_u8(111), "LCLO", "Lowercase Letter o", "o"),
            );
            table.insert(
                Byte::from_u8(112),
                AsciiChar::new(Byte::from_u8(112), "LCLP", "Lowercase Letter p", "p"),
            );
            table.insert(
                Byte::from_u8(113),
                AsciiChar::new(Byte::from_u8(113), "LCLQ", "Lowercase Letter q", "q"),
            );
            table.insert(
                Byte::from_u8(114),
                AsciiChar::new(Byte::from_u8(114), "LCLR", "Lowercase Letter r", "r"),
            );
            table.insert(
                Byte::from_u8(115),
                AsciiChar::new(Byte::from_u8(115), "LCLS", "Lowercase Letter s", "s"),
            );
            table.insert(
                Byte::from_u8(116),
                AsciiChar::new(Byte::from_u8(116), "LCLT", "Lowercase Letter t", "t"),
            );
            table.insert(
                Byte::from_u8(117),
                AsciiChar::new(Byte::from_u8(117), "LCLU", "Lowercase Letter u", "u"),
            );
            table.insert(
                Byte::from_u8(118),
                AsciiChar::new(Byte::from_u8(118), "LCLV", "Lowercase Letter v", "v"),
            );
            table.insert(
                Byte::from_u8(119),
                AsciiChar::new(Byte::from_u8(119), "LCLW", "Lowercase Letter w", "w"),
            );
            table.insert(
                Byte::from_u8(120),
                AsciiChar::new(Byte::from_u8(120), "LCLX", "Lowercase Letter x", "x"),
            );
            table.insert(
                Byte::from_u8(121),
                AsciiChar::new(Byte::from_u8(121), "LCLY", "Lowercase Letter y", "y"),
            );
            table.insert(
                Byte::from_u8(122),
                AsciiChar::new(Byte::from_u8(122), "LCLZ", "Lowercase Letter z", "z"),
            );
            table.insert(
                Byte::from_u8(123),
                AsciiChar::new(Byte::from_u8(123), "SOCB", "Open curly brace", "{"),
            );
            table.insert(
                Byte::from_u8(124),
                AsciiChar::new(Byte::from_u8(124), "SVBAR", "Vertical bar", "|"),
            );
            table.insert(
                Byte::from_u8(125),
                AsciiChar::new(Byte::from_u8(125), "SCCB", "Close curly brace", "}"),
            );
            table.insert(
                Byte::from_u8(126),
                AsciiChar::new(Byte::from_u8(126), "STLD", "Tilde", "~"),
            );
            table.insert(
                Byte::from_u8(127),
                AsciiChar::new(Byte::from_u8(127), "CDEL", "Delete", "\\127"),
            );
        }

        Self { table }
    }

    /// Get an ASCII character from the table by its byte value.
    ///
    /// # Arguments
    ///
    /// * `byte` - The [`Byte`](struct.Byte.html) value of the ASCII character
    ///   to get.
    ///
    /// # Returns
    ///
    /// * `Some(&AsciiChar)` - The [`AsciiChar`](struct.AsciiChar.html) value
    ///  corresponding to the given [Byte](struct.Byte.html) value.
    /// * `None` - If the given [`Byte`](struct.Byte.html) value does not
    ///  correspond to an ASCII character.
    ///
    /// # Examples
    ///
    /// ```
    /// use brainfoamkit_lib::{
    ///     AsciiTable,
    ///     Byte,
    /// };
    ///
    /// let ascii_table = AsciiTable::new();
    ///
    /// assert_eq!(
    ///     ascii_table.get(Byte::from_u8(0)).unwrap().character_code(),
    ///     "CNUL"
    /// );
    /// assert_eq!(
    ///     ascii_table.get(Byte::from_u8(1)).unwrap().character_code(),
    ///     "CSOH"
    /// );
    /// assert_eq!(
    ///     ascii_table.get(Byte::from_u8(2)).unwrap().character_code(),
    ///     "CSTX"
    /// );
    /// ```
    #[must_use]
    pub fn get(&self, byte: Byte) -> Option<&AsciiChar> {
        self.table.get(&byte)
    }
}

impl Default for AsciiTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ascii_table_new() {
        let ascii_table = AsciiTable::new();

        assert_eq!(
            ascii_table.get(Byte::from_u8(0)).unwrap().character_code(),
            "CNUL"
        );
        assert_eq!(
            ascii_table.get(Byte::from_u8(1)).unwrap().character_code(),
            "CSOH"
        );
        assert_eq!(
            ascii_table.get(Byte::from_u8(2)).unwrap().character_code(),
            "CSTX"
        );
    }

    #[test]
    fn test_ascii_table_get() {
        let ascii_table = AsciiTable::new();

        // Test that the 'get' method returns the correct AsciiChar for a given Byte
        // value
        assert_eq!(
            ascii_table.get(Byte::from_u8(97)).unwrap().character_code(),
            "LCLA",
            "Character code for Byte value 97 should be 'LCLA'"
        );
        assert_eq!(
            ascii_table.get(Byte::from_u8(98)).unwrap().character_code(),
            "LCLB",
            "Character code for Byte value 98 should be 'LCLB'"
        );
        assert_eq!(
            ascii_table.get(Byte::from_u8(99)).unwrap().character_code(),
            "LCLC",
            "Character code for Byte value 99 should be 'LCLC'"
        );

        // Test that the 'get' method returns None for a Byte value that does not
        // correspond to an AsciiChar
        assert_eq!(
            ascii_table.get(Byte::from_u8(128)),
            None,
            "There should be no AsciiChar for Byte value 128"
        );
    }

    #[test]
    fn test_ascii_table_default() {
        let ascii_table = AsciiTable::default();

        // Test that the 'default' method returns an AsciiTable with the correct
        // AsciiChar values
        assert_eq!(
            ascii_table.get(Byte::from_u8(97)).unwrap().character_code(),
            "LCLA",
            "Character code for Byte value 97 should be 'LCLA'"
        );
        assert_eq!(
            ascii_table.get(Byte::from_u8(98)).unwrap().character_code(),
            "LCLB",
            "Character code for Byte value 98 should be 'LCLB'"
        );
        assert_eq!(
            ascii_table.get(Byte::from_u8(99)).unwrap().character_code(),
            "LCLC",
            "Character code for Byte value 99 should be 'LCLC'"
        );

        // Test that the 'get' method returns None for a Byte value that does not
        // correspond to an AsciiChar
        assert_eq!(
            ascii_table.get(Byte::from_u8(128)),
            None,
            "There should be no AsciiChar for Byte value 128"
        );
    }
}
