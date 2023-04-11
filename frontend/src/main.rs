mod core;
mod datetime;
mod gui;
mod util;
mod requests;

use druid::{AppLauncher, WindowDesc};
use crate::core::{ApplicationData, Delegate};

fn main()
{
    println!("Starting visualization application");

    let main_window = WindowDesc::new(gui::build_gui())
        .title("Super-Duper Weather Visualization")
        .window_size((1200.0, 600.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .delegate(Delegate {})
        .launch(ApplicationData::new())
        .expect("Failed to launch application");
}
