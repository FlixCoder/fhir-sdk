//! FHIR Time, Date, DateTime and Instant types.

use std::{cmp::Ordering, str::FromStr};

use serde::{Deserialize, Serialize};
use time::{error::Parse, format_description::well_known::Rfc3339, OffsetDateTime};

use crate::error::DateFormatError;

/// FHIR instant type: <https://hl7.org/fhir/datatypes.html#instant>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Instant(#[serde(with = "time::serde::rfc3339")] pub OffsetDateTime);

/// FHIR date type: <https://hl7.org/fhir/datatypes.html#date>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Date {
	/// Date in the format of YYYY
	Year(i32),
	/// Date in the format of YYYY-MM
	YearMonth(i32, time::Month),
	/// Date in the format of YYYY-MM-DD
	Date(time::Date),
}

/// FHIR dateTime type: <https://hl7.org/fhir/datatypes.html#dateTime>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum DateTime {
	/// Date that does not contain time or timezone
	Date(Date),
	/// Full date and time
	DateTime(Instant),
}

/// FHIR time type: <https://hl7.org/fhir/datatypes.html#time>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Time(#[serde(with = "serde_time")] pub time::Time);

/// Serde module for serialize and deserialize function for the type.
mod serde_time {
	use serde::{Deserialize, Serialize};
	use time::{format_description::FormatItem, macros::format_description};

	/// Time format `hh:mm:ss`.
	const TIME_FORMAT: &[FormatItem<'_>] = format_description!("[hour]:[minute]:[second]");
	/// Time format for `hh:mm:ss[.SSS]`.
	const TIME_FORMAT_SUBSEC: &[FormatItem<'_>] = fhir_time_format();

	/// Time format with optional subseconds.
	const fn fhir_time_format() -> &'static [FormatItem<'static>] {
		/// Optional subseconds.
		const OPTIONAL_SUB_SECONDS: FormatItem<'_> =
			FormatItem::Optional(&FormatItem::Compound(format_description!(".[subsecond]")));
		&[FormatItem::Compound(TIME_FORMAT), OPTIONAL_SUB_SECONDS]
	}

	/// Serialize time, using subseconds iff appropriate.
	#[allow(clippy::trivially_copy_pass_by_ref)] // Parameter types are set by serde.
	pub fn serialize<S>(time: &time::Time, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let format = if time.nanosecond() == 0 { TIME_FORMAT } else { TIME_FORMAT_SUBSEC };
		time.format(format).map_err(serde::ser::Error::custom)?.serialize(serializer)
	}

	/// Deserialize time, subseconds optional.
	pub fn deserialize<'de, D>(deserializer: D) -> Result<time::Time, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		time::Time::parse(&string, TIME_FORMAT_SUBSEC).map_err(serde::de::Error::custom)
	}
}

impl Serialize for Date {
	/// Serialize date.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self {
			// Serialize YYYY
			Date::Year(year) => {
				if (1000..10000).contains(year) {
					year.to_string().serialize(serializer)
				} else {
					Err(serde::ser::Error::custom("Year is not 4 digits long"))
				}
			}
			// Serialize YYYY-MM
			Date::YearMonth(year, month) => {
				if (1000..10000).contains(year) {
					serializer.serialize_str(&format!("{year}-{:02}", *month as u8))
				} else {
					Err(serde::ser::Error::custom("Year is not 4 digits long"))
				}
			}
			// Serialize YYYY-MM-DD
			Date::Date(date) => {
				/// Full date format
				const FORMAT: &[time::format_description::FormatItem<'_>] =
					time::macros::format_description!("[year]-[month]-[day]");
				date.format(FORMAT).map_err(serde::ser::Error::custom)?.serialize(serializer)
			}
		}
	}
}

impl FromStr for Date {
	type Err = DateFormatError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Split date into parts
		// YYYY(1)-MM(2)-DD(3)
		match s.split('-').count() {
			1 => Ok(Date::Year(s.parse::<i32>()?)),
			2 => {
				let (year, month) = s.split_once('-').ok_or(DateFormatError::StringSplit)?;
				// Convert strings into integers
				let year = year.parse::<i32>()?;
				let month = month.parse::<u8>()?;

				Ok(Date::YearMonth(year, month.try_into()?))
			}
			3 => {
				/// Full date format
				const FORMAT: &[time::format_description::FormatItem<'_>] =
					time::macros::format_description!("[year]-[month]-[day]");
				Ok(Date::Date(time::Date::parse(s, FORMAT)?))
			}
			_ => Err(DateFormatError::InvalidDate),
		}
	}
}

impl<'de> Deserialize<'de> for Date {
	/// Deserialize date.
	fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Date::from_str(&string).map_err(serde::de::Error::custom)
	}
}

