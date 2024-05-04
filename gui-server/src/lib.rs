use pip_boi_api::MenuTab;
use serde::{Deserialize, Serialize};
pub mod app;
pub mod tabs;

// #[cfg(feature = "ssr")]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct TabChangeMsg {
    pub tab: MenuTab,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;
    use log::*;

    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(Level::Trace);
    log::trace!("hydrate mode - hydrating");

    mount_to_body(App);
    // leptos_dom::HydrationCtx::stop_hydrating();
}
