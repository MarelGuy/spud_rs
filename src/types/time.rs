use core::{fmt, str::FromStr};

use chrono::{NaiveDateTime, NaiveTime, Timelike};

use crate::SpudError;

/// A struct representing a time in the format HH:MM:SS.NS.
/// This struct can be created from chrono's `NaiveTime` or `NaiveDateTime`,
/// and can also be parsed from a string in the same format.
///
/// # Notes
/// - The `NS` (nanoseconds) part is optional. If not provided, it defaults to `0` and won't be displayed when converting to string.
/// - This struct does not handle time zones or daylight saving time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl Time {
    /// Creates a new `Time` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the hour is not between 0 and 23, minute is not between 0 and 59,
    pub fn new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self, SpudError> {
        if hour > 23 {
            return Err(SpudError::ValidationError(
                "Hour must be between 0 and 23".to_owned(),
            ));
        }

        if minute > 59 {
            return Err(SpudError::ValidationError(
                "Minute must be between 0 and 59".to_owned(),
            ));
        }

        if second > 59 {
            return Err(SpudError::ValidationError(
                "Second must be between 0 and 59".to_owned(),
            ));
        }

        if nanosecond >= 1_000_000_000 {
            return Err(SpudError::ValidationError(
                "Nanosecond must be less than 1 billion".to_owned(),
            ));
        }

        Ok(Time {
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    pub(crate) fn as_le_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(1 + 1 + 1 + 4);

        bytes.extend_from_slice(&self.hour.to_le_bytes());
        bytes.extend_from_slice(&self.minute.to_le_bytes());
        bytes.extend_from_slice(&self.second.to_le_bytes());
        bytes.extend_from_slice(&self.nanosecond.to_le_bytes());

        bytes
    }
}

impl TryFrom<NaiveTime> for Time {
    type Error = SpudError;

    fn try_from(time: NaiveTime) -> Result<Self, Self::Error> {
        Ok(Time {
            hour: u8::try_from(time.hour())
                .map_err(|_| SpudError::ValidationError("hour out of range".to_owned()))?,
            minute: u8::try_from(time.minute())
                .map_err(|_| SpudError::ValidationError("minute out of range".to_owned()))?,
            second: u8::try_from(time.second())
                .map_err(|_| SpudError::ValidationError("second out of range".to_owned()))?,
            nanosecond: time.nanosecond(),
        })
    }
}

impl TryFrom<NaiveDateTime> for Time {
    type Error = SpudError;

    fn try_from(time: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(Time {
            hour: u8::try_from(time.hour())
                .map_err(|_| SpudError::ValidationError("hour out of range".to_owned()))?,
            minute: u8::try_from(time.minute())
                .map_err(|_| SpudError::ValidationError("minute out of range".to_owned()))?,
            second: u8::try_from(time.second())
                .map_err(|_| SpudError::ValidationError("second out of range".to_owned()))?,
            nanosecond: time.nanosecond(),
        })
    }
}

impl FromStr for Time {
    type Err = SpudError;

    /// Parses a string in the format "HH:MM:SS" or "HH:MM:SS.NS" into a `Time` instance.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 3 {
            return Err(SpudError::ValidationError("Invalid time format".to_owned()));
        }

        if parts[2].contains('.') {
            let ns_parts: Vec<&str> = parts[2].split('.').collect();

            parts[2] = ns_parts[0];
            parts.push(ns_parts[1]);
        }

        let hour: u8 = u8::from_str(parts[0])
            .map_err(|_| SpudError::ValidationError("Invalid hour".to_owned()))?;

        let minute: u8 = u8::from_str(parts[1])
            .map_err(|_| SpudError::ValidationError("Invalid minute".to_owned()))?;

        let second: u8 = u8::from_str(parts[2])
            .map_err(|_| SpudError::ValidationError("Invalid second".to_owned()))?;

        let nanosecond: u32 = if parts.len() > 3 {
            u32::from_str(parts[3])
                .map_err(|_| SpudError::ValidationError("Invalid nanosecond".to_owned()))?
        } else {
            0
        };

        if hour > 23 {
            return Err(SpudError::ValidationError(
                "Hour must be between 0 and 23".to_owned(),
            ));
        }

        if minute > 59 {
            return Err(SpudError::ValidationError(
                "Minute must be between 0 and 59".to_owned(),
            ));
        }

        if second > 59 {
            return Err(SpudError::ValidationError(
                "Second must be between 0 and 59".to_owned(),
            ));
        }

        if nanosecond >= 1_000_000_000 {
            return Err(SpudError::ValidationError(
                "Nanosecond must be less than 1 billion".to_owned(),
            ));
        }

        Ok(Time {
            hour,
            minute,
            second,
            nanosecond,
        })
    }
}

