use chrono::Duration;
use druid::im::Vector;
use crate::core::DurationWrapper;

pub(crate) fn find_extremes(data: &Vector<(DurationWrapper, f32)>) -> (f32, f32)
{
    let mut min_value: f32 = 1000.0;
    let mut max_value: f32 = -1000.0;

    for value in data.iter()
    {
        if min_value > value.1
        {
            min_value = value.1;
        }

        if max_value < value.1
        {
            max_value = value.1;
        }
    }

    return (min_value.floor(), max_value.ceil());
}

pub(crate) fn convert_to_duration(data: &Vector<(DurationWrapper, f32)>) -> Vector<(Duration, f32)>
{
    println!("Converting data..");
    let mut converted_data : Vector<(Duration, f32)> = Vector::new();

    for value in data.iter()
    {
        converted_data.push_back((value.0.0, value.1));
    }

    return converted_data;
}