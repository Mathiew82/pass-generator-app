mod logic;
mod ui;

use gtk4::Application;
use gtk4::prelude::*;

fn main() {
    let app = Application::builder()
        .application_id("com.mathiew82.pass-generator-app")
        .build();

    app.connect_activate(|app| {
        ui::layout::build_ui(app);
    });

    app.run();
}
