use gtk4::prelude::*;
use gtk4::{Box, Button, Label};

use crate::ui::texts::{GENERATED_PASSWORD_PLACEHOLDER, GENERATED_PASSWORD_TITLE};
use crate::ui::widgets::label_ui;

pub struct GeneratedPasswordComponent {
    pub root: Box,
    pub value_label: Label,
    pub copy_button: Button,
}

pub fn build() -> GeneratedPasswordComponent {
    let header = label_ui(GENERATED_PASSWORD_TITLE, 0.5, Some("generated-password-header-text"));
    let value_label = label_ui(
        GENERATED_PASSWORD_PLACEHOLDER,
        0.5,
        Some("generated-password-text"),
    );
    value_label.set_hexpand(true);
    value_label.add_css_class("generated-password-text");

    let copy_button = Button::with_label("Copiar");
    copy_button.add_css_class("copy-button");

    let row = Box::new(gtk4::Orientation::Horizontal, 10);
    row.set_hexpand(true);
    row.append(&value_label);
    row.append(&copy_button);

    let root = Box::new(gtk4::Orientation::Vertical, 0);
    root.set_hexpand(true);
    root.add_css_class("box-generated-password");
    root.append(&header);
    root.append(&row);

    GeneratedPasswordComponent { root, value_label, copy_button }
}
