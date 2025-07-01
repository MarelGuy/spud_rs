use core::{fmt, str::FromStr};

use chrono::{Datelike, NaiveDate, NaiveDateTime};

use crate::SpudError;

/// A struct representing a date in the format YYYY-MM-DD.
/// This struct can be created from chrono's `NaiveDate` or `NaiveDateTime`,
/// and can also be parsed from a string in the same format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    /// Creates a new `Date` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the month is not between 1 and 12, or if the day is not valid for the given month and year.
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, SpudError> {
        if !(1..=12).contains(&month) {
            return Err(SpudError::ValidationError(
                "The month must be between 1 and 12".into(),
            ));
        }

        let max_days: u8 = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };

        if !(1..=max_days).contains(&day) {
            return Err(SpudError::ValidationError(format!(
                "Invalid day. The month {month} of the year {year} has {max_days} days."
            )));
        }

        Ok(Date { year, month, day })
    }

    pub(crate) fn as_le_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(4);

        bytes.extend_from_slice(&self.year.to_le_bytes());
        bytes.push(self.month);
        bytes.push(self.day);

        bytes
    }
}

impl TryFrom<NaiveDate> for Date {
    type Error = SpudError;

    fn try_from(date: NaiveDate) -> Result<Self, Self::Error> {
        Ok(Date {
            year: u16::try_from(date.year())
                .map_err(|_| SpudError::ValidationError("Invalid year".to_owned()))?,
            month: u8::try_from(date.month())
                .map_err(|_| SpudError::ValidationError("Invalid month".to_owned()))?,
            day: u8::try_from(date.day())
                .map_err(|_| SpudError::ValidationError("Invalid day".to_owned()))?,
        })
    }
}

impl TryFrom<NaiveDateTime> for Date {
    type Error = SpudError;

    fn try_from(date: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(Date {
            year: u16::try_from(date.year())
                .map_err(|_| SpudError::ValidationError("Invalid year".to_owned()))?,
            month: u8::try_from(date.month())
                .map_err(|_| SpudError::ValidationError("Invalid month".to_owned()))?,
            day: u8::try_from(date.day())
                .map_err(|_| SpudError::ValidationError("Invalid day".to_owned()))?,
        })
    }
}

impl FromStr for Date {
    type Err = fmt::Error;

    /// Parses a string in the format "YYYY-MM-DD" into a `Date` instance.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 3 {
            return Err(fmt::Error);
        }

        let year: u16 = u16::from_str(parts[0]).map_err(|_| fmt::Error)?;
        let month: u8 = u8::from_str(parts[1]).map_err(|_| fmt::Error)?;
        let day: u8 = u8::from_str(parts[2]).map_err(|_| fmt::Error)?;

        Ok(Date { year, month, day })
    }
}

impl TryFrom<Date> for NaiveDate {
    type Error = SpudError;

    fn try_from(date: Date) -> Result<Self, Self::Error> {
        NaiveDate::from_ymd_opt(
            i32::from(date.year),
            u32::from(date.month),
            u32::from(date.day),
        )
        .ok_or_else(|| SpudError::ValidationError("Invalid date".to_owned()))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
