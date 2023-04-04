use std::thread;
use std::sync::{Arc, Mutex};

use druid::{widget::{Flex, Label, Button}, AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use plotters::prelude::*;
use plotters_druid::Plot;

use reqwest::{blocking::get, Error};
use serde_json::Value;

#[derive(Clone, Data, Default, Lens)]
struct AppData
{
    datetime: Arc<Mutex<Vec<String>>>,
    temperature: Arc<Mutex<Vec<f64>>>,
    humidity: Arc<Mutex<Vec<f64>>>,
}

impl AppData {
    fn new() -> Self
    {
        AppData
        {
            datetime: Arc::new(Mutex::new(vec![])),
            temperature: Arc::new(Mutex::new(vec![])),
            humidity: Arc::new(Mutex::new(vec![])),
        }
    }

    fn populate_data(&mut self)
    {
        let datetime_data_ref = Arc::clone(&self.datetime);
        let temperature_data_ref = Arc::clone(&self.temperature);
        let humidity_data_ref = Arc::clone(&self.humidity);

        thread::spawn(move || {
            let parsed_weather_data: Value = serde_json::from_str(get_weather_data().unwrap().as_str()).unwrap();

            println!("Received weather data: {}", parsed_weather_data);

            let array = parsed_weather_data.as_array().unwrap();

            let mut datetime_data = datetime_data_ref.lock().unwrap();
            let mut temperature_data = temperature_data_ref.lock().unwrap();
            let mut humidity_data = humidity_data_ref.lock().unwrap();

            datetime_data.clear();
            temperature_data.clear();
            humidity_data.clear();

            for object in array
            {
                datetime_data.push(object["datetime"].to_string());
                temperature_data.push(object["temperature"].as_f64().unwrap());
                humidity_data.push(object["humidity"].as_f64().unwrap());
            }

            println!("Finished populating weather data");
        });
    }
}

fn get_weather_data() -> Result<String, Error>
{
    let weather_data = get("http://localhost:3500/get")?.text()?;
    Ok(weather_data)
}

/* GUI components */
fn build_plot_widget() -> impl Widget<AppData>
{
    Plot::new(|_, _, root| {
        let font = FontDesc::new(FontFamily::SansSerif, 16., FontStyle::Normal);

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(10)
            .build_cartesian_2d(0.0..1_f32, 0.0..6_f32)
            .unwrap();

        chart
            .configure_mesh()
            .axis_style(&RGBColor(28, 28, 28))
            .x_label_style(font.clone().with_color(&WHITE))
            .y_label_style(font.clone().with_color(&WHITE))
            .draw()
            .unwrap();

        // @TODO: Two series plots will be needed - temperature and humidity. Currently, dummy data are shown.
        chart
            .draw_series(LineSeries::new(
                (0..100).map(|x| x as f32).map(|x| (x, x * x)),
                &RED,
            ))
            .unwrap()
            .label("Weather data")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .background_style(&RGBColor(41, 41, 41))
            .border_style(&RGBColor(28, 28, 28))
            .label_font(font.with_color(&WHITE))
            .draw()
            .unwrap();
    })
}

fn build_refresh_button() -> impl Widget<AppData>
{
    Flex::column()
        .with_child(
            Button::new("Refresh")
                .padding(5.0)
                .on_click(|_, data: &mut AppData, _| {
                    data.populate_data();
                }))
}

fn build_application() -> impl Widget<AppData>
{
    Flex::column()
        .with_child(Label::new("Weather data"))
        .with_spacer(5.0)
        .with_flex_child(build_plot_widget(), 1.)
        .with_spacer(5.0)
        .with_child(build_refresh_button())
        .padding(10.0)
}

fn main()
{
    println!("starting frontend application");

    let main_window = WindowDesc::new(build_application())
        .title("Super-Duper Weather Visualization")
        .window_size((600.0, 600.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppData::new())
        .expect("Failed to launch application");
}
