use leptos::*;
use pip_boi_api::MenuTab;

// use serde::{Deserialize, Serialize};
// use leptos_server_signal::create_server_signal;

#[component]
pub fn DuckyApp() -> impl IntoView {
    // leptos_server_signal::provide_websocket("ws://localhost:3000/sss/map-data").unwrap();

    // let map_data = create_server_signal::<MapData>("map_data");

    view! {
        <div class="justify-center grid grid-flow-col">
            // TODO: do maps stuff here
            <super::TodoPage menu_tab=MenuTab::Ducky/>
        </div>
    }
}
