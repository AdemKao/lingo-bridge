//! Status-only Tauri command: this scaffold does not capture any audio.

use lingo_core::CallRoutes;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct BootstrapStatus {
    os: &'static str,
    outbound: String,
    inbound: String,
    line_audio_capture_ready: bool,
    virtual_microphone_ready: bool,
    realtime_translation_ready: bool,
}

#[tauri::command]
fn bootstrap_status() -> BootstrapStatus {
    let routes = CallRoutes::default();
    BootstrapStatus {
        os: std::env::consts::OS,
        outbound: format!(
            "{} → {}",
            routes.outbound.source.code(),
            routes.outbound.target.code()
        ),
        inbound: format!(
            "{} → {}",
            routes.inbound.source.code(),
            routes.inbound.target.code()
        ),
        line_audio_capture_ready: false,
        virtual_microphone_ready: false,
        realtime_translation_ready: false,
    }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![bootstrap_status])
        .run(tauri::generate_context!())
        .expect("failed to start Lingo Bridge desktop");
}
