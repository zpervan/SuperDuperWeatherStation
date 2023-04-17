use druid::{ExtEventSink, Selector, Target};
use chrono::NaiveDate;
use serde_json::Value;
use reqwest::blocking::get;

use crate::datetime;
use crate::core::{ApplicationData, DurationWrapper};

use std::thread;

// Commands
pub const FETCHED_WEATHER_DATA: Selector<ApplicationData> = Selector::new("fetched_weather_data");

// Endpoints
// @TODO: Check if it's possible to combine the endpoint str's during the init phase
const BASE_URL: &str = "http://192.168.1.6:3500";
const FETCH_WEATHER_DATA_BY_DATE_ENDPOINT: &str = "/get/";
const FETCH_DATES_ENDPOINT: &str = "/dates";
const FETCH_LATEST_DATE_ENDPOINT: &str = "/latest";

pub fn get_weather_data(sink: ExtEventSink, date: String)
{
    thread::spawn(move || {
        println!("Fetching weather data");
        let url = format!("{}{}{}", BASE_URL, FETCH_WEATHER_DATA_BY_DATE_ENDPOINT, date);
        let parsed_data: Value = serde_json::from_str(get(url).unwrap().text().unwrap().as_str()).unwrap();

        let mut app_data = ApplicationData::new();

        for value in parsed_data.as_array().unwrap()
        {
            let time = datetime::parse_time(&value["created_on"].to_string().replace("\"", ""));

            app_data.temperature.push_back((DurationWrapper(time), value["temperature"].as_f64().unwrap() as f32));
            app_data.humidity.push_back((DurationWrapper(time), value["humidity"].as_f64().unwrap() as f32));
        }

        println!("HEAD Date: {}", app_data.temperature.head().unwrap().0.0);
        println!("BACK Date: {}", app_data.temperature.back().unwrap().0.0);

        sink.submit_command(FETCHED_WEATHER_DATA, app_data, Target::Auto).expect("Failed to submit weather data command");
    });
}

pub fn get_dates() -> Vec<(String, String)>
{
    println!("Fetching dates");
    let url = format!("{}{}", BASE_URL, FETCH_DATES_ENDPOINT);
    let parsed_data: Value = serde_json::from_str(get(url).unwrap().text().unwrap().as_str()).unwrap();

    let mut dates: Vec<(String, String)> = Vec::new();
    let mut parsed_date: NaiveDate;

    for value in parsed_data.as_array().unwrap()
    {
        parsed_date = NaiveDate::parse_from_str(value.as_str().unwrap(), "%Y%m%d").unwrap();
        dates.push((parsed_date.format("%m/%d/%Y").to_string(), value.to_string().replace("\"", "")));
    }

    dates
}

pub fn get_latest_date() -> String
{
    println!("Fetching latest date");
    let url = format!("{}{}{}", BASE_URL, FETCH_DATES_ENDPOINT, FETCH_LATEST_DATE_ENDPOINT);
    let parsed_data: Value = serde_json::from_str(get(url).unwrap().text().unwrap().as_str()).unwrap();

    parsed_data.to_string().replace("\"", "")
}