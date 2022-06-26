use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use std::io::{BufRead, BufReader, Read};
use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
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
        let mut json = format!("{{\"timestamp\": {}, \"signal_values\": {{", self.timestamp);
        for val in &self.signal_values {
            write!(&mut json, "\"{}\": {}, ", val.0, val.1).unwrap();
        }
        json.push_str("}}");
        json
    }
}

#[derive(Debug, PartialEq)]
pub struct TraceReaderError {
    message: String,
}

impl Display for TraceReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid trace input")
    }
}

impl Error for TraceReaderError {}

impl From<std::io::Error> for TraceReaderError {
    fn from(e: std::io::Error) -> Self {
        TraceReaderError {
            message: e.to_string(),
        }
    }
}

impl From<ParseFloatError> for TraceReaderError {
    fn from(e: ParseFloatError) -> Self {
        TraceReaderError {
            message: e.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct TraceReader {}

impl TraceReader {
    pub fn read<T: Read>(reader: T) -> Result<Vec<Sample>, TraceReaderError> {
        let reader = BufReader::new(reader);

        let mut lines = reader.lines().flatten();

        let first_line = lines.next().ok_or(TraceReaderError {
            message: "Line too short".to_string(),
        })?;
        let signal_names: Vec<_> = first_line.split(';').skip(1).map(|s| s.trim()).collect();

        let mut samples = vec![];

        for line in lines {
            let mut split_line = line.split(';');
            let ts = split_line
                .next()
                .ok_or(TraceReaderError {
                    message: "Line too short".to_string(),
                })?
                .parse()?;

            let mut values = HashMap::new();

            for (value, name) in split_line.zip(signal_names.iter()) {
                if let Ok(parsed_value) = value.parse() {
                    values.insert(name.to_string(), parsed_value);
                }
            }

            samples.push(Sample {
                timestamp: ts,
                signal_values: values,
            });
        }
        Ok(samples)
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

        assert!(
            res == "{\"timestamp\": 1.112, \"signal_values\": {\"signal2\": 3.14, \"signal1\": 42.42, }}" 
                || res == "{\"timestamp\": 1.112, \"signal_values\": {\"signal1\": 42.42, \"signal2\": 3.14, }}"
        )
    }
}
