use chrono::NaiveDate;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub date: NaiveDate,
    pub feeling: u8,
}

impl Entry {
    pub fn new(date: NaiveDate, feeling: u8) -> Result<Self, EntryError> {
        if !(1..=10).contains(&feeling) {
            return Err(EntryError::InvalidFeeling(feeling));
        }
        if date > chrono::Local::now().date_naive() {
            return Err(EntryError::FutureDate(date));
        }
        Ok(Self { date, feeling })
    }

    pub fn to_csv_row(&self) -> String {
        format!("{},{}", self.date, self.feeling)
    }
}

impl FromStr for Entry {
    type Err = EntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date_str, feeling_str) = s
            .split_once(',')
            .ok_or_else(|| EntryError::ParseError(s.to_string()))?;

        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| EntryError::ParseError(s.to_string()))?;
        let feeling: u8 = feeling_str
            .parse()
            .map_err(|_| EntryError::ParseError(s.to_string()))?;

        if !(1..=10).contains(&feeling) {
            return Err(EntryError::InvalidFeeling(feeling));
        }

        Ok(Self { date, feeling })
    }
}

#[derive(Debug)]
pub enum EntryError {
    InvalidFeeling(u8),
    FutureDate(NaiveDate),
    ParseError(String),
}

impl fmt::Display for EntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFeeling(v) => write!(f, "feeling must be 1-10, got {v}"),
            Self::FutureDate(d) => write!(f, "date is in the future: {d}"),
            Self::ParseError(s) => write!(f, "failed to parse entry: {s}"),
        }
    }
}

impl std::error::Error for EntryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_entry() {
        let entry: Entry = "2024-03-15,7".parse().unwrap();
        assert_eq!(entry.feeling, 7);
        assert_eq!(entry.date, NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
    }

    #[test]
    fn reject_invalid_feeling() {
        assert!("2024-03-15,0".parse::<Entry>().is_err());
        assert!("2024-03-15,11".parse::<Entry>().is_err());
    }

    #[test]
    fn roundtrip_csv() {
        let entry: Entry = "2024-03-15,7".parse().unwrap();
        assert_eq!(entry.to_csv_row(), "2024-03-15,7");
    }
}
