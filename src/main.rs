use crate::converter::TraceReader;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod converter;

fn main() -> Result<(), Box<dyn Error>> {
    // Open input file
    let input = File::open("traces/Recording_1.csv")?;

    // Read file to samples
    let samples = TraceReader::read(input)?;

    // Open output file
    let output = File::create("traces/Recording_1.json")?;

    // Write samples as JSON to output
    let mut bw = BufWriter::new(output);

    write!(
        &mut bw,
        "[{}]",
        samples
            .into_iter()
            .map(|s| s.to_json())
            .collect::<Vec<_>>()
            .join(",")
    )?;
    bw.flush()?;

    Ok(())
}
