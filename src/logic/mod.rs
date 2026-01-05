mod password;

use gtk4::prelude::*;

use crate::ui::state::SharedState;

pub fn connect_logic(state: SharedState) {
    let state_clone = state.clone();

    state.borrow().generate_button.connect_clicked(move |_| {
        let state = state_clone.borrow();

        let password = password::generate_password(
            state.uppercase_check.is_active(),
            state.lowercase_check.is_active(),
            state.numbers_check.is_active(),
            state.symbols_check.is_active(),
        );

        state.generated_password_text.set_text(&password);
    });
}
