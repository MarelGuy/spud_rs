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
        let date: Date = Date { year, month, day };

        date.check_validity()?;

        Ok(date)
    }

    pub(crate) fn as_le_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(4);

        bytes.extend_from_slice(&self.year.to_le_bytes());
        bytes.push(self.month);
        bytes.push(self.day);

        bytes
    }

    fn check_validity(self) -> Result<(), SpudError> {
        if !(1..=12).contains(&self.month) {
            return Err(SpudError::ValidationError(
                "The month must be between 1 and 12".into(),
            ));
        }

        let max_days: u8 = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };

        if !(1..=max_days).contains(&self.day) {
            return Err(SpudError::ValidationError(format!(
                "Invalid day. The month {} of the year {} has {} days.",
                self.month, self.year, max_days
            )));
        }

        Ok(())
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

        let date: Date = Date { year, month, day };

        date.check_validity().map_err(|_| fmt::Error)?;

        Ok(date)
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

#[cfg(test)]
mod tests {
    use chrono::NaiveTime;

    use super::*;

    #[test]
    fn test_date_creation() {
        let date: Result<Date, SpudError> = Date::new(2023, 3, 15);

        assert!(date.is_ok());
    }

    #[test]
    fn test_date_creation_invalid() {
        let date: Result<Date, SpudError> = Date::new(2023, 13, 15);

        assert!(date.is_err());

        let date: Result<Date, SpudError> = Date::new(2023, 2, 30);

        assert!(date.is_err());
    }

    #[test]
    fn test_date_display() {
        let date: Date = Date::new(2023, 3, 15).unwrap();

        assert_eq!(date.to_string(), "2023-03-15");
    }

    #[test]
    fn test_date_from_naive_date() {
        let naive_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 3, 15).unwrap();
        let date: Result<Date, SpudError> = Date::try_from(naive_date);

        assert!(date.is_ok());
        assert_eq!(date.unwrap().to_string(), "2023-03-15");
    }

    #[test]
    fn test_date_from_naive_date_time() {
        let naive_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 3, 15).unwrap();
        let naive_time: NaiveTime = NaiveTime::from_hms_nano_opt(12, 30, 45, 500_000_000).unwrap();

        let naive_datetime: NaiveDateTime = NaiveDateTime::new(naive_date, naive_time);

        let date: Result<Date, SpudError> = Date::try_from(naive_datetime);

        assert!(date.is_ok());
        assert_eq!(date.unwrap().to_string(), "2023-03-15");
    }

    #[test]
    fn test_date_from_str() {
        let date_str: &str = "2023-03-15";
        let date: Result<Date, fmt::Error> = Date::from_str(date_str);

        assert!(date.is_ok());
        assert_eq!(date.unwrap().to_string(), "2023-03-15");
    }

    #[test]
    fn test_date_from_str_invalid() {
        let invalid_date_str: &str = "2023-13-15";
        let date: Result<Date, fmt::Error> = Date::from_str(invalid_date_str);

        assert!(date.is_err());

        let invalid_date_str: &str = "2023-02-30";
        let date: Result<Date, fmt::Error> = Date::from_str(invalid_date_str);

        assert!(date.is_err());
    }

    #[test]
    fn test_date_to_naive_date() {
        let date: Date = Date::new(2023, 3, 15).unwrap();
        let naive_date: Result<NaiveDate, SpudError> = NaiveDate::try_from(date);

        assert!(naive_date.is_ok());
        assert_eq!(naive_date.unwrap().to_string(), "2023-03-15");
    }

    #[test]
    fn test_date_to_le_bytes() {
        let date: Date = Date::new(2023, 3, 15).unwrap();
        let bytes: Vec<u8> = date.as_le_bytes();

        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes[0..2], [0xe7, 0x07]); // 2023 in little-endian
        assert_eq!(bytes[2], 3); // March
        assert_eq!(bytes[3], 15); // 15th day
    }
}
