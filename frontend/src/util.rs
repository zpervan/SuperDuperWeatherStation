use druid::im::Vector;
use crate::core::DurationWrapper;

/// Finds the minimum and maximum value for the given vector with duration (date) values
///
/// Returns a pair containing min and max duration (dates)
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
