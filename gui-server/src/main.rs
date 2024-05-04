use std::sync::Mutex;

#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::server::*;
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use pip_boi_daemon::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    // let msg_event_addr = web::Data::new(PipBoiMsgData.start());
    let server_signals = web::Data::new(Mutex::new(ServerSignalContainer::default()));

    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(server_signals.clone())
            .route("/ws", web::get().to(server_event))
            .service(
                web::scope("/api")
                    .route("/set/{id}", web::get().to(set))
                    .route("/publish-hb", web::get().to(publish_hb))
                    .route("/map-zoom/{direction}", web::get().to(map_zoom_in))
                    .route("set-tab/{tab}", web::get().to(set_tab)),
            )
            // serve JS/WASM/CSS from 'pkg'
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .route("/tiles/{zoom}/{x}/{y}.png", web::get().to(tiles))
            // serve map tiles from 'tiles'
            // .service(Files::new("/tiles", "/tiles").show_files_listing())
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use pip_boi_daemon::app::*;

    console_log::init_with_level(Level::Trace);

    leptos::mount_to_body(App);
}

#[cfg(feature = "ssr")]
async fn tiles(
    path: actix_web::web::Path<(u8, u32, u32)>,
) -> actix_web::Result<actix_files::NamedFile> {
    let path = format!("./tiles/{}/{}/{}.png", path.0, path.1, path.2);
    let file = actix_files::NamedFile::open(path)?;

    Ok(file)
}

#[cfg(feature = "ssr")]
pub async fn server_event(
    data: actix_web::web::Data<Mutex<crate::server::ServerSignalContainer>>,
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> impl actix_web::Responder {
    use actix_ws::handle;
    use leptos_server_signal::ServerSignal;
    use pip_boi_daemon::{
        tabs::{GraphData, MapData},
        TabChangeMsg,
    };

    let (res, session, _msg_stream) = handle(&req, stream).unwrap();

    let mut hb_graph = ServerSignal::<GraphData>::new("heart_beat_graph", session.clone()).unwrap();
    let _ = hb_graph.with(|data| data.set_up(50)).await;

    let mut spo2_graph =
        ServerSignal::<GraphData>::new("blood_oxygen_graph", session.clone()).unwrap();
    let _ = spo2_graph.with(|data| data.set_up(50)).await;

    let menu_tab = ServerSignal::<TabChangeMsg>::new("tab-alert", session.clone()).unwrap();

    let mut map_data = ServerSignal::<MapData>::new("map_data", session.clone()).unwrap();
    let _ = map_data
        .with(|data| {
            data.longitude = -73.935242;
            data.latitude = 40.730610;

            for _ in 0..11 {
                _ = data.zoom.inc();
            }
        })
        .await;

    let mut data = data.lock().unwrap();
    // let mut hbg = ;
    data.heart_beat_graph.push(hb_graph);
    // let mut bog = data.blood_oxygen_graph;
    data.blood_oxygen_graph.push(spo2_graph);
    // let mut mt = data.menu_tab;
    data.menu_tab.push(menu_tab);
    data.map_data.push(map_data);

    res
}