impl TryFrom<Time> for NaiveTime {
    type Error = SpudError;

    fn try_from(time: Time) -> Result<Self, Self::Error> {
        NaiveTime::from_hms_nano_opt(
            u32::from(time.hour),
            u32::from(time.minute),
            u32::from(time.second),
            time.nanosecond,
        )
        .ok_or_else(|| SpudError::ValidationError("Invalid time conversion".to_owned()))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.nanosecond == 0 {
            write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
        } else {
            write!(
                f,
                "{:02}:{:02}:{:02}.{:09}",
                self.hour, self.minute, self.second, self.nanosecond
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_time_creation() {
        let time: Time = Time::new(12, 30, 45, 500_000_000).unwrap();

        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 45);
        assert_eq!(time.nanosecond, 500_000_000);
    }

    #[test]
    fn test_time_creation_invalid() {
        assert!(Time::new(24, 0, 0, 0).is_err());
        assert!(Time::new(23, 60, 0, 0).is_err());
        assert!(Time::new(23, 59, 60, 0).is_err());
        assert!(Time::new(23, 59, 59, 1_000_000_000).is_err());
    }

    #[test]
    fn test_time_from_naive_time() {
        let naive_time: NaiveTime = NaiveTime::from_hms_nano_opt(12, 30, 45, 500_000_000).unwrap();
        let time: Time = naive_time.try_into().unwrap();

        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 45);
        assert_eq!(time.nanosecond, 500_000_000);
    }

    #[test]
    fn test_time_from_naive_date_time() {
        let naive_date: NaiveDate = NaiveDate::from_ymd_opt(2023, 3, 15).unwrap();
        let naive_time: NaiveTime = NaiveTime::from_hms_nano_opt(12, 30, 45, 500_000_000).unwrap();

        let naive_datetime: NaiveDateTime = NaiveDateTime::new(naive_date, naive_time);

        let time: Time = naive_datetime.try_into().unwrap();

        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 45);
        assert_eq!(time.nanosecond, 500_000_000);
    }

    #[test]
    fn test_time_to_naive_time() {
        let time: Time = Time::new(12, 30, 45, 500_000_000).unwrap();
        let naive_time: NaiveTime = time.try_into().unwrap();

        assert_eq!(naive_time.hour(), 12);
        assert_eq!(naive_time.minute(), 30);
        assert_eq!(naive_time.second(), 45);
        assert_eq!(naive_time.nanosecond(), 500_000_000);
    }

    #[test]
    fn test_display_time() {
        let time: Time = Time::new(12, 30, 45, 500_000_000).unwrap();

        assert_eq!(time.to_string(), "12:30:45.500000000");

        let time_no_ns: Time = Time::new(12, 30, 45, 0).unwrap();

        assert_eq!(time_no_ns.to_string(), "12:30:45");
    }

    #[test]
    fn test_parse_time() {
        let time: Time = "12:30:45.500000000".parse().unwrap();

        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 30);
        assert_eq!(time.second, 45);
        assert_eq!(time.nanosecond, 500_000_000);

        let time_no_ns: Time = "12:30:45".parse().unwrap();

        assert_eq!(time_no_ns.hour, 12);
        assert_eq!(time_no_ns.minute, 30);
        assert_eq!(time_no_ns.second, 45);
        assert_eq!(time_no_ns.nanosecond, 0);
    }

    #[test]
    fn test_parse_invalid_time() {
        assert!("25:00:00".parse::<Time>().is_err());
        assert!("12:60:00".parse::<Time>().is_err());
        assert!("12:30:60".parse::<Time>().is_err());
        assert!("12:30:45.1000000000".parse::<Time>().is_err());
        assert!("12:30".parse::<Time>().is_err());
    }
}
