use crate::logic::state::PasswordOptions;
use crate::ui::texts::{HIGH_SECURITY, LOW_SECURITY, MEDIUM_SECURITY};

pub fn display_info(opts: PasswordOptions) -> (&'static str, &'static str) {
    let mut score = 0;
    if opts.length >= 8 {
        score += 1;
    }
    if opts.length >= 12 {
        score += 1;
    }
    if opts.lowercase {
        score += 1;
    }
    if opts.uppercase {
        score += 1;
    }
    if opts.numbers {
        score += 1;
    }
    if opts.symbols {
        score += 1;
    }

    if score <= 2 {
        (LOW_SECURITY, "security-low")
    } else if score <= 4 {
        (MEDIUM_SECURITY, "security-medium")
    } else {
        (HIGH_SECURITY, "security-high")
    }
}
