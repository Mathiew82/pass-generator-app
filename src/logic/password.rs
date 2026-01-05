use crate::logic::state::PasswordOptions;

pub fn generate_password(opts: PasswordOptions) -> String {
    println!("uppercase: {}", opts.uppercase);
    println!("lowercase: {}", opts.lowercase);
    println!("numbers: {}", opts.numbers);
    println!("symbols: {}", opts.symbols);
    "Ejemplo123!".to_string()
}
