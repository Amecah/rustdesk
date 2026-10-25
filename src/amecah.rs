//! Built-in configuration for the AMECAH build of RustDesk.
//!
//! Upstream RustDesk only accepts this kind of configuration from a `custom.txt`
//! signed by RustDesk Ltd. Here it is compiled in, so the client always talks to
//! the AMECAH server and needs no setup on the customer's machine.
//!
//! Build-time environment variables:
//! - `AMECAH_RENDEZVOUS_SERVER`: hbbs/hbbr host (default `soporte.amecah.com.mx`).
//! - `AMECAH_RS_PUB_KEY`: server public key (`id_ed25519.pub`). Without it the
//!   connection to the server is not authenticated.
//! - `AMECAH_QUICK_SUPPORT=1`: incoming-only client, no installation, no settings
//!   (the equivalent of TeamViewer QuickSupport). Otherwise a full client is built
//!   for technicians.
//! - `AMECAH_APP_NAME`: app name. By default it is only changed on Windows, since on
//!   macOS the name must match the `.app` bundle and on Linux the package names.

use serde_json::{json, Map, Value};
use std::collections::HashMap;

const DEFAULT_RENDEZVOUS_SERVER: &str = "soporte.amecah.com.mx";

fn non_empty(v: Option<&'static str>) -> Option<&'static str> {
    v.map(str::trim).filter(|v| !v.is_empty())
}

pub fn rendezvous_server() -> &'static str {
    non_empty(option_env!("AMECAH_RENDEZVOUS_SERVER")).unwrap_or(DEFAULT_RENDEZVOUS_SERVER)
}

pub fn is_quick_support() -> bool {
    matches!(
        non_empty(option_env!("AMECAH_QUICK_SUPPORT")),
        Some("1") | Some("true") | Some("Y")
    )
}

fn app_name() -> Option<&'static str> {
    if let Some(name) = non_empty(option_env!("AMECAH_APP_NAME")) {
        return Some(name);
    }
    if cfg!(windows) {
        Some(if is_quick_support() {
            "AmecahSoporte"
        } else {
            "Amecah"
        })
    } else {
        None
    }
}

fn build_config() -> HashMap<String, Value> {
    let server = rendezvous_server();

    // Forced: the user cannot point the client to another server.
    let mut overrides = Map::new();
    overrides.insert("custom-rendezvous-server".into(), json!(server));
    overrides.insert("relay-server".into(), json!(server));
    // The API server (port 21114) only exists in RustDesk Server Pro.
    overrides.insert("api-server".into(), json!(""));
    match non_empty(option_env!("AMECAH_RS_PUB_KEY")) {
        Some(key) => {
            overrides.insert("key".into(), json!(key));
        }
        None => log::warn!("AMECAH_RS_PUB_KEY was not set at build time"),
    }
    // Updates come from AMECAH, not from rustdesk.com.
    overrides.insert("enable-check-update".into(), json!("N"));
    overrides.insert("allow-auto-update".into(), json!("N"));
    overrides.insert("register-device".into(), json!("N"));
    overrides.insert("hide-server-settings".into(), json!("Y"));

    let mut defaults = Map::new();
    defaults.insert("lang".into(), json!("es"));

    let mut data = HashMap::new();
    if let Some(name) = app_name() {
        data.insert("app-name".into(), json!(name));
    }
    if is_quick_support() {
        data.insert("conn-type".into(), json!("incoming"));
        data.insert("disable-installation".into(), json!("Y"));
        data.insert("disable-settings".into(), json!("Y"));
        data.insert("disable-account".into(), json!("Y"));
        data.insert("disable-ab".into(), json!("Y"));
        // Like QuickSupport: one-time password, shown to the customer.
        overrides.insert(
            "verification-method".into(),
            json!("use-temporary-password"),
        );
        overrides.insert("approve-mode".into(), json!("password"));
    }
    data.insert("default-settings".into(), Value::Object(defaults));
    data.insert("override-settings".into(), Value::Object(overrides));
    data
}

pub fn load_builtin_config() {
    crate::common::apply_custom_client(build_config());
}
