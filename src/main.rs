use gtk4::Application;
use gtk4::prelude::*;

mod ui;

fn main() {
    let app = Application::builder()
        .application_id("com.mathiew82.pass-generator-app")
        .build();

    app.connect_activate(ui::layout::build_ui);
    app.run();
}
