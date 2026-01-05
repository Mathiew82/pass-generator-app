use gtk4::prelude::*;
use gtk4::{Box, Label};

use crate::ui::widgets::label_ui;
use crate::ui::texts::{GENERATED_PASSWORD_TITLE, GENERATED_PASSWORD_PLACEHOLDER};

pub struct GeneratedPasswordComponent {
    pub root: Box,
    pub value_label: Label,
}

pub fn build() -> GeneratedPasswordComponent {
    let header = label_ui(GENERATED_PASSWORD_TITLE, 0.5, Some("generated-password-header-text"));
    let value_label = label_ui(
        GENERATED_PASSWORD_PLACEHOLDER,
        0.5,
        Some("generated-password-text"),
    );

    let root = Box::new(gtk4::Orientation::Vertical, 0);
    root.set_hexpand(true);
    root.add_css_class("box-generated-password");
    root.append(&header);
    root.append(&value_label);

    GeneratedPasswordComponent { root, value_label }
}
