use crate::ui::state::{AppState, SharedState};
use crate::ui::styles::display_ui;
use crate::ui::widgets::*;
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ui(app: &Application) -> SharedState {
    display_ui();

    let logo = image_ui("assets/logo.png", Align::Center, true);

    let box_header = box_header_ui();
    box_header.append(&logo);

    let box_header_text = label_ui(
        "Contraseña generada",
        0.5,
        Some("generated-password-header-text"),
    );

    let generated_password_text = label_ui(
        "LNDJ7zyDf8Q86RP+x=AJFu8bH$VsdsAA",
        0.5,
        Some("generated-password-text"),
    );

    let box_generated_password = box_generated_password_ui();
    box_generated_password.append(&box_header_text);
    box_generated_password.append(&generated_password_text);

    let uppercase_check = check_ui("Mayúsculas");
    let lowercase_check = check_ui("Minúsculas");
    let numbers_check = check_ui("Números");
    let symbols_check = check_ui("Símbolos");
    uppercase_check.set_active(true);
    lowercase_check.set_active(true);
    numbers_check.set_active(true);

    let options_left_column = options_left_column_ui();
    options_left_column.append(&uppercase_check);
    options_left_column.append(&lowercase_check);
    let options_right_column = options_right_column_ui();
    options_right_column.append(&numbers_check);
    options_right_column.append(&symbols_check);
    let options_wrapper = options_wrapper_ui();
    options_wrapper.append(&options_left_column);
    options_wrapper.append(&options_right_column);

    let generate_button = generate_button_ui();

    let box_container = box_container_ui();
    box_container.append(&box_header);
    box_container.append(&box_generated_password);
    box_container.append(&options_wrapper);
    box_container.append(&generate_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pass Generator App")
        .default_width(500)
        .resizable(false)
        .child(&box_container)
        .build();
    window.show();

    Rc::new(RefCell::new(AppState {
        generate_button,
        uppercase_check,
        lowercase_check,
        numbers_check,
        symbols_check,
        generated_password_text,
    }))
}
