use clap::Parser;
use linked_hash_map::LinkedHashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashSet;

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
    #[clap(short, long)]
    columns: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Split the columns argument into a vector of column names
    let columns: Vec<String> = args.columns.split(',')
        .map(|s| s.to_string())
        .collect();

    // Specify the input and output file paths
    let input_path = Path::new(&args.input);
    let output_path = Path::new(&args.output);

    // Open the input file for reading
    let input_file = File::open(&input_path)?;
    let reader = BufReader::new(input_file);

    // Create a LinkedHashMap to store the merged names for each id
    let mut data: LinkedHashMap<String, Vec<HashSet<String>>> = LinkedHashMap::new();
    let mut headers: Vec<String> = Vec::new();
    let mut id_column_index: usize = 0;
    let mut column_indices: Vec<usize> = Vec::new();

    // Read the input file line by line
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if index == 0 {
            // Store header names
            headers = parts.iter().map(|&s| s.to_string()).collect();

            // Get the index of the ID column
            if let Some(pos) = headers.iter().position(|h| h == &args.id_column) {
                id_column_index = pos;
            } else {
                eprintln!("ID column '{}' not found in headers", args.id_column);
                return Ok(());
            }

            // Get the indices of the specified columns
            for column_name in &columns {
                if let Some(pos) = headers.iter().position(|h| h == column_name) {
                    column_indices.push(pos);
                } else {
                    eprintln!("Column '{}' not found in headers", column_name);
                    return Ok(());
                }
            }
        } else {
            let id = parts[id_column_index].to_string();
            let merged_values = column_indices.iter()
                .filter_map(|&i| parts.get(i))
                .map(|&s| s.to_string())
                .collect::<Vec<String>>();

            let entry = data.entry(id).or_insert_with(|| vec![HashSet::new(); column_indices.len()]);
            for (set, value) in entry.iter_mut().zip(merged_values) {
                set.insert(value);
            }
        }
    }

    // Open the output file for writing
    let mut output_file = File::create(&output_path)?;

    // Write the header (replicate from input file)
    writeln!(output_file, "{}", headers.join(","))?;

    // Write the merged data to the output file in the order they appeared
    for (id, sets) in data {
        let mut row = vec![id];
        for set in sets {
            row.push(set.into_iter().collect::<Vec<_>>().join(";"));
        }
        writeln!(output_file, "{}", row.join(","))?;
    }

    Ok(())
}
