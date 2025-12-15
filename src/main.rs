use gtk4::gdk::Display;
use gtk4::{Align, prelude::*};
use gtk4::{Application, ApplicationWindow, Box, Button, CheckButton, CssProvider, Image, Label};

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
            padding: 10px 20px 20px 20px;
        }
        .logo {
            border: 6px solid rgba(0, 0, 0, 0.2);
            border-radius: 50%;
            min-width: 100px;
            min-height: 100px;
            margin-bottom: 10px;
        }
        .options-wrapper {
            margin: 20px 0;
        }
        .label {
            font-weight: bold;
        }
        .generated-password-header-text {
            color: #f0f0f0;
        }
        .generated-password-text {
            background-color: #0b1521;
            border: 1px solid #2f66a2;
            color: #aaaaaa;
            font-weight: normal;
            margin-top: 5px;
            padding: 10px;
        }
        .check {
            color: #f0f0f0;
        }
        button.generate-button {
            background-color: #2f66a2;
            background-image: none;
            border: none;
            border-radius: 0px;
            color: #f0f0f0;
            font-size: 16px;
            font-weight: bold;
            padding: 10px;
            transition: background-color 0.1s;
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

fn image_ui(path: &str, halign: Align, hexpand: bool) -> Image {
    let image = Image::from_file(path);
    image.set_halign(halign);
    image.set_hexpand(hexpand);
    image.add_css_class("logo");
    image
}

fn label_ui(text: &str, xalign: f32, css_class: Option<&str>) -> Label {
    let label = Label::new(Some(text));
    label.set_hexpand(true);
    label.set_halign(Align::Fill);
    label.set_xalign(xalign);
    label.add_css_class("label");
    if let Some(class) = css_class {
        label.add_css_class(class);
    }
    label
}

fn box_header_ui() -> Box {
    let box_header = Box::new(gtk4::Orientation::Vertical, 0);
    box_header.set_hexpand(true);
    box_header.add_css_class("box-header");
    box_header
}

fn box_generated_password_ui() -> Box {
    let box_generated_password = Box::new(gtk4::Orientation::Vertical, 0);
    box_generated_password.set_hexpand(true);
    box_generated_password.add_css_class("box-generated-password");
    box_generated_password
}

fn check_ui(label: &str) -> CheckButton {
    let uppercase_check = CheckButton::new();
    uppercase_check.set_label(Some(label));
    uppercase_check.add_css_class("check");
    uppercase_check
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
    // === Display ===
    display_ui();
    // ===============

    // === Logo ===
    let logo = image_ui("assets/logo.png", Align::Center, true);
    // ============

    // === Box Header ===
    let box_header = box_header_ui();
    box_header.append(&logo);
    // ==================

    // === Header Text ===
    let box_header_text = label_ui(
        "Contraseña generada",
        0.5,
        Some("generated-password-header-text"),
    );
    // ===================

    // === Generated Password Text ===
    let generated_password_text = label_ui(
        "LNDJ7zyDf8Q86RP+x=AJFu8bH$VsdsAA",
        0.5,
        Some("generated-password-text"),
    );
    // ===================

    // === Box Generated Password ===
    let box_generated_password = box_generated_password_ui();
    box_generated_password.append(&box_header_text);
    box_generated_password.append(&generated_password_text);
    // ==============================

    // === Check Options ===
    let uppercase_check = check_ui("Mayúsculas");
    let lowercase_check = check_ui("Minúsculas");
    let numbers_check = check_ui("Números");
    let symbols_check = check_ui("Símbolos");
    uppercase_check.set_active(true);
    lowercase_check.set_active(true);
    numbers_check.set_active(true);
    // =====================

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
    box_container.append(&box_header);
    box_container.append(&box_generated_password);
    box_container.append(&options_wrapper);
    box_container.append(&generate_button);
    // =====================

    // === Window ===
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pass Generator App")
        .default_width(500)
        .resizable(false)
        .child(&box_container)
        .build();
    window.show();
    // ==============
}