impl FromStr for DateTime {
	type Err = DateFormatError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.contains('T') {
			let instant = Instant::from_str(s)?;
			Ok(DateTime::DateTime(instant))
		} else {
			let date = Date::from_str(s)?;
			Ok(DateTime::Date(date))
		}
	}
}

impl<'de> Deserialize<'de> for DateTime {
	/// Deserialize datetime.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Self::from_str(&string).map_err(serde::de::Error::custom)
	}
}

impl FromStr for Instant {
	type Err = Parse;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Instant(OffsetDateTime::parse(s, &Rfc3339)?))
	}
}

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Date::Date(ld), r) => ld.partial_cmp(r),
			(l, Date::Date(rd)) => l.partial_cmp(rd),
			(Date::Year(ly), Date::Year(ry)) => ly.partial_cmp(ry),
			(Date::Year(ly), Date::YearMonth(ry, _rm)) => ly.partial_cmp(ry),
			(Date::YearMonth(ly, _lm), Date::Year(ry)) => ly.partial_cmp(ry),
			(Date::YearMonth(ly, lm), Date::YearMonth(ry, rm)) => match ly.partial_cmp(ry)? {
				Ordering::Less => Some(Ordering::Less),
				Ordering::Greater => Some(Ordering::Greater),
				Ordering::Equal => (*lm as u8).partial_cmp(&(*rm as u8)),
			},
		}
	}
}

impl PartialOrd for DateTime {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(DateTime::Date(ld), DateTime::Date(rd)) => ld.partial_cmp(rd),
			(DateTime::Date(ld), DateTime::DateTime(Instant(rdtm))) => ld.partial_cmp(&rdtm.date()),
			(DateTime::DateTime(Instant(ldtm)), DateTime::Date(rd)) => ldtm.date().partial_cmp(rd),
			(DateTime::DateTime(ldtm), DateTime::DateTime(rdtm)) => ldtm.partial_cmp(rdtm),
		}
	}
}

impl PartialEq<time::Date> for Date {
	fn eq(&self, other: &time::Date) -> bool {
		match self {
			Self::Year(year) => *year == other.year(),
			Self::YearMonth(year, month) => *year == other.year() && *month == other.month(),
			Self::Date(date) => date == other,
		}
	}
}

impl PartialEq<Date> for time::Date {
	fn eq(&self, other: &Date) -> bool {
		match other {
			Date::Year(year) => self.year() == *year,
			Date::YearMonth(year, month) => self.year() == *year && self.month() == *month,
			Date::Date(date) => self == date,
		}
	}
}

impl PartialOrd<time::Date> for Date {
	fn partial_cmp(&self, other: &time::Date) -> Option<Ordering> {
		match self {
			Date::Year(year) => year.partial_cmp(&other.year()),
			Date::YearMonth(year, month) => match year.partial_cmp(&other.year())? {
				Ordering::Less => Some(Ordering::Less),
				Ordering::Greater => Some(Ordering::Greater),
				Ordering::Equal => (*month as u8).partial_cmp(&(other.month() as u8)),
			},
			Date::Date(date) => date.partial_cmp(other),
		}
	}
}

impl PartialOrd<Date> for time::Date {
	fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
		match other {
			Date::Year(year) => self.year().partial_cmp(year),
			Date::YearMonth(year, month) => match self.year().partial_cmp(year)? {
				Ordering::Less => Some(Ordering::Less),
				Ordering::Greater => Some(Ordering::Greater),
				Ordering::Equal => (self.month() as u8).partial_cmp(&(*month as u8)),
			},
			Date::Date(date) => self.partial_cmp(date),
		}
	}
}

impl PartialEq<OffsetDateTime> for DateTime {
	fn eq(&self, other: &OffsetDateTime) -> bool {
		match self {
			Self::Date(date) => *date == other.date(),
			Self::DateTime(Instant(datetime)) => datetime == other,
		}
	}
}

impl PartialEq<DateTime> for OffsetDateTime {
	fn eq(&self, other: &DateTime) -> bool {
		match other {
			DateTime::Date(date) => self.date() == *date,
			DateTime::DateTime(Instant(datetime)) => self == datetime,
		}
	}
}

impl PartialOrd<OffsetDateTime> for DateTime {
	fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
		match self {
			DateTime::Date(date) => date.partial_cmp(&other.date()),
			DateTime::DateTime(Instant(datetime)) => datetime.partial_cmp(other),
		}
	}
}

impl PartialOrd<DateTime> for OffsetDateTime {
	fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
		match other {
			DateTime::Date(date) => self.date().partial_cmp(date),
			DateTime::DateTime(Instant(datetime)) => self.partial_cmp(datetime),
		}
	}
}
