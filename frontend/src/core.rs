use chrono::Duration;
use druid::{AppDelegate, Command, Data, DelegateCtx, Env, Lens, Handled, Target};
use druid::im::Vector;

use crate::requests;

#[derive(Clone)]
pub struct DurationWrapper(pub Duration);

impl Data for DurationWrapper
{
    fn same(&self, other: &Self) -> bool
    {
        self.0.eq(&other.0)
    }
}

#[derive(Clone, Data, Lens)]
pub struct ApplicationData
{
    pub temperature: Vector<(DurationWrapper, f32)>,
    pub humidity: Vector<(DurationWrapper, f32)>,
    pub current_date: String,
}

impl ApplicationData
{
    pub fn new() -> Self
    {
        let mut empty_dates: Vector<(String, String)> = Vector::new();
        empty_dates.push_back(("N/A".to_string(), "N/A".to_string()));

        ApplicationData
        {
            temperature: Vector::new(),
            humidity: Vector::new(),
            current_date: requests::get_latest_date(),
        }
    }
}

pub(crate) struct Delegate;

impl AppDelegate<ApplicationData> for Delegate
{
    fn command(&mut self, _: &mut DelegateCtx, _: Target, cmd: &Command, data: &mut ApplicationData, _: &Env) -> Handled
    {
        if let Some(processed_weather_data) = cmd.get(requests::FETCHED_WEATHER_DATA)
        {
            data.temperature = processed_weather_data.temperature.clone();
            data.humidity = processed_weather_data.humidity.clone();

            return Handled::Yes;
        }

        Handled::No
    }
}