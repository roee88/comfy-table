use crate::cell::Cell;

pub struct Row {
    cells: Vec<Cell>,
}

impl Row {
    pub fn new() -> Row {
        Row { cells: Vec::new() }
    }

    pub fn from_cells(cells: Vec<Cell>) -> Row {
        Row { cells: cells }
    }

    /// Get the longest content width for all cells of this row
    pub fn max_content_widths(&self) -> Vec<usize> {
        // Iterate over all cells
        self.cells
            .iter()
            .map(|cell| {
                // Iterate over all content strings and return a vector of string widths.
                // Each entry represents the longest string width for a cell.
                cell.content
                    .iter()
                    .map(|string| string.len())
                    .max()
                    .unwrap_or(0)
            })
            .collect()
    }

    /// Return the amount of cells on this row.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}