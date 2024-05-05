pub use leptos::*;
use pip_boi_api::MenuTab;

pub mod cal;
pub mod com;
pub mod ducky;
pub mod map;
pub mod stats;
pub mod todo;

pub use cal::CalApp;
pub use com::{ComApp, SerialData, SerialMessages};
pub use ducky::DuckyApp;
pub use map::{MapApp, MapData};
pub use stats::{GraphData, StatsApp};
pub use todo::TodoApp;

#[component]
pub fn TodoPage(menu_tab: MenuTab) -> impl IntoView {
    view! {
        <div class="text-xl font-xl px-4 text-green-700">
            { menu_tab.to_string() }
        </div>
        <div class="text-xl font-xl px-4 text-green-700">
            "This page is a Work In Progress, consider it a TODO."
        </div>
    }
}
