use crate::ui::state::{AppState, SharedState};
use crate::ui::styles::display_ui;
use crate::ui::widgets::*;
use crate::ui::texts::*;
use crate::ui::components::generated_password;
use crate::ui::components::options_panel;

use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow};

use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ui(app: &Application) -> SharedState {
    display_ui();

    let logo = image_ui("assets/logo.png", Align::Center, true);

    let box_header = box_header_ui();
    box_header.append(&logo);

    let generated = generated_password::build();
    let options = options_panel::build();

    let generate_button = generate_button_ui();

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

    Rc::new(RefCell::new(AppState {
        generate_button,
        uppercase_check: options.uppercase_check,
        lowercase_check: options.lowercase_check,
        numbers_check: options.numbers_check,
        symbols_check: options.symbols_check,
        generated_password_text: generated.value_label,
    }))
}
