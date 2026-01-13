// SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use brainfoamkit_lib::{
    AsciiTable,
    Byte,
};
use prettytable::{
    Table,
    format::{
        self,
    },
    row,
};

/// Displays an ASCII reference table showing all standard ASCII characters
/// (0-127) with their decimal, binary, hexadecimal representations and
/// human-readable forms. This is useful for quick reference when working with
/// ASCII values in Brainfuck programs.
fn main() {
    let mut table = Table::new();
    let ascii = AsciiTable::new();

    // Create table headers for different number representations
    table.set_titles(row![bc => "Binary", "Hexadecimal", "Character Code", "Character Value", "Character Description"]);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // Populate the table with ASCII values 0-127 and their representations
    for num in 0..128 {
        let byte = Byte::from(num);
        let char = ascii.get(byte);
        match char {
            None => panic!(),
            Some(val) => {
                table.add_row(row![c=>
                    format!("{display}", display = val.binary_value()),
                    format!("{display}", display = val.hexadecimal_value()),
                    format!("{display}", display = val.character_code()),
                    format!("{display}", display = val.character_value()),
                    format!("{display}", display = val.character_description())
                ]);
            }
        }
    }

    table.printstd();
}
