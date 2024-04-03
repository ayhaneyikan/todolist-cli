use std::fmt::Display;

use regex::Regex;
use serde::{Serialize, Deserialize};

/// Helper which parses a date from a string
pub fn parse_date(s: &str) -> Result<Date, String> {
    // full date regex
    let regex = Regex::new(r"^(\d{1,2})(?:\/|-)(\d{1,2})(?:(?:\/|-)(\d{4}|\d{2}))?$").unwrap();
    
    // check if valid format
    if !regex.is_match(s) {
        return Err(String::from("Invalid date format. Must be MM/DD or MM/DD/YYYY"));
    }

    // extract captures
    let caps = regex.captures(s).unwrap();
    let month = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
    let day = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
    let year = caps.get(3).map_or(None, |m| {
        let y = m.as_str().parse::<u16>().unwrap();
        match y {
            0 ..= 99 => Some(2000 + y),  // assume 2000s if only 2-digit year
            _ => Some(y),
        }
    });

    // validate month-day combos
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => {
            if day > 31 {
                return Err(String::from("Invalid date. Month has at most 31 days"));
            }
        },
        4 | 6 | 9 | 11 => {
            if day > 30 {
                return Err(String::from("Invalid date. Month has at most 30 days"));
            }
        },
        2 => {
            if year.is_none() {
                if day > 28 {
                    return Err(String::from("Invalid date. February has at most 28 days"));
                }
            } else {
                if day > 29 {
                    return Err(String::from("Invalid date. February has at most 29 days"));
                }
            }
        },
        _ => return Err(String::from("Invalid date. Month must be between 1 and 12")),
    }

    Ok(Date::new(month, day, year))
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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