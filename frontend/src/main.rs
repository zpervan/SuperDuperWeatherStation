use druid::{widget::{Flex, Label, Button}, AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use druid::im::Vector;
use plotters::prelude::*;
use plotters_druid::Plot;
use reqwest::Error;
use serde_json::Value;

#[derive(Clone, Data, Default, Lens)]
struct AppData
{
    datetime: Vector<String>,
    temperature: Vector<f32>,
    humidity: Vector<f32>,
}

async fn get_weather_data() -> Result<String, Error>
{
    let weather_data = reqwest::get("http://localhost:3500/get").await?.text().await?;
    let json: Value = serde_json::from_str(weather_data.as_str()).unwrap();
    let array = json.as_array().unwrap();

    for object in array
    {
        println!("Datetime: {}", object["datetime"].as_str().unwrap());
        println!("Temperature: {}", object["temperature"].as_str().unwrap());
        println!("Humidity: {}", object["humidity"].as_str().unwrap());
    }

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
        .with_child(Button::new("Refresh").padding(5.0).on_click(|_, _, _| {
            tokio::spawn(async move {
                match get_weather_data().await {
                    Ok(data) => println!("{}", data),
                    Err(e) => println!("Error: {}", e)
                }
            }
            );
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

#[tokio::main]
async fn main()
{
    println!("starting frontend application");

    let main_window = WindowDesc::new(build_application())
        .title("Super-Duper Weather Visualization")
        .window_size((600.0, 600.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppData::default())
        .expect("Failed to launch application");
}
