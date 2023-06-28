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
    ///         "signal2": 3.14
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
    use std::io::Cursor;

    #[test]
    fn trace_reader_read() {
        let csv = "timeStamp;ParkFunc;AngleParkFunc\n\
        3.83;65533.0;4093.0\n\
        4.76;400.0; "
            .to_string();
        let c = Cursor::new(csv);

        let exp = Ok(vec![
            Sample {
                timestamp: 3.83,
                signal_values: HashMap::from([
                    ("ParkFunc".to_string(), 65533.0),
                    ("AngleParkFunc".to_string(), 4093.0),
                ]),
            },
            Sample {
                timestamp: 4.76,
                signal_values: HashMap::from([("ParkFunc".to_string(), 400.0)]),
            },
        ]);

        assert_eq!(TraceReader::read(c), exp);
    }

    #[test]
    fn sample_to_json() {
        let s = Sample {
            timestamp: 1.112,
            signal_values: HashMap::from([
                ("signal1".to_string(), 42.42),
                ("signal2".to_string(), 3.14),
            ]),
        };

        let res = s.to_json();

        // Since HashMap is not ordered signals are in arbitrary order
        assert!(
            res == "{\"timestamp\": 1.112, \"signal_values\": {\"signal2\": 3.14, \"signal1\": 42.42}}"
                || res == "{\"timestamp\": 1.112, \"signal_values\": {\"signal1\": 42.42, \"signal2\": 3.14}}"
        )
    }
}
