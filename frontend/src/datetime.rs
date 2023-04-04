use chrono::{DateTime, Utc, Duration, NaiveTime};

pub fn parse_time(datetime: &String) -> Duration
{
    let converted_datetime = DateTime::parse_from_rfc3339(&datetime).unwrap();
    let local_datetime = converted_datetime.with_timezone(&Utc);
    local_datetime.time().signed_duration_since(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}