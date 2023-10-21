use lazy_static::lazy_static;
use regex::Regex;
use std::convert::From;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::fmt;

#[derive(Debug)]
pub struct TemperatureLine {
    pub time_step: u64,
    pub readings: Vec<f64>,
}

#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IOError(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::IOError(err) => {
                write!(f, "{:?}", err)
            }
        }
    }
}

//------------------------------------------------------------------------------
const TIME_STEP_SIZE: u64 = 30;

lazy_static! {
    static ref LINE_DELIM_RE: Regex = Regex::new(r"[^0-9]*\s+|[^0-9]*$").unwrap();
}

pub fn read_temperature_file(filename: &str) -> Result<Vec<TemperatureLine>, ParseError> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let readings = read_temperatures(reader);
    readings
}

pub fn read_temperatures<R>(reader: R) -> Result<Vec<TemperatureLine>, ParseError>
where
    R: BufRead,
{
    let mut readings: Vec<TemperatureLine> = Vec::new();

    for (idx, wrapped_line) in reader.lines().enumerate() {
        match wrapped_line {
            Ok(line) => {
                let time = (idx as u64) * TIME_STEP_SIZE;

                let core_temps: Vec<f64> = LINE_DELIM_RE
                    .split(line.trim())
                    .into_iter()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                let new_reading = TemperatureLine {
                    time_step: time,
                    readings: core_temps,
                };

                readings.push(new_reading);
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    Ok(readings)
}
