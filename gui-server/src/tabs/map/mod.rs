use anyhow::bail;
use leptos::{html::Img, *};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
// use std::f64::consts::PI;
// use pip_boi_api::MenuTab;

// pub type MapData = (String, String);

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Zoom {
    zoom: u8,
    max: u8,
}

impl Zoom {
    pub fn new(max: u8) -> Self {
        Self { max, zoom: 0 }
    }

    pub fn inc(&mut self) -> anyhow::Result<()> {
        if self.zoom < self.max {
            self.zoom += 1;
            Ok(())
        } else {
            bail!("zoom already at max. refusing to zoom in further.")
        }
    }

    pub fn dec(&mut self) -> anyhow::Result<()> {
        if self.zoom > 0 {
            self.zoom -= 1;
            Ok(())
        } else {
            bail!("zoom already at min. refusing to zoom out further.")
        }
    }

    pub fn get_zoom(&self) -> u8 {
        self.zoom
    }
}

impl std::fmt::Debug for Zoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("{}", self.zoom)).finish()
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Self::new(16)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MapData {
    pub longitude: f32,
    pub latitude: f32,
    pub zoom: Zoom,
}

impl MapData {
    pub fn zoom_in(&mut self) -> anyhow::Result<()> {
        self.zoom.inc()
    }

    pub fn zoom_out(&mut self) -> anyhow::Result<()> {
        self.zoom.dec()
    }

    fn url(&self) -> String {
        let (x, y) =
            slippy_map_tiles::lat_lon_to_tile(self.latitude, self.longitude, self.zoom.get_zoom());

        format!("/tiles/{}/{x}/{y}.png", self.zoom.get_zoom())
    }

    fn merc_loc(&self) -> (f64, f64) {
        let n = (1 << self.zoom.get_zoom()) as f64;

        let lat = self.latitude as f64;
        let long = self.longitude as f64;

        // let x = (long + 180.0) * (map_width / 360.0);
        let x = n * (long + 180.0) / 360.0;
        let lat_rad = lat * PI / 180.0;

        // let merc_n = ((PI / 4.0).tan() + (lat_rad / 2.0)).ln();

        // let y = (map_height / 2.0) - (map_width * merc_n / (2.0 * PI));
        let y = n * (1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / PI) / 2.0;

        (x, y)
        // if let Some(ll) = LatLon::new(self.latitude, self.longitude) {
        //     ll.to_3857()
        // } else {
        //     (0.0, 0.0)
        // }
    }

    fn pixle_loc(&self) -> (f64, f64) {
        let map_size = 256.0;

        let (x, y) = self.merc_loc();

        // slippy_map_tiles::merc_location_to_tile_coords(x, y, self.zoom.get_zoom()).1
        ((x.fract() * map_size), (y.fract() * map_size))
    }
}

#[component]
pub fn MapApp(map_data: ReadSignal<MapData>) -> impl IntoView {
    use leptos_use::{use_element_size, UseElementSizeReturn};
    let el = create_node_ref::<Img>();

    let UseElementSizeReturn { width, height } = use_element_size(el);

    view! {
        <div class="justify-center h-full w-full object-cover p-8">
            <div class="flex justify-center h-full w-full relative inline">
                <img class="relative block" node_ref=el src=move || map_data().url() />

                <svg class="absolute top-0 block" viewbox="0 0 256 256" height={ move || {height.get()} } width={ move || {width.get()} } preserveAspectRatio="none" fill="none">

                    { move || {
                        let (x, y) = map_data().pixle_loc();

                        view! {
                            <circle cx={move || { (x * width.get()) / 256.0 }} cy={move || { (y * height.get()) / 256.0 }} r="5" stroke="rebeccapurple" fill="rebeccapurple"/>
                            // <circle cx="100%" cy=256 r=10 stroke="blue" fill="blue"/>
                        }
                    }}
                />
                </svg>
            </div>
        </div>
    }
}
