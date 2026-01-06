use gtk4::prelude::*;

use crate::logic::password;
use crate::logic::feedback;
use crate::logic::state::PasswordOptions;

pub fn connect_generate_button(
    generate_button: &gtk4::Button,
    length_spin: &gtk4::SpinButton,
    uppercase_check: &gtk4::CheckButton,
    lowercase_check: &gtk4::CheckButton,
    numbers_check: &gtk4::CheckButton,
    symbols_check: &gtk4::CheckButton,
    output_label: &gtk4::Label,
    security_feedback_label: &gtk4::Label,
) {
    let generate_button = generate_button.clone();
    let uppercase_check = uppercase_check.clone();
    let lowercase_check = lowercase_check.clone();
    let numbers_check = numbers_check.clone();
    let symbols_check = symbols_check.clone();
    let output_label = output_label.clone();
    let security_feedback_label = security_feedback_label.clone();
    let length_spin = length_spin.clone();

    generate_button.connect_clicked(move |_| {
        let opts = PasswordOptions {
            length: length_spin.value() as usize,
            uppercase: uppercase_check.is_active(),
            lowercase: lowercase_check.is_active(),
            numbers: numbers_check.is_active(),
            symbols: symbols_check.is_active(),
        };

        let password = password::generate_password(opts);
        output_label.set_text(&password);

        let (text, class) = feedback::display_info(opts);
        security_feedback_label.set_text(text);

        security_feedback_label.remove_css_class("security-low");
        security_feedback_label.remove_css_class("security-medium");
        security_feedback_label.remove_css_class("security-high");
        security_feedback_label.add_css_class(class);
    });
}
