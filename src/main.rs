mod types;
use std::{error::Error, path::Path, process};

use csv::StringRecord;
use serde::Deserialize;

/// Represents CSV table data
#[derive(Debug)]
pub struct CsvTable {
    pub columns: StringRecord,
    pub rows: Vec<StringRecord>,
}

impl CsvTable {
    /// Attempt to deserialize the table into another data structure
    fn deserialize<'de, E: Deserialize<'de>>(&'de self) -> Result<Vec<E>, Box<dyn Error>> {
        let mut results: Vec<E> = vec![];
        for row in &self.rows {
            let o: E = row.deserialize(Some(&self.columns))?;
            results.push(o)
        }
        Ok(results)
    }
}

/// This function will read a CSV file with multiple tables assuming the following...
/// 1. There is no leading whitespace for the first table
/// 2. Each table in the CSV has column headers
/// 3. Each table in the CSV is separated with 3 empty lines
pub fn read_csv_to_tables(csv_path: &Path) -> Result<Vec<CsvTable>, Box<dyn Error>> {
    let file = std::fs::File::open(csv_path)?;
    let reader = std::io::BufReader::new(file);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .skip_blank_lines(false)
        .from_reader(reader);

    let mut tables = vec![];

    let mut current_table = CsvTable {
        columns: StringRecord::new(),
        rows: vec![],
    };

    let mut empty_count = 0;
    let mut table_cursor = 0;

    for result in rdr.records() {
        let row = result?;
        if table_cursor == 0 {
            current_table.columns = row;
            table_cursor += 1;
        } else {
            let mut row_is_empty = false;

            if row.len() == 1 && row.get(0) == Some("") {
                row_is_empty = true;
                empty_count += 1;
            } else {
                empty_count = 0;
            }

            if empty_count == 3 {
                // Start new table
                tables.push(current_table);

                current_table = CsvTable {
                    columns: StringRecord::new(),
                    rows: vec![],
                };

                table_cursor = 0;
            } else {
                table_cursor += 1;
            }

            if !row_is_empty {
                current_table.rows.push(row);
            }
        }
    }

    if table_cursor > 0 {
        tables.push(current_table);
    }

    Ok(tables)
}

fn main() {
    match read_csv_to_tables(Path::new("documents/example.csv")) {
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
        Ok(tables) => {
            if let Some(accounts_table) = tables.first() {
                let data: Vec<crate::types::Account> = accounts_table.deserialize().unwrap();
                println!("{:#?}", data);
            }

            if let Some(transaction_table) = tables.get(1) {
                let data: Vec<crate::types::Transaction> = transaction_table.deserialize().unwrap();
                println!("{:#?}", data);
            }
        }
    }
}
