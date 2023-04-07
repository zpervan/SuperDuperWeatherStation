use std::thread;
use chrono::Duration;
use druid::{AppDelegate, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, Handled, Selector, Target};
use druid::im::Vector;
use reqwest::blocking::get;
use serde_json::Value;
use crate::datetime;

const PROCESSING_FINISHED: Selector<ApplicationData> = Selector::new("processing_finished");

#[derive(Clone)]
pub struct DurationWrapper(pub Duration);

impl Data for DurationWrapper
{
    fn same(&self, other: &Self) -> bool
    {
        self.0.eq(&other.0)
    }
}

#[allow(clippy::rc_buffer)]
#[derive(Clone, Data, Lens)]
pub struct ApplicationData
{
    pub temperature: Vector<(DurationWrapper, f32)>,
    pub humidity: Vector<(DurationWrapper, f32)>,
    pub dates: Vector<String>,
    pub current_date: String,
}

// @TODO: Add a functionality which will only fetch the dates and
impl ApplicationData
{
    pub fn new() -> Self
    {
        ApplicationData
        {
            temperature: Vector::new(),
            humidity: Vector::new(),
            dates: Vector::new(),
            // @TODO: When initialized, fill this with the latest date from the dates vector
            current_date: "01012000".to_string(),
        }
    }

    pub fn populate_weather_data(&mut self, sink: ExtEventSink)
    {
        thread::spawn(move || {
            println!("Populating weather data for plotting");
            let parsed_data: Value = serde_json::from_str(get("http://localhost:3500/get").unwrap().text().unwrap().as_str()).unwrap();

            let mut app_data = ApplicationData::new();

            for value in parsed_data.as_array().unwrap()
            {
                let time = datetime::parse_time(&value["created_on"].to_string().replace("\"", ""));

                app_data.temperature.push_back((DurationWrapper(time), value["temperature"].as_f64().unwrap() as f32));
                app_data.humidity.push_back((DurationWrapper(time), value["humidity"].as_f64().unwrap() as f32));
            }

            sink.submit_command(PROCESSING_FINISHED, app_data, Target::Auto).expect("Failed to submit command");
        });
    }
}

pub(crate) struct Delegate;

impl AppDelegate<ApplicationData> for Delegate
{
    fn command(&mut self, _: &mut DelegateCtx, _: Target, cmd: &Command, data: &mut ApplicationData, _: &Env) -> Handled {
        if let Some(processed_data) = cmd.get(PROCESSING_FINISHED) {
            data.temperature = processed_data.temperature.clone();
            data.humidity = processed_data.humidity.clone();

            Handled::Yes
        } else {
            Handled::No
        }
    }
}