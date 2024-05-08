use std::fmt::Display;

use chrono::Datelike;
use regex::Regex;
use serde::{Deserialize, Serialize};

use self::errors::DateError;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq)]
pub struct Date {
    pub month: u8,
    pub day: u8,
    pub year: Option<u16>,
}

impl Date {
    pub fn new(month: u8, day: u8, year: Option<u16>) -> Self {
        Self { month, day, year }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.year {
            Some(y) => write!(f, "{:02}/{:02}/{}", self.month, self.day, y),
            None => write!(f, "{:02}/{:02}", self.month, self.day),
        }
    }
}

// sorting impls

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        let curr_year = chrono::Utc::now().year() as u16;
        let years_eq = match (self.year, other.year) {
            (None, None) => true,
            (Some(y1), Some(y2)) => y1 == y2,
            (None, Some(y)) => y == curr_year,
            (Some(y), None) => y == curr_year,
        };
        self.month == other.month && self.day == other.day && years_eq
    }
}
impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // goes through large to fine grained
        // continues to next check if equal, otherwise returns ordering early
        let curr_year = chrono::Utc::now().year() as u16;
        match (self.year, other.year) {
            (None, None) => {}
            (None, Some(y)) => match curr_year.cmp(&y) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            },
            (Some(y), None) => match y.cmp(&curr_year) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            },
            (Some(y1), Some(y2)) => match y1.cmp(&y2) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            },
        }
        match self.month.cmp(&other.month) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.day.cmp(&other.day)
    }
}

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DateError {
        /// Attempting to create a list with a used name
        #[error("Could not parse Date from: {given:?}. {error:?}")]
        DateParseError { given: String, error: String },
    }
}

/// Helper which parses a date from a string
pub fn parse_date(s: &str) -> Result<Date, DateError> {
    // regex which matches dates of the form MM/DD, MM/DD/YY, MM/DD/YYYY
    let regex = Regex::new(r"^(\d{1,2})(?:\/|-)(\d{1,2})(?:(?:\/|-)(\d{4}|\d{2}))?$").unwrap();

    // check if valid format
    if !regex.is_match(s) {
        return Err(DateError::DateParseError { given: s.to_string(), error: "Invalid date format. Accepted formats: MM/DD, MM/DD/YY, MM/DD/YYYY  (note single digit days and months are also accepted)".to_string() });
    }

    // extract captures
    let caps = regex.captures(s).unwrap();
    let month = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
    let day = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
    let year = caps.get(3).map(|m| {
        let y = m.as_str().parse::<u16>().unwrap();
        match y {
            0..=99 => 2000 + y, // assume 2000s if only 2-digit year
            _ => y,
        }
    });

    // validate month-day combos
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => {
            if day > 31 {
                return Err(DateError::DateParseError {
                    given: s.to_string(),
                    error: "Invalid date. This month has at most 31 days".to_string(),
                });
            }
        }
        4 | 6 | 9 | 11 => {
            if day > 30 {
                return Err(DateError::DateParseError {
                    given: s.to_string(),
                    error: "Invalid date. This month has at most 30 days".to_string(),
                });
            }
        }
        2 => {
            if year.is_none() {
                if day > 28 {
                    return Err(DateError::DateParseError {
                        given: s.to_string(),
                        error: "Invalid date. February has at most 28 days".to_string(),
                    });
                }
            } else if day > 29 {
                return Err(DateError::DateParseError {
                    given: s.to_string(),
                    error: "Invalid date. February has at most 29 days".to_string(),
                });
            }
        }
        _ => {
            return Err(DateError::DateParseError {
                given: s.to_string(),
                error: "Invalid date. Month must be between 1 and 12".to_string(),
            })
        }
    }

    Ok(Date::new(month, day, year))
}
