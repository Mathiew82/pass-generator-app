use crate::logic::state::PasswordOptions;
use crate::ui::texts::{LOW_SECURITY, MEDIUM_SECURITY, HIGH_SECURITY};

pub fn display_info(opts: PasswordOptions) -> (&'static str, &'static str) {
    let check_options: [bool; 4] = [opts.lowercase, opts.uppercase, opts.numbers, opts.symbols];
    let selected_checks = check_options.into_iter().filter(|v| *v).count();

    if opts.length < 8 {
        (LOW_SECURITY, "security-low")
    } else if opts.length >= 8 && selected_checks < 3 {
        (MEDIUM_SECURITY, "security-medium")
    } else {
        (HIGH_SECURITY, "security-high")
    }
}
