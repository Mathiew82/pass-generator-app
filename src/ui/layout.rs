use crate::ui::styles::display_ui;
use crate::ui::widgets::*;
use crate::ui::texts::*;
use crate::ui::components::generated_password;
use crate::ui::components::options_panel;
use crate::logic::password;
use crate::logic::state::PasswordOptions;

use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow};

pub fn build_ui(app: &Application) {
    display_ui();

    let logo = image_ui("assets/logo.png", Align::Center, true);

    let box_header = box_header_ui();
    box_header.append(&logo);

    let generate_button = generate_button_ui();
    let generated = generated_password::build();
    let options = options_panel::build();

    let uppercase_check = options.uppercase_check.clone();
    let lowercase_check = options.lowercase_check.clone();
    let numbers_check = options.numbers_check.clone();
    let symbols_check = options.symbols_check.clone();
    let output_label = generated.value_label.clone();

    generate_button.connect_clicked(move |_| {
        let opts = PasswordOptions {
            uppercase: uppercase_check.is_active(),
            lowercase: lowercase_check.is_active(),
            numbers: numbers_check.is_active(),
            symbols: symbols_check.is_active(),
        };

        let password = password::generate_password(opts);
        output_label.set_text(&password);
    });

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
