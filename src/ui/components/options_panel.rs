use gtk4::prelude::*;
use gtk4::{Box, CheckButton};

use crate::ui::texts::{CHECK_LOWERCASE, CHECK_NUMBERS, CHECK_SYMBOLS, CHECK_UPPERCASE};
use crate::ui::widgets::{
    check_ui, options_left_column_ui, options_right_column_ui, options_wrapper_ui,
};

pub struct OptionsPanel {
    pub root: Box,
    pub uppercase_check: CheckButton,
    pub lowercase_check: CheckButton,
    pub numbers_check: CheckButton,
    pub symbols_check: CheckButton,
}

pub fn build() -> OptionsPanel {
    let uppercase_check = check_ui(CHECK_UPPERCASE);
    let lowercase_check = check_ui(CHECK_LOWERCASE);
    let numbers_check = check_ui(CHECK_NUMBERS);
    let symbols_check = check_ui(CHECK_SYMBOLS);

    uppercase_check.set_active(true);
    lowercase_check.set_active(true);
    numbers_check.set_active(true);

    let left = options_left_column_ui();
    left.append(&uppercase_check);
    left.append(&lowercase_check);

    let right = options_right_column_ui();
    right.append(&numbers_check);
    right.append(&symbols_check);

    let root = options_wrapper_ui();
    root.append(&left);
    root.append(&right);

    OptionsPanel {
        root,
        uppercase_check,
        lowercase_check,
        numbers_check,
        symbols_check,
    }
}
