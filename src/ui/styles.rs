use std::sync::OnceLock;
use gtk4::CssProvider;
use gtk4::gdk::Display;

static CSS_LOADED: OnceLock<()> = OnceLock::new();

pub fn display_ui() {
    CSS_LOADED.get_or_init(|| {
        let css = r#"
            .box-container {
                background-color: #1b2531;
                font-size: 16px;
                padding: 10px 20px 20px 20px;
            }
            .logo {
                border: 6px solid rgba(0, 0, 0, 0.2);
                border-radius: 50%;
                min-width: 100px;
                min-height: 100px;
                margin-bottom: 10px;
            }
            .options-wrapper {
                margin: 20px 0;
            }
            .label {
                font-weight: bold;
            }
            .generated-password-header-text {
                color: #f0f0f0;
            }
            .generated-password-text {
                background-color: #0b1521;
                border: 1px solid #2f66a2;
                color: #aaaaaa;
                font-weight: normal;
                margin-top: 5px;
                padding: 10px;
            }
            .check {
                color: #f0f0f0;
            }
            button.generate-button {
                background-color: #2f66a2;
                background-image: none;
                border: none;
                border-radius: 0px;
                color: #f0f0f0;
                font-size: 16px;
                font-weight: bold;
                padding: 10px;
                transition: background-color 0.1s;
            }
            button.generate-button:hover {
                background-color: #3f76b2;
            }
        "#;

        let css_provider = CssProvider::new();
        css_provider.load_from_data(css);

        let display = Display::default().expect("No display");
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
}
