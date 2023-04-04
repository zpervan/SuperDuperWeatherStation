use crate::core::ApplicationData;
use druid::{Widget, WidgetExt};
use druid::widget::{Flex, Label, Button};
use plotters::prelude::*;
use plotters_druid::Plot;

pub fn build_gui() -> impl Widget<ApplicationData>
{
    Flex::column()
        .with_child(Label::new("Weather data"))
        .with_spacer(5.0)
        .with_flex_child(build_plot_widget(), 1.)
        .with_spacer(5.0)
        .with_child(build_refresh_button())
        .padding(10.0)
}

fn build_plot_widget() -> impl Widget<ApplicationData>
{
    Plot::new(|_, data: &ApplicationData, root| {
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(10)
            .margin_left(10)
            .margin_top(20)
            .build_cartesian_2d(0.0..data.temperature.len() as f32, 18.0f32..26.0f32)
            .unwrap();

        let font = FontDesc::new(FontFamily::SansSerif, 16., FontStyle::Normal);

        chart
            .configure_mesh()
            .axis_style(&RGBColor(28, 28, 28))
            .x_label_style(font.clone().with_color(&WHITE))
            .y_label_style(font.clone().with_color(&WHITE))
            // .x_label_formatter(&|y| format!("{:02}:{:02}", y.num_minutes(), y.num_seconds() % 60))
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(data.temperature.clone(), &RED))
            .unwrap()
            .label("Temperature")
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

fn build_refresh_button() -> impl Widget<ApplicationData>
{
    Flex::column()
        .with_child(
            Button::new("Refresh")
                .padding(5.0)
                .on_click(|ctx, data: &mut ApplicationData, _| {
                    data.populate_data(ctx.get_external_handle());
                }))
}

