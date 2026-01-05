use gtk4::{Button, CheckButton, Label};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct AppState {
    pub generate_button: Button,
    pub uppercase_check: CheckButton,
    pub lowercase_check: CheckButton,
    pub numbers_check: CheckButton,
    pub symbols_check: CheckButton,
    pub generated_password_text: Label,
}

pub type SharedState = Rc<RefCell<AppState>>;
