pub mod state;
mod password;

use gtk4::prelude::*;

use crate::logic::state::PasswordOptions;
use crate::ui::state::SharedState;

pub fn connect_logic(state: SharedState) {
    let state_clone = state.clone();

    state.borrow().generate_button.connect_clicked(move |_| {
        let state = state_clone.borrow();

        let opts = PasswordOptions {
            uppercase: state.uppercase_check.is_active(),
            lowercase: state.lowercase_check.is_active(),
            numbers: state.numbers_check.is_active(),
            symbols: state.symbols_check.is_active(),
        };

        let password = password::generate_password(opts);

        state.generated_password_text.set_text(&password);
    });
}
