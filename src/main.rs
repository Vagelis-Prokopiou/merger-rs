use anyhow::{Context, Result};
use clap::Parser;
use csv;
use itertools::Itertools; // Add this import
use std::collections::HashSet;
use std::fs::File;
use linked_hash_map::LinkedHashMap;
use std::io::Write;

#[derive(Parser)]
struct Args {
    /// Input CSV file path
    input: String,

    /// Output CSV file path
    output: String,

    /// ID column name
    #[clap(short, long)]
    id_column: String,

    /// Columns to merge (comma-separated list)
    #[clap(long)]
    columns: String,

    /// Delimiter for concatenated values
    #[clap(long, default_value = ";")]
    concat_delimiter: String,

    /// Delimiter for output headers
    #[clap(long, default_value = ",")]
    header_delimiter: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Split the columns argument into a vector of column names
    let columns: Vec<String> = args.columns.split(',')
        .map(|s| s.to_string())
        .collect();

    // Open the input file for reading
    let input_file = File::open(&args.input)
        .with_context(|| format!("Failed to open input file '{}'", args.input))?;
    let mut rdr = csv::Reader::from_reader(input_file);

    // Retrieve the headers
    let headers = rdr.headers()?.clone();

    // Get the index of the ID column
    let id_column_index = headers.iter().position(|h| h == &args.id_column)
        .with_context(|| format!("ID column '{}' not found in headers", args.id_column))?;

    // Get the indices of the specified columns
    let column_indices: Vec<usize> = columns.iter()
        .map(|col| headers.iter().position(|h| h == col)
            .with_context(|| format!("Column '{}' not found in headers", col)))
        .collect::<Result<Vec<_>>>()?;

    // Create a LinkedHashMap to store the merged names for each id
    let mut data: LinkedHashMap<String, Vec<HashSet<String>>> = LinkedHashMap::new();

    // Read the input file line by line
    for result in rdr.records() {
        let record = result?;

        let id = record[id_column_index].to_string();

        let entry = data.entry(id).or_insert_with(|| vec![HashSet::new(); column_indices.len()]);
        for (set, &ci) in entry.iter_mut().zip(&column_indices) {
            let value = record[ci].to_string();
            set.insert(value);
        }
    }

    // Open the output file for writing
    let mut output_file = File::create(&args.output)
        .with_context(|| format!("Failed to create output file '{}'", args.output))?;

    // Write the header
    writeln!(output_file, "{}", headers.iter().join(&args.header_delimiter))?;

    // Write the merged data to the output file in the order they appeared
    for (id, sets) in data {
        let mut row = vec![id];
        for set in sets {
            row.push(set.into_iter().collect::<Vec<_>>().join(&args.concat_delimiter));
        }
        writeln!(output_file, "{}", row.join(&args.header_delimiter))?;
    }

    Ok(())
}
