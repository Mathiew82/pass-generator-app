#[derive(Debug, Clone, Copy)]
pub struct PasswordOptions {
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}
