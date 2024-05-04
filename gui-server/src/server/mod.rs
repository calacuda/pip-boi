use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use leptos_server_signal::ServerSignal;
use pip_boi_api::{MenuTab, MsgType};
use pip_boi_daemon::{tabs::*, *};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub mod ingest;

#[derive(Debug, Serialize, Deserialize)]
pub enum ZoomDirection {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

#[derive(Debug, Default)]
pub struct ServerSignalContainer {
    // pub serial_rx: Vec<ServerSignal<...>>
    pub heart_beat_graph: Vec<ServerSignal<GraphData>>,
    pub blood_oxygen_graph: Vec<ServerSignal<GraphData>>,
    pub menu_tab: Vec<ServerSignal<TabChangeMsg>>,
    pub map_data: Vec<ServerSignal<MapData>>,
}

pub async fn publish_hb(
    data: web::Data<Mutex<ServerSignalContainer>>,
    _req: HttpRequest,
    _stream: web::Payload,
) -> Result<String, Error> {
    let mut data = data.lock().unwrap();

    for _ in 0..2 {
        for sig in data.heart_beat_graph.iter_mut() {
            _ = sig.with(|data| data.append(0.5)).await;
        }
    }
    for n in [0.75, 1.0, 0.75, 0.5, 0.25, 0.0, 0.25] {
        for sig in data.heart_beat_graph.iter_mut() {
            _ = sig.with(|data| data.append(n)).await;
        }
    }

    for _ in 0..5 {
        for sig in data.heart_beat_graph.iter_mut() {
            _ = sig.with(|data| data.append(0.5)).await;
        }
    }

    Ok("Beat!".to_string())
}

pub async fn map_zoom_in(
    path: web::Path<(ZoomDirection,)>,
    data: web::Data<Mutex<ServerSignalContainer>>,
) -> Result<String, Error> {
    let mut data = data.lock().unwrap();
    let mut msg = format!("zoomed {:?}", path.0);

    match path.0 {
        ZoomDirection::In => {
            for sig in data.map_data.iter_mut() {
                _ = sig
                    .with(|data| {
                        if let Err(e) = data.zoom_in() {
                            msg = format!("{e}");
                        }
                    })
                    .await;
            }
        }
        ZoomDirection::Out => {
            for sig in data.map_data.iter_mut() {
                _ = sig
                    .with(|data| {
                        if let Err(e) = data.zoom_out() {
                            msg = format!("{e}");
                        }
                    })
                    .await;
            }
        }
    }

    Ok(msg)
}

pub async fn set_tab(
    path: web::Path<(MenuTab,)>,
    data: web::Data<Mutex<ServerSignalContainer>>,
) -> Result<String, Error> {
    for sig in data.lock().unwrap().menu_tab.iter_mut() {
        _ = sig.with(|data| data.tab = path.0).await;
    }

    Ok(format!("{}", path.0))
}

pub async fn set(
    data: web::Data<Mutex<ServerSignalContainer>>,
    path: web::Path<(MsgType,)>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // Ok("todo".to_string())
    ws::start(
        ingest::Ingest {
            event: data,
            data_type: path.0.clone(),
        },
        &req,
        stream,
    )
}
