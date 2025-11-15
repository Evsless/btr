use crate::error::BtrError;
use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl Period {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Result<Self, BtrError> {
        if start > end {
            return Err(BtrError::InvalidPeriod(String::from(
                "Period beggining must be before the period end.",
            )));
        }

        Ok(Self { start, end })
    }

    pub fn current_month() -> Result<Self, BtrError> {
        let date = Utc::now().date_naive();

        Self::month(date.month(), date.year())
    }

    pub fn current_year() -> Result<Self, BtrError> {
        let date = Utc::now().date_naive();

        Self::year(date.year())
    }

    pub fn month(month: u32, year: i32) -> Result<Self, BtrError> {
        if !(1..=12).contains(&month) {
            return Err(BtrError::InvalidData(Some(String::from(
                "String must be in range from 1 to 12.",
            ))));
        }

        let start = NaiveDate::from_ymd_opt(year, month, 1).ok_or(BtrError::InvalidData(None))?;

        let end = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .and_then(|d| d.pred_opt())
        .ok_or(BtrError::InvalidData(None))?;

        Self::new(start, end)
    }

    pub fn year(year: i32) -> Result<Self, BtrError> {
        let start = NaiveDate::from_ymd_opt(year, 1, 1).ok_or(BtrError::InvalidData(None))?;

        let end = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .and_then(|d| d.pred_opt())
            .ok_or(BtrError::InvalidData(None))?;

        Self::new(start, end)
    }
}
