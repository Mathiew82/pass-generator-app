use gtk4::prelude::*;
use gtk4::{Box, Label};

use crate::ui::widgets::label_ui;

pub struct SecurityFeedbackComponent {
    pub root: Box,
    pub value_label: Label,
}

pub fn build() -> SecurityFeedbackComponent {
    let value_label = label_ui(
        "",
        0.5,
        Some("security-feedback-text"),
    );
    value_label.add_css_class("security-feedback-text");

    let root = Box::new(gtk4::Orientation::Vertical, 0);
    root.set_hexpand(true);
    root.add_css_class("box-security-feedback");
    root.append(&value_label);

    SecurityFeedbackComponent { root, value_label }
}
