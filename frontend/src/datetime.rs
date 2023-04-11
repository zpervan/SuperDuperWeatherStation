use chrono::{DateTime, Utc, Duration, NaiveTime};
use druid::im::Vector;

use crate::core::DurationWrapper;

pub fn parse_time(datetime: &String) -> Duration
{
    let converted_datetime = DateTime::parse_from_rfc3339(&datetime).unwrap();
    let local_datetime = converted_datetime.with_timezone(&Utc);
    local_datetime.time().signed_duration_since(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}

pub fn convert_to_duration(data: &Vector<(DurationWrapper, f32)>) -> Vector<(Duration, f32)>
{
    println!("Converting data..");
    let mut converted_data: Vector<(Duration, f32)> = Vector::new();

    for value in data.iter()
    {
        converted_data.push_back((value.0.0, value.1));
    }

    return converted_data;
}