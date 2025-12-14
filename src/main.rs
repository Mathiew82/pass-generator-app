use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, CssProvider};

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
            padding: 20px;
        }
        button.generate-button {
            background-color: #2f66a2;
            background-image: none;
            border: 2px solid #5f96d2;
            border-radius: 8px;
            color: white;
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

fn generate_button_ui() -> Button {
    let button = Button::with_label("Generar");
    button.set_hexpand(true);
    button.add_css_class("generate-button");
    button
}

fn box_container_ui() -> Box {
    let box_container = Box::new(gtk4::Orientation::Vertical, 0);
    box_container.set_hexpand(true);
    box_container.add_css_class("box-container");
    box_container
}

fn build_ui(app: &Application) {
    display_ui();

    let generate_button = generate_button_ui();
    let box_container = box_container_ui();
    box_container.append(&generate_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pass Generator App")
        .default_width(500)
        .default_height(500)
        .child(&box_container)
        .build();
    window.show();
}
