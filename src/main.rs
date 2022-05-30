use crate::converter::TraceReader;
use std::error::Error;
use std::fs::File;

mod converter;

fn main() -> Result<(), Box<dyn Error>> {
    // Open input file
    let input = File::open("traces/Recording_1.csv")?;

    // Read file to samples
    let samples = TraceReader::read(input)?;

    // Open output file
    let output = File::create("traces/Recording_1.json")?;

    todo!("Write samples as JSON to output");

    Ok(())
}
