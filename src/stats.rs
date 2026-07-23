use crate::model::Entry;
use chrono::NaiveDate;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub count: u32,
    pub average: f64,
    pub min: u32,
    pub max: u32,
}

impl Stats {
    pub fn from_entries(entries: &[Entry], start: NaiveDate, end: NaiveDate) -> Self {
        let in_range: Vec<&Entry> = entries.iter().filter(|e| e.date >= start && e.date <= end).collect();
        let count = in_range.len() as u32;
        if count == 0 {
            return Self{ count:0, average: 0.0, min: 0, max:0}
        }

        let sum: u32 = in_range.iter().map(|e| e.feeling as u32).sum();
        let max = in_range.iter().map(|e| e.feeling as u32).max().unwrap();
        let min = in_range.iter().map(|e| e.feeling as u32).min().unwrap();
        let average = sum as f64 / count as f64;

        Self{count, average, min, max}
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.count == 0 {
            write!(f, " no stats")
        } else {
            write!(f, " avg:[{:.1}] min:[{}] max:[{}] count:[{}]", self.average, self.min, self.max, self.count)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_range_is_zeroed() {
        let stats = Stats::from_entries(
            &[],
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        assert_eq!(stats.count, 0);
    }

    #[test]
    fn computes_avg_min_max() {
        let entries = vec![
            "2024-01-01,6".parse().unwrap(),
            "2024-01-02,8".parse().unwrap(),
        ];
        let stats = Stats::from_entries(
            &entries,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, 6);
        assert_eq!(stats.max, 8);
        assert_eq!(stats.average, 7.0);
    }

    #[test]
    fn excludes_out_of_range() {
        let entries = vec!["2023-12-31,3".parse::<Entry>().unwrap()];
        let stats = Stats::from_entries(
            &entries,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        );
        assert_eq!(stats.count, 0);
    }

  }
