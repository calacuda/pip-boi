use crate::TabChangeMsg;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_server_signal::create_server_signal;
use log::*;
use pip_boi_api::MenuTab;

#[component]
pub fn App() -> impl IntoView {
    use crate::tabs::*;
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // #[cfg(feature = "csr")]
    // {
    //     let tab = window().location();
    //     info!("window location: {tab:?}");
    // }
    // #[cfg(not(feature = "csr"))]
    // {
    //     let tab = MenuTab::Stats;
    //     info!("menu => Stats");
    // }

    // let (menu_tab, set_menu_tab) = create_signal(MenuTab::Stats);

    // TODO: move server signals here.

    leptos_server_signal::provide_websocket("ws://localhost:3000/ws").unwrap();

    let heart_rate_data = create_server_signal::<GraphData>("heart_beat_graph");
    let spo2_data = create_server_signal::<GraphData>("blood_oxygen_graph");
    let map_data = create_server_signal::<MapData>("map_data");

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pip-boi-daemon.css"/>

        // sets the document title
        <Title text="PIP-Boi"/>

    <div class="h-screen w-screen bg-black">
            // content for this welcome page
            <Router>
                <nav>
                    <div class="w-screen h-[10vh]">
                        <MenuBar/>
                    </div>
                </nav>
                <main class="bg-black w-[100vw] h-[90vh] justify-center">
                    <Routes>
                        <Route path="" view=HomePage/>
                        // TODO: set menu_tab based on route
                        <Route path="/stats" view=move || { view! { <StatsApp heart_rate_data=heart_rate_data spo2_data=spo2_data/> }} />
                        <Route path="/map" view=move || { view! { <MapApp map_data=map_data/> }} />
                        <Route path="/com" view=ComApp/>
                        <Route path="/ducky" view=DuckyApp/>
                        <Route path="/cal" view=CalApp/>
                        <Route path="/todo" view=TodoApp/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        // let resp = expect_context::<leptos_actix::ResponseOptions>();
        // resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1 class="text-xl font-xl px-4 text-green-700">"Not Found"</h1>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // The navigation function returned by `use_navigate` should not be called during server rendering.
    let redir = move || {
        spawn_local(async {
            _ = redirect("/stats".to_string()).await;
        });
    };

    view! {
        <div class="text-xl font-xl px-4 text-green-700">
            "redirecting to stats page"
            { redir() }
        </div>
    }
}

#[server(Redirect, "/redirect")]
pub async fn redirect(target: String) -> Result<(), ServerFnError> {
    leptos_actix::redirect(&target);

    Ok(())
}

#[server(UpdateTab, "/update_tab")]
pub async fn update_tab(tab: MenuTab) -> Result<(), ServerFnError> {
    if let Err(e) = reqwest::get(&format!(
        "http://localhost:3000/api/set-tab/{}",
        tab.to_string().to_lowercase()
    ))
    .await
    {
        error!("could not update tab on server. got error: {e}");
    }

    Ok(())
}

#[component]
fn MenuBar() -> impl IntoView {
    use pip_boi_api::IntoEnumIterator;

    // leptos_server_signal::provide_websocket("ws://localhost:3000/ws").unwrap();

    let get_tab = create_server_signal::<TabChangeMsg>("tab-alert");

    let tabs: Vec<MenuTab> = MenuTab::iter().collect();
    let tabs_2: Vec<MenuTab> = MenuTab::iter().collect();
    // console_log(format!("{tabs:?}").as_str());
    let n_tabs = tabs.len();
    // let navigate = use_navigate();
    let navigate = use_navigate();

    create_effect(move |_| {
        info!("navigating to - {}", get_tab.get().tab);

        navigate(
            &get_tab.get().tab.to_string().to_lowercase(),
            Default::default(),
        );
    });

    let buttons = move || {
        tabs.clone()
            .into_iter()
            .map(|tab| {
                view! {
                    <div class="row-start-1 row-span-1" role="group">
                        <a
                            class="text-xl font-xl px-4 text-green-700 inline-block"
                            rel="external"
                            on:click=move |_| {
                                if tab != get_tab().tab {
                                    spawn_local(async move { let _ = update_tab(tab).await; });
                                }
                            }
                            // href={ format!("/{}", tab.to_string().to_lowercase()) }
                            // href=format!("/{}", tab.to_string().to_lowercase())
                        >
                            {move || {
                                if tab == get_tab.get().tab {
                                    // log::debug!("selected tab = {tab}");
                                    view! {
                                        <div class="inline inset-y-0 left-0">
                                            "\u{250c}\u{2500} "
                                        </div>
                                        <div class="inline text-green-400">
                                            { format!("{tab}") }
                                        </div>
                                        <div class="inline inset-y-0 right-0">
                                            " \u{2500}\u{2510}"
                                        </div>
                                    }
                                } else {
                                    // format!("  {tab}  ")

                                    view!{
                                        <div class="text-green-900" >
                                            { format!("  {tab}  ") }
                                        </div>
                                        <div> </div>
                                    }
                                }
                            }}
                        </a>
                    </div>
                }
            })
            .collect::<Vec<_>>()
    };

    let underline = move || {
        tabs_2
        .clone()
        .into_iter()
        .map(|tab| {
            view! {
                <div class="relative row-start-2 text-xl font-xl text-green-700" role="group">
                    {move || {
                        let tab_len = format!("{tab}").len();

                        if tab == get_tab.get().tab {
                            view! {
                                <div class="absolute inset-y-0 left-0">
                                    {format!("\u{2500}\u{2518}")}
                                </div>
                                <div class="absolute inset-y-0 right-0">
                                    {format!("\u{2514}\u{2500}")}
                                </div>
                            }
                        } else {
                            view! {
                                <div>
                                    {format!("{}", std::iter::repeat("\u{2500}").take(tab_len + (4 * 2)).collect::<String>())}
                                </div>
                                <div> </div>
                            }
                        }
                    }}
                </div>
            }
        })
        .collect::<Vec<_>>()
    };

    view! {
        // { debug!("rendering menu bar") }
        <div class="grid grid-flex-col justify-center bg-black stretch">
            <div class="col-start-1 col-span-1 row-start-1 row-span-1"> </div>
            { buttons }
            <div class=format!("cols-start-{} col-span-1 row-start-1 row-span-1", n_tabs + 2)> </div>

            <div class="relative col-start-1 col-span-1 row-start-2 text-xl font-xl px-4 text-green-700">
                <p class="absolute inset-y-0 right-0">
                    "\u{250c}\u{2500}"
                </p>
            </div>
            { underline }
            <div class=format!("relative text-left cols-start-{} col-span-1 row-start-2 text-xl font-xl px-4 text-green-700", n_tabs + 2)>
                <p class="absolute inset-y-0 left-0">
                    "\u{2500}\u{2510}"
                </p>
            </div>
        </div>
    }
}
