use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, PartialEq)]
pub struct Timestamp {
    timestamp: u64,
}

#[derive(Clone, PartialEq)]
pub struct Time {
    seconds: u32,
}

#[derive(Clone, PartialEq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

pub struct DateDays {
    days: u32,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }
    pub fn parse(s: &str) -> Result<Date, Box<dyn Error>> {
        if let Ok(date) = Date::parse_iso(s) {
            Ok(date)
        } else if let Ok(date) = Date::parse_us(s) {
            Ok(date)
        } else if let Ok(date) = Date::parse_eu(s) {
            Ok(date)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Not a date: {}", s),
            )))
        }
    }
    pub fn parse_iso(s: &str) -> Result<Date, Box<dyn Error>> {
        let mut parts = s.split('-');
        let year = parts.next().ok_or("Not a date")?.parse::<u16>()?;
        let month = parts.next().ok_or("Not a date")?.parse::<u8>()?;
        let day = parts.next().ok_or("Not a date")?.parse::<u8>()?;
        if Date::validate_date(year, month, day) {
            Ok(Date { year, month, day })
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Not a date: {}", s),
            )))
        }
    }

    pub fn parse_us(s: &str) -> Result<Date, Box<dyn Error>> {
        let mut parts = s.split('/');
        let month = parts.next().ok_or("Not a date")?.parse::<u8>()?;
        let day = parts.next().ok_or("Not a date")?.parse::<u8>()?;
        let year = parts.next().ok_or("Not a date")?.parse::<u16>()?;
        if Date::validate_date(year, month, day) {
            Ok(Date { year, month, day })
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Not a date: {}", s),
            )))
        }
    }
    pub fn parse_eu(s: &str) -> Result<Date, Box<dyn Error>> {
        let mut parts = s.split('.');
        let day = parts.next().ok_or("Not a date")?.parse::<u8>()?;
        let month = parts.next().ok_or("Not a date")?.parse::<u8>()?;
        let year_as_str = parts.next().ok_or("Not a date")?;
        let mut year = if year_as_str.is_empty() {
            Date::today().year
        } else {
            year_as_str.parse::<u16>()?
        };

        if year < 40 {
            year += 2000;
        } else if year < 100 {
            year += 1900;
        }
        if Date::validate_date(year, month, day) {
            Ok(Date { year, month, day })
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Not a date: {}", s),
            )))
        }
    }
    fn validate_date(year: u16, month: u8, day: u8) -> bool {
        //year >= 1970 && year < 2999 && month > 0 && month < 13 && day > 0 && day < 32
        (1970..3000).contains(&year) && (0..13).contains(&month) && (0..32).contains(&day)
    }
    pub fn today() -> Date {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let seconds = since_the_epoch.as_secs();
        let days = seconds / (60 * 60 * 24);
        let mut remaining_days = days;
        let mut year = 1970;
        let mut month = 1;
        let mut day = 1;

        while remaining_days > 0 {
            let days_in_year = if Timestamp::is_leap_year(year) {
                366
            } else {
                365
            };
            if remaining_days >= days_in_year {
                remaining_days -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        while remaining_days > 0 {
            let days_in_month = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if Timestamp::is_leap_year(year) {
                        29
                    } else {
                        28
                    }
                }
                _ => panic!("invalid month"),
            };
            if remaining_days >= days_in_month {
                remaining_days -= days_in_month;
                month += 1;
            } else {
                break;
            }
        }

        day += remaining_days;
        Date {
            year: year as u16,
            month: month as u8,
            day: day as u8,
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl DateDays {
    pub fn today() -> DateDays {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let seconds = since_the_epoch.as_secs();
        let days = seconds / (60 * 60 * 24);
        DateDays { days: days as u32 }
    }
}

impl std::fmt::Display for DateDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut remaining_days = self.days;
        let mut year = 1970;
        let mut month = 1;
        let mut day = 1;

        while remaining_days > 0 {
            let days_in_year = if Timestamp::is_leap_year(year) {
                366
            } else {
                365
            };
            if remaining_days >= days_in_year {
                remaining_days -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        while remaining_days > 0 {
            let days_in_month = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if Timestamp::is_leap_year(year) {
                        29
                    } else {
                        28
                    }
                }
                _ => panic!("invalid month"),
            };
            if remaining_days >= days_in_month {
                remaining_days -= days_in_month;
                month += 1;
            } else {
                break;
            }
        }

        day += remaining_days;
        write!(f, "{}-{:02}-{:02}", year, month, day)
    }
}

impl Time {
    pub fn new(seconds: u32) -> Time {
        Time { seconds }
    }

    pub fn now() -> Time {
        Timestamp::now().as_time()
    }

    pub fn parse(s: &str) -> Result<Time, Box<dyn Error>> {
        let mut parts = s.split(':');
        let hours = parts.next().ok_or("Not a time")?.parse::<u8>()?;
        let minutes = parts.next().ok_or("Not a time")?.parse::<u8>()?;
        let seconds = parts.next().ok_or("Not a time")?.parse::<u8>()?;
        if Time::validate_time(hours, minutes, seconds) {
            Ok(Time {
                seconds: hours as u32 * 3600 + minutes as u32 * 60 + seconds as u32,
            })
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Not a time: {}", s),
            )))
        }
    }

    fn validate_time(hours: u8, minutes: u8, seconds: u8) -> bool {
        hours < 24 && minutes < 60 && seconds < 60
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hours = self.seconds / (60 * 60);
        let minutes = (self.seconds % (60 * 60)) / 60;
        let seconds = self.seconds % 60;
        write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl Timestamp {
    pub fn now() -> Timestamp {
        Timestamp {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn is_leap_year(year: u64) -> bool {
        (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn to_filename_string(&self) -> String {
        format!("{}", self).as_str()[0..(4 + 1 + 2 + 1 + 2 + 1 + 2 + 1 + 2)]
            .to_string()
            .replace(':', ".")
            .replace(' ', "_")
    }

    pub fn as_time(&self) -> Time {
        Time::new(((self.timestamp + 60 * 60 * 2) % (60 * 60 * 24)) as u32)
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let timestamp_local = self.timestamp + 3600 * 2; // convert to local time
        let time = Time::new((timestamp_local % (60 * 60 * 24)) as u32);
        let days = timestamp_local / (60 * 60 * 24);
        let date = DateDays { days: days as u32 };

        write!(f, "{} {}", date, time)
    }
}
