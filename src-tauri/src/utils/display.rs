use std::borrow::Cow;
use tokio::join;

/// Injects the display styles and scripts into the head tag of the
/// provided `input` HTML
pub async fn inject_display_current(input: &str) -> String {
    let (display_script, display_styles) = join!(get_display_script(), get_display_styles());

    inject_display(input, &display_script, &display_styles)
}

/// Injects the display script and styles into the head tag of the
/// provided HTML `value`
fn inject_display(value: &str, display_script: &str, display_styles: &str) -> String {
    value.replace(
        "<head>",
        &format!("<head><script>{display_script}</script><style>{display_styles}</style>",),
    )
}

/// In release mode bake in the display script
#[cfg(not(debug_assertions))]
async fn get_display_script() -> Cow<'static, str> {
    Cow::Borrowed(include_str!("../../../display/dist/display.js"))
}

/// When debugging, load the display script directly from the file system
/// this allows updating it at runtime
#[cfg(debug_assertions)]
async fn get_display_script() -> Cow<'static, str> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable missing");

    let manifest_dir = std::path::Path::new(&manifest_dir);
    let display_script_path = manifest_dir.join("../display/dist/display.js");

    Cow::Owned(
        tokio::fs::read_to_string(display_script_path)
            .await
            .unwrap(),
    )
}

/// In release mode bake in the display script
#[cfg(not(debug_assertions))]
async fn get_display_styles() -> Cow<'static, str> {
    Cow::Borrowed(include_str!("../../../display/dist/display.css"))
}

/// When debugging, load the display script directly from the file system
/// this allows updating it at runtime
#[cfg(debug_assertions)]
async fn get_display_styles() -> Cow<'static, str> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable missing");

    let manifest_dir = std::path::Path::new(&manifest_dir);
    let display_script_path = manifest_dir.join("../display/dist/display.css");

    Cow::Owned(
        tokio::fs::read_to_string(display_script_path)
            .await
            .unwrap(),
    )
}
