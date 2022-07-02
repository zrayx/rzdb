use std::time::{SystemTime, UNIX_EPOCH};

pub struct Timestamp {
    timestamp: u64,
}

pub struct Time {
    seconds: u32,
}

pub struct Date {
    days: u32,
}

impl Date {
    pub fn today() -> Date {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let seconds = since_the_epoch.as_secs();
        let days = seconds / (60 * 60 * 24);
        Date { days: days as u32 }
    }
}

impl std::fmt::Display for Date {
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
        let date = Date { days: days as u32 };

        write!(f, "{} {}", date, time)
    }
}
