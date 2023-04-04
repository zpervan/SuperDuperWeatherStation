mod core;
mod datetime;
mod gui;

use druid::{ AppLauncher, WindowDesc};
use crate::core::{ApplicationData, Delegate};

fn main()
{
    println!("starting frontend application");

    let main_window = WindowDesc::new(gui::build_gui())
        .title("Super-Duper Weather Visualization")
        .window_size((1200.0, 600.0));

    let app_data = ApplicationData::new();

    AppLauncher::with_window(main_window)
        .log_to_console()
        .delegate(Delegate{})
        .launch(app_data)
        .expect("Failed to launch application");
}
