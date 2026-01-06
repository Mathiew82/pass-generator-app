use gtk4::prelude::*;
use gtk4::{Box, Label};

use crate::ui::widgets::label_ui;
use crate::ui::texts::{SECURITY_FEEDBACK_HEADER, PLACEHOLDER_SECURITY};

pub struct SecurityFeedbackComponent {
    pub root: Box,
    pub value_label: Label,
}

pub fn build() -> SecurityFeedbackComponent {
    let header = label_ui(SECURITY_FEEDBACK_HEADER, 0.5, Some("security-feedback-header-text"));
    let value_label = label_ui(
        PLACEHOLDER_SECURITY,
        0.5,
        Some("security-feedback-text"),
    );

    let root = Box::new(gtk4::Orientation::Vertical, 0);
    root.set_hexpand(true);
    root.add_css_class("box-security-feedback");
    root.append(&header);
    root.append(&value_label);

    SecurityFeedbackComponent { root, value_label }
}
