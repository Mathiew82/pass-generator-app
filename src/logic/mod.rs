use crate::ui::state::SharedState;
use gtk4::prelude::*;

pub fn connect_logic(state: SharedState) {
    let state_clone = state.clone();

    state.borrow().generate_button.connect_clicked(move |_| {
        let state = state_clone.borrow();

        let password = generate_password(
            state.uppercase_check.is_active(),
            state.lowercase_check.is_active(),
            state.numbers_check.is_active(),
            state.symbols_check.is_active(),
        );

        state.generated_password_text.set_text(&password);
    });
}

fn generate_password(uppercase: bool, lowercase: bool, numbers: bool, symbols: bool) -> String {
    println!("uppercase: {}", uppercase);
    println!("lowercase: {}", lowercase);
    println!("numbers: {}", numbers);
    println!("symbols: {}", symbols);
    "Ejemplo123!".to_string()
}
