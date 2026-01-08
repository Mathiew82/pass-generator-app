fn main() {
    if cfg!(target_os = "windows") {
        winresource::WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()
            .unwrap();
    }
}
