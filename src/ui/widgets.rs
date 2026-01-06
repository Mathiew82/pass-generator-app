use gtk4::prelude::*;
use gtk4::{Adjustment, Align, Box, Button, CheckButton, Image, Label, SpinButton};
use gtk4::gdk_pixbuf::PixbufLoader;

use crate::ui::texts::*;

pub fn image_ui(halign: Align, hexpand: bool) -> Image {
    let bytes = include_bytes!("../../assets/logo.png");

    let loader = PixbufLoader::new();
    loader.write(bytes).expect("Failed to read logo bytes");
    loader.close().expect("Failed to close PixbufLoader");

    let pixbuf = loader.pixbuf().expect("Failed to decode logo");
    let image = Image::from_pixbuf(Some(&pixbuf));

    image.set_halign(halign);
    image.set_hexpand(hexpand);
    image.add_css_class("logo");
    image
}

pub fn label_ui(text: &str, xalign: f32, css_class: Option<&str>) -> Label {
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

pub fn box_header_ui() -> Box {
    let box_header = Box::new(gtk4::Orientation::Vertical, 0);
    box_header.set_hexpand(true);
    box_header.add_css_class("box-header");
    box_header
}

pub fn check_ui(label: &str) -> CheckButton {
    let uppercase_check = CheckButton::new();
    uppercase_check.set_label(Some(label));
    uppercase_check.add_css_class("check");
    uppercase_check
}

pub fn length_spin_ui() -> SpinButton {
    let adj = Adjustment::new(16.0, 4.0, 32.0, 1.0, 1.0, 0.0);
    let spin = SpinButton::new(Some(&adj), 1.0, 0);
    spin.set_hexpand(true);
    spin
}

pub fn options_left_column_ui() -> Box {
    let options_left_column = Box::new(gtk4::Orientation::Vertical, 0);
    options_left_column.set_hexpand(true);
    options_left_column.add_css_class("options-left-column");
    options_left_column
}

pub fn options_right_column_ui() -> Box {
    let options_right_column = Box::new(gtk4::Orientation::Vertical, 0);
    options_right_column.set_hexpand(true);
    options_right_column.add_css_class("options-right-column");
    options_right_column
}

pub fn options_wrapper_ui() -> Box {
    let options_wrapper = Box::new(gtk4::Orientation::Horizontal, 0);
    options_wrapper.set_hexpand(true);
    options_wrapper.add_css_class("options-wrapper");
    options_wrapper
}

pub fn generate_button_ui() -> Button {
    let button = Button::with_label(GENERATE_BUTTON_LABEL);
    button.set_hexpand(true);
    button.add_css_class("generate-button");
    button
}

pub fn box_container_ui() -> Box {
    let box_container = Box::new(gtk4::Orientation::Vertical, 0);
    box_container.set_hexpand(true);
    box_container.add_css_class("box-container");
    box_container
}
