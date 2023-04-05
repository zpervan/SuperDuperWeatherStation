use crate::core::ApplicationData;
use crate::util;
use druid::{Widget, WidgetExt};
use druid::widget::{Flex, Label, Button};
use plotters::prelude::*;
use plotters::prelude::full_palette::{GREY, LIGHTBLUE_400, RED_400};
use plotters_druid::Plot;

pub fn build_gui() -> impl Widget<ApplicationData>
{
    Flex::column()
        .with_child(Label::new("Weather data"))
        .with_spacer(5.0)
        .with_flex_child(
            Flex::row()
                .with_flex_child(build_temperature_plot_widget(), 1.)
                .with_spacer(5.0)
                .with_flex_child(build_humidity_plot_widget(), 1.), 1.)
        .with_spacer(5.0)
        .with_child(build_refresh_button())
        .padding(10.0)
}

// @TODO: Try to generalize the plot builders so we have a single one
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
            .margin_right(30)
            .margin_left(30)
            .margin_top(20)
            .build_cartesian_2d(time_from..time_to, min_temperature..max_temperature)
            .unwrap();

        let font = FontDesc::new(FontFamily::SansSerif, 18., FontStyle::Normal);

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
            .draw_series(AreaSeries::new(util::convert_to_duration(&data.temperature), 0.0, &RED_400.mix(0.75)))
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
            .margin_right(30)
            .margin_left(30)
            .margin_top(20)
            .build_cartesian_2d(time_from..time_to, min_humidity..max_humidity)
            .unwrap();

        let font = FontDesc::new(FontFamily::SansSerif, 18., FontStyle::Normal);

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
            .draw_series(AreaSeries::new(util::convert_to_duration(&data.humidity), 0.0, &LIGHTBLUE_400.mix(0.75)))
            .unwrap();
    })
}

fn build_refresh_button() -> impl Widget<ApplicationData>
{
    Flex::column()
        .with_child(
            Button::new("Refresh")
                .on_click(|ctx, data: &mut ApplicationData, _| {
                    data.populate_data(ctx.get_external_handle());
                }))
}

