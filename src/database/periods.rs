use core::fmt;

use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum PeriodError {
    InvalidData,
    InvalidDateRange,
}

impl fmt::Display for PeriodError {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
      match self {
          PeriodError::InvalidData => {
            write!(f, "Failed to create a date.")
          },
          PeriodError::InvalidDateRange => {
            write!(f, "Start date must be before end date.")
          }
      } 
   } 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate
}

impl Period {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Result<Self, PeriodError> {
        if start > end {
            return Err(PeriodError::InvalidDateRange);
        }

        Ok(Self { start, end })
    }

    pub fn current_month() -> Result<Self, PeriodError> {
        let date = Utc::now().date_naive();

        Self::month(date.month(), date.year())
    }

    pub fn current_year() -> Result<Self, PeriodError> {
        let date = Utc::now().date_naive();

        Self::year(date.year())
    }

    pub fn month(month: u32, year: i32) -> Result<Self, PeriodError> {
        if !(1..=12).contains(&month) {
            return Err(PeriodError::InvalidData);
        }

        let start = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or(PeriodError::InvalidData)?;

        let end = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .and_then(|d| d.pred_opt())
        .ok_or(PeriodError::InvalidData)?;

        Self::new(start, end)
    }

    pub fn year(year: i32) -> Result<Self, PeriodError> {
        let start = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or(PeriodError::InvalidData)?;

        let end = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .and_then(|d| d.pred_opt())
            .ok_or(PeriodError::InvalidData)?;

        Self::new(start, end)
    }
}
