/* Test Tables - one table */

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_width(40)
        .set_header(vec![
            Cell::new("Grade").set_alignment(CellAlignment::Center).fg(Color::Red),
            Cell::new("Comment").set_alignment(CellAlignment::Center).fg(Color::Green),
            Cell::new("Added By").set_alignment(CellAlignment::Center).fg(Color::Blue),
        ])
        .add_row(vec![
            Cell::new("2").set_alignment(CellAlignment::Center).fg(Color::Red),
            Cell::new("Test").set_alignment(CellAlignment::Center).fg(Color::Green),
            Cell::new("Teacher Name").set_alignment(CellAlignment::Center).fg(Color::Blue),
        ]);

    let column_indices: Vec<_> = table.column_iter().enumerate().map(|(index, _)| index).collect();

    for index in column_indices {
        table.column_mut(index).expect("Invalid Value").set_cell_alignment(CellAlignment::Center);
    }
    println!("{table}");
}