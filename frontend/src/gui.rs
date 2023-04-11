use druid::{FontDescriptor, Widget, WidgetExt};
use druid::widget::{Flex, Label, Button};
use druid_widget_nursery::DropdownSelect;
use plotters::prelude::*;
use plotters::prelude::full_palette::{GREY, LIGHTBLUE_400, RED_400};
use plotters_druid::Plot;

use crate::core::ApplicationData;
use crate::{datetime, util, requests};

const MARGIN: i32 = 30;
const SPACE: f64 = 5.0;

pub fn build_gui() -> impl Widget<ApplicationData>
{
    Flex::column()
        .with_child(Label::new("Weather data"))
        .with_spacer(SPACE)
        .with_flex_child(
            Flex::row()
                .with_flex_child(build_temperature_plot_widget(), 1.0)
                .with_spacer(SPACE)
                .with_flex_child(build_humidity_plot_widget(), 1.0), 1.0,
        )
        .with_spacer(SPACE)
        .with_child(
            Flex::row()
                .with_child(build_refresh_button())
                .with_spacer(SPACE)
                .with_child(build_date_dropdown())
        )
        .with_spacer(SPACE)
}

// @TODO: Try to generalize the plot builder functions so we have a single one
fn build_temperature_plot_widget() -> impl Widget<ApplicationData>
{
    Plot::new(|_, data: &ApplicationData, root| {
        if data.temperature.is_empty()
        {
            return;
        }

        let (min_temperature, max_temperature) = util::find_extremes(&data.temperature);
        let time_from = data.temperature.head().unwrap().0.0;
        let time_to = data.temperature.back().unwrap().0.0;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(50)
            .y_label_area_size(70)
            .margin(MARGIN)
            .build_cartesian_2d(time_from..time_to, min_temperature..max_temperature)
            .unwrap();

        let font = FontDesc::new(plotters::prelude::FontFamily::SansSerif, 18., FontStyle::Normal);

        chart
            .configure_mesh()
            .x_desc("Time[hh:mm]")
            .y_desc("Temperature[°C]")
            .axis_style(&RGBColor(28, 28, 28))
            .light_line_style(&GREY.mix(0.1))
            .bold_line_style(&GREY.mix(0.3))
            .x_label_style(font.clone().with_color(&WHITE))
            .y_label_style(font.clone().with_color(&WHITE))
            .x_label_formatter(&|y| format!("{:02}:{:02}", y.num_hours(), y.num_minutes() % 60))
            .draw()
            .unwrap();

        chart
            .draw_series(AreaSeries::new(datetime::convert_to_duration(&data.temperature), 0.0, &RED_400.mix(0.75)))
            .unwrap()
            .label("Temperature")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED_400));
    })
}

fn build_humidity_plot_widget() -> impl Widget<ApplicationData>
{
    Plot::new(|_, data: &ApplicationData, root| {
        if data.humidity.is_empty()
        {
            return;
        }

        let (min_humidity, max_humidity) = util::find_extremes(&data.humidity);
        let time_from = data.humidity.head().unwrap().0.0;
        let time_to = data.humidity.back().unwrap().0.0;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(50)
            .y_label_area_size(70)
            .margin(MARGIN)
            .build_cartesian_2d(time_from..time_to, min_humidity..max_humidity)
            .unwrap();

        let font = FontDesc::new(plotters::prelude::FontFamily::SansSerif, 18., FontStyle::Normal);

        chart
            .configure_mesh()
            .x_desc("Time[hh:mm]")
            .y_desc("Relative Humidity[%]")
            .axis_style(&RGBColor(28, 28, 28))
            .light_line_style(&GREY.mix(0.1))
            .bold_line_style(&GREY.mix(0.3))
            .x_label_style(font.clone().with_color(&WHITE))
            .y_label_style(font.clone().with_color(&WHITE))
            .x_label_formatter(&|y| format!("{:02}:{:02}", y.num_hours(), y.num_minutes() % 60))
            .draw()
            .unwrap();

        chart
            .draw_series(AreaSeries::new(datetime::convert_to_duration(&data.humidity), 0.0, &LIGHTBLUE_400.mix(0.75)))
            .unwrap();
    })
}

fn build_refresh_button() -> impl Widget<ApplicationData>
{
    let font = FontDescriptor::new(druid::FontFamily::SANS_SERIF).with_size(40.0);
    Button::from_label(Label::new("⭯").with_font(font))
        .on_click(|ctx, data: &mut ApplicationData, _| {
            requests::get_weather_data(ctx.get_external_handle(), data.current_date.clone());
        })
}

fn build_date_dropdown() -> impl Widget<ApplicationData>
{
    DropdownSelect::new(requests::get_dates())
        .align_right()
        .lens(ApplicationData::current_date)
}

