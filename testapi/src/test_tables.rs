/* Test Tables - function */
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;


fn create_table(header: Vec<&str>, rows: Vec<Vec<&str>>, header_colors: Vec<Color>) -> Table {
    let mut table = Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
        .set_width(40);

    let header_cells: Vec<Cell> = header
        .iter()
        .zip(header_colors.into_iter())
        .map(|(text, color)| {
            Cell::new(*text)
                .set_alignment(CellAlignment::Center)
                .fg(color)
        })
        .collect();

    table.set_header(header_cells);

    for row in rows {
        let row_cells: Vec<Cell> = row
            .iter()
            .enumerate()
            .map(|(index, text)| {
                let color = match index {
                    0 => Color::Red,
                    1 => Color::Green,
                    2 => Color::Blue,
                    _ => Color::Reset, // Reset to default color for additional columns
                };
                Cell::new(*text)
                    .set_alignment(CellAlignment::Center)
                    .fg(color)
            })
            .collect();

        table.add_row(row_cells);
    }

    let column_indices: Vec<_> = table.column_iter().enumerate().map(|(index, _)| index).collect();

    for index in column_indices {
        table
            .column_mut(index)
            .expect("Invalid Value")
            .set_cell_alignment(CellAlignment::Center);
    }

    table
}

fn main() {
    let header = vec!["Grade", "Comment", "Added By"];
    let rows = vec![
        vec!["2", "Test", "Teacher Name"],
        vec!["3", "Example", "Another Teacher"],
    ];
    let row_colors = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
    ];

    let table = create_table(header, rows, row_colors);
    println!("{}", table);
}