pub fn generate_password(uppercase: bool, lowercase: bool, numbers: bool, symbols: bool) -> String {
    println!("uppercase: {}", uppercase);
    println!("lowercase: {}", lowercase);
    println!("numbers: {}", numbers);
    println!("symbols: {}", symbols);
    "Ejemplo123!".to_string()
}
