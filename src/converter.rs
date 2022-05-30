use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Sample {
    timestamp: f64,
    signal_values: HashMap<String, f64>,
}

impl Sample {
    /// Convert Sample to JSON
    ///
    /// # Example
    /// {
    ///     "timestamp": 1.112,
    ///     "signal_values": {
    ///         "signal1": 42.42,
    ///         "signal2": 3.14,
    ///     }
    /// }
    pub fn to_json(&self) -> String {
        todo!("Implement me!")
    }
}

#[derive(Debug)]
pub struct TraceReaderError {}

impl Display for TraceReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid trace input")
    }
}

impl Error for TraceReaderError {}

#[derive(Debug)]
pub struct TraceReader {}

impl TraceReader {
    pub fn read<T: Read>(reader: T) -> Result<Vec<Sample>, TraceReaderError> {
        let reader = BufReader::new(reader);
        todo!("Implement me!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
