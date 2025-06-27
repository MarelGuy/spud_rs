use core::{fmt, str::FromStr};

use std::error::Error;

use chrono::{NaiveDateTime, NaiveTime, Timelike};

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
    pub fn new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self, Box<dyn Error>> {
        if hour > 23 {
            return Err("Hour must be between 0 and 23".into());
        }

        if minute > 59 {
            return Err("Minute must be between 0 and 59".into());
        }

        if second > 59 {
            return Err("Second must be between 0 and 59".into());
        }

        if nanosecond >= 1_000_000_000 {
            return Err("Nanosecond must be less than 1 billion".into());
        }

        Ok(Time {
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    pub(crate) fn as_le_bytes(self) -> Vec<u8> {
        velcro::vec![
            ..self.hour.to_le_bytes(),
            ..self.minute.to_le_bytes(),
            ..self.second.to_le_bytes(),
            ..self.nanosecond.to_le_bytes()
        ]
    }
}

impl TryFrom<NaiveTime> for Time {
    type Error = Box<dyn Error>;

    fn try_from(time: NaiveTime) -> Result<Self, Self::Error> {
        Ok(Time {
            hour: u8::try_from(time.hour()).map_err(|_| "hour out of range")?,
            minute: u8::try_from(time.minute()).map_err(|_| "minute out of range")?,
            second: u8::try_from(time.second()).map_err(|_| "second out of range")?,
            nanosecond: time.nanosecond(),
        })
    }
}

impl TryFrom<NaiveDateTime> for Time {
    type Error = Box<dyn Error>;

    fn try_from(time: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(Time {
            hour: u8::try_from(time.hour()).map_err(|_| "hour out of range")?,
            minute: u8::try_from(time.minute()).map_err(|_| "minute out of range")?,
            second: u8::try_from(time.second()).map_err(|_| "second out of range")?,
            nanosecond: time.nanosecond(),
        })
    }
}

impl FromStr for Time {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 && parts.len() != 4 {
            return Err(fmt::Error);
        }

        let hour: u8 = u8::from_str(parts[0]).map_err(|_| fmt::Error)?;
        let minute: u8 = u8::from_str(parts[1]).map_err(|_| fmt::Error)?;
        let second: u8 = u8::from_str(parts[2]).map_err(|_| fmt::Error)?;
        let nanosecond = if parts.len() == 4 {
            u32::from_str(parts[3]).map_err(|_| fmt::Error)?
        } else {
            0
        };

        Ok(Time {
            hour,
            minute,
            second,
            nanosecond,
        })
    }
}

impl TryFrom<Time> for NaiveTime {
    type Error = Box<dyn Error>;

    fn try_from(time: Time) -> Result<Self, Self::Error> {
        NaiveTime::from_hms_nano_opt(
            u32::from(time.hour),
            u32::from(time.minute),
            u32::from(time.second),
            time.nanosecond,
        )
        .ok_or_else(|| "Invalid time conversion".into())
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
