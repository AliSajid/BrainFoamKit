// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use brainfoamkit_lib::{
    AsciiChar,
    AsciiTable,
    Byte,
};
use prettytable::{
    format::{
        self,
    },
    row,
    Table,
};

fn main() {
    let mut table = Table::new();
    let ascii = AsciiTable::new();

    table.set_titles(row![bc => "Byte", "Binary", "Hexadecimal", "String", "Representation"]);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    for num in 0..128 {
        let byte = Byte::from(num);
        let char = ascii
            .get(byte)
            .map_or("NA".to_owned(), AsciiChar::character_value);
        table.add_row(row![c=>
            format!("{num}", num = &byte),
            format!("{num:#010b}", num = u8::from(&byte)),
            format!("{num:#04X}", num = u8::from(&byte)),
            format!("{num}", num = byte.to_string()),
            format!("{char}", char = char)
        ]);
    }

    table.printstd();
}
