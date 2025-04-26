use std::borrow::Cow;

use tokio::join;

pub async fn inject_property_inspector_current(input: &str) -> String {
    let (inspector_script, inspector_styles) =
        join!(get_inspector_script(), get_inspector_styles());

    inject_property_inspector(input, &inspector_script, &inspector_styles)
}

fn inject_property_inspector(
    value: &str,
    inspector_script: &str,
    inspector_styles: &str,
) -> String {
    value.replace(
        "<head>",
        &format!("<head><script>{inspector_script}</script><style>{inspector_styles}</style>",),
    )
}

/// In release mode bake in the inspector script
#[cfg(not(debug_assertions))]
async fn get_inspector_script() -> Cow<'static, str> {
    Cow::Borrowed(include_str!("../../../../inspector/dist/inspector.js"))
}

/// When debugging, load the inspector script directly from the file system
/// this allows updating it at runtime
#[cfg(debug_assertions)]
async fn get_inspector_script() -> Cow<'static, str> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable missing");

    let manifest_dir = std::path::Path::new(&manifest_dir);
    let inspector_script_path = manifest_dir.join("../inspector/dist/inspector.js");

    Cow::Owned(
        tokio::fs::read_to_string(inspector_script_path)
            .await
            .unwrap(),
    )
}

/// In release mode bake in the inspector script
#[cfg(not(debug_assertions))]
async fn get_inspector_styles() -> Cow<'static, str> {
    Cow::Borrowed(include_str!("../../../../inspector/dist/inspector.css"))
}

/// When debugging, load the inspector script directly from the file system
/// this allows updating it at runtime
#[cfg(debug_assertions)]
async fn get_inspector_styles() -> Cow<'static, str> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable missing");

    let manifest_dir = std::path::Path::new(&manifest_dir);
    let inspector_script_path = manifest_dir.join("../inspector/dist/inspector.css");

    Cow::Owned(
        tokio::fs::read_to_string(inspector_script_path)
            .await
            .unwrap(),
    )
}
