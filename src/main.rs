use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, CheckButton, CssProvider};

fn main() {
    let app = Application::builder()
        .application_id("com.mathiew82.pass-generator-app")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn display_ui() {
    let css = r#"
        .box-container {
            background-color: #1b2531;
            font-size: 16px;
            padding: 20px;
        }
        .options-wrapper {
            margin: 20px 0;
        }
        .check {
            color: #f0f0f0;
        }
        button.generate-button {
            background-color: #2f66a2;
            background-image: none;
            border: 2px solid #5f96d2;
            border-radius: 8px;
            color: #f0f0f0;
            font-size: 16px;
            padding: 10px;
            transition: background-color 0.2s;
        }
        button.generate-button:hover {
            background-color: #3f76b2;
        }
    "#;

    let css_provider = CssProvider::new();
    css_provider.load_from_data(css);

    let display = Display::default().expect("No display");
    gtk4::style_context_add_provider_for_display(
        &display,
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn check_ui(label: Option<&str>) -> CheckButton {
    let uppercase_check = CheckButton::new();
    uppercase_check.set_label(label);
    uppercase_check.add_css_class("check");
    uppercase_check
}

fn generate_button_ui() -> Button {
    let button = Button::with_label("Generar");
    button.set_hexpand(true);
    button.add_css_class("generate-button");
    button
}

fn options_left_column_ui() -> Box {
    let options_left_column = Box::new(gtk4::Orientation::Vertical, 0);
    options_left_column.set_hexpand(true);
    options_left_column.add_css_class("options-left-column");
    options_left_column
}

fn options_right_column_ui() -> Box {
    let options_right_column = Box::new(gtk4::Orientation::Vertical, 0);
    options_right_column.set_hexpand(true);
    options_right_column.add_css_class("options-right-column");
    options_right_column
}

fn options_wrapper_ui() -> Box {
    let options_wrapper = Box::new(gtk4::Orientation::Horizontal, 0);
    options_wrapper.set_hexpand(true);
    options_wrapper.add_css_class("options-wrapper");
    options_wrapper
}

fn box_container_ui() -> Box {
    let box_container = Box::new(gtk4::Orientation::Vertical, 0);
    box_container.set_hexpand(true);
    box_container.add_css_class("box-container");
    box_container
}

fn build_ui(app: &Application) {
    // === Display ===
    display_ui();
    // ===============

    // === Options ===
    let uppercase_check = check_ui(Some("Mayúsculas"));
    let lowercase_check = check_ui(Some("Minúsculas"));
    let numbers_check = check_ui(Some("Números"));
    // numbers_check.set_halign(gtk4::Align::End);
    let symbols_check = check_ui(Some("Símbolos"));
    // ===============

    // === Options Wrapper ===
    let options_left_column = options_left_column_ui();
    options_left_column.append(&uppercase_check);
    options_left_column.append(&lowercase_check);
    let options_right_column = options_right_column_ui();
    options_right_column.append(&numbers_check);
    options_right_column.append(&symbols_check);
    let options_wrapper = options_wrapper_ui();
    options_wrapper.append(&options_left_column);
    options_wrapper.append(&options_right_column);
    // =======================

    // === Generate Button ===
    let generate_button = generate_button_ui();
    // =======================

    // === Box Container ===
    let box_container = box_container_ui();
    box_container.append(&options_wrapper);
    box_container.append(&generate_button);
    // =====================

    // === Window ===
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pass Generator App")
        .default_width(500)
        .default_height(500)
        .child(&box_container)
        .build();
    window.show();
    // ==============
}
