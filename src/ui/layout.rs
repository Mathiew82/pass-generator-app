use crate::ui::styles::display_ui;
use crate::ui::widgets::*;
use crate::ui::texts::*;
use crate::ui::controller;
use crate::ui::components::generated_password;
use crate::ui::components::options_panel;

use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow};

pub fn build_ui(app: &Application) {
    display_ui();

    let logo = image_ui("assets/logo.png", Align::Center, true);

    let box_header = box_header_ui();
    box_header.append(&logo);

    let generate_button = generate_button_ui();
    let options = options_panel::build();
    let generated = generated_password::build();

    controller::connect_generate_button(
        &generate_button,
        &options.length_spin,
        &options.uppercase_check,
        &options.lowercase_check,
        &options.numbers_check,
        &options.symbols_check,
        &generated.value_label,
    );

    let box_container = box_container_ui();
    box_container.append(&box_header);
    box_container.append(&generated.root);
    box_container.append(&options.root);
    box_container.append(&generate_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_TITLE)
        .default_width(500)
        .resizable(false)
        .child(&box_container)
        .build();
    window.show();
}
