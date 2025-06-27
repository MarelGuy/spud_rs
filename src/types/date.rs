use core::{fmt, str::FromStr};
use std::error::Error;

use chrono::{Datelike, NaiveDate, NaiveDateTime};

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
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, Box<dyn Error>> {
        // Il controllo sul mese rimane invariato
        if !(1..=12).contains(&month) {
            return Err("Il mese deve essere compreso tra 1 e 12".into());
        }

        // Determiniamo il numero massimo di giorni per il mese inserito
        let max_days = match month {
            // Mesi con 31 giorni
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            // Mesi con 30 giorni
            4 | 6 | 9 | 11 => 30,
            // Febbraio
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            // Questo caso è teoricamente irraggiungibile grazie al primo controllo
            _ => unreachable!(),
        };

        // Ora controlliamo che il giorno sia valido per quel mese specifico
        if !(1..=max_days).contains(&day) {
            return Err(format!(
                "Giorno non valido. Il mese {month} dell'anno {year} ha {max_days} giorni."
            )
            .into());
        }

        Ok(Date { year, month, day })
    }

    pub(crate) fn as_le_bytes(self) -> Vec<u8> {
        velcro::vec![
            ..self.year.to_le_bytes(),
            ..self.month.to_le_bytes(),
            ..self.day.to_le_bytes()
        ]
    }
}

impl TryFrom<NaiveDate> for Date {
    type Error = Box<dyn Error>;

    fn try_from(date: NaiveDate) -> Result<Self, Self::Error> {
        Ok(Date {
            year: u16::try_from(date.year()).map_err(|_| "Invalid year")?,
            month: u8::try_from(date.month()).map_err(|_| "Invalid month")?,
            day: u8::try_from(date.day()).map_err(|_| "Invalid day")?,
        })
    }
}

impl TryFrom<NaiveDateTime> for Date {
    type Error = Box<dyn Error>;

    fn try_from(date: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(Date {
            year: u16::try_from(date.year()).map_err(|_| "Invalid year")?,
            month: u8::try_from(date.month()).map_err(|_| "Invalid month")?,
            day: u8::try_from(date.day()).map_err(|_| "Invalid day")?,
        })
    }
}

impl FromStr for Date {
    type Err = fmt::Error;
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
    type Error = Box<dyn Error>;

    fn try_from(date: Date) -> Result<Self, Self::Error> {
        NaiveDate::from_ymd_opt(
            i32::from(date.year),
            u32::from(date.month),
            u32::from(date.day),
        )
        .ok_or_else(|| "Invalid date conversion".into())
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
