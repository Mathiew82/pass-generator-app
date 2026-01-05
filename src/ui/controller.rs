use gtk4::prelude::*;

use crate::logic::password;
use crate::logic::state::PasswordOptions;

pub fn connect_generate_button(
    generate_button: &gtk4::Button,
    uppercase_check: &gtk4::CheckButton,
    lowercase_check: &gtk4::CheckButton,
    numbers_check: &gtk4::CheckButton,
    symbols_check: &gtk4::CheckButton,
    output_label: &gtk4::Label,
) {
    let generate_button = generate_button.clone();
    let uppercase_check = uppercase_check.clone();
    let lowercase_check = lowercase_check.clone();
    let numbers_check = numbers_check.clone();
    let symbols_check = symbols_check.clone();
    let output_label = output_label.clone();

    generate_button.connect_clicked(move |_| {
        let opts = PasswordOptions {
            length: 16,
            uppercase: uppercase_check.is_active(),
            lowercase: lowercase_check.is_active(),
            numbers: numbers_check.is_active(),
            symbols: symbols_check.is_active(),
        };

        let password = password::generate_password(opts);
        output_label.set_text(&password);
    });
}
