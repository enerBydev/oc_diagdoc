//! Wrapper para fechas con parsing flexible.

use crate::errors::OcError;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// Fecha con parsing flexible.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OcDate(DateTime<Utc>);

impl OcDate {
    /// Fecha actual.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Desde DateTime.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Inner DateTime.
    pub fn inner(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Formato ISO.
    pub fn to_iso(&self) -> String {
        self.0.format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    /// Solo fecha.
    pub fn to_date(&self) -> String {
        self.0.format("%Y-%m-%d").to_string()
    }

    /// Días desde esta fecha.
    pub fn days_ago(&self) -> i64 {
        (Utc::now() - self.0).num_days()
    }

    /// ¿Es hoy?
    pub fn is_today(&self) -> bool {
        self.0.date_naive() == Utc::now().date_naive()
    }
}

impl Default for OcDate {
    fn default() -> Self {
        Self::now()
    }
}

impl FromStr for OcDate {
    type Err = OcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Intenta varios formatos
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(Self(dt.with_timezone(&Utc)));
        }

        if let Ok(dt) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            let dt = dt.and_hms_opt(0, 0, 0).unwrap();
            return Ok(Self(DateTime::from_naive_utc_and_offset(dt, Utc)));
        }

        Err(OcError::InvalidDate(s.to_string()))
    }
}

impl Serialize for OcDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_iso())
    }
}

impl<'de> Deserialize<'de> for OcDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for OcDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_date())
    }
}

impl std::ops::Deref for OcDate {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_parsing() {
        let d: OcDate = "2026-01-30".parse().unwrap();
        assert_eq!(d.to_date(), "2026-01-30");
    }
}
