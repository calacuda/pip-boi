use leptos::{html::Div, *};
use leptos_use::{use_element_size, UseElementSizeReturn};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct GraphData {
    pub data: Vec<f64>,
    pub i: usize,
    size: usize,
}

impl GraphData {
    pub fn set_up(&mut self, size: usize) {
        // self.data = Vec::from(&[0.5; 50])
        // self.data = (0..size).map(|_| 0.5).collect();
        // self.data = Vec::with_capacity(size);
        self.size = size;
        self.i = 0;
    }

    pub fn append(&mut self, n: f64) {
        self.i = (self.i + 1) % self.size;

        if self.data.len() == self.size {
            self.data[self.i] = n;
        } else {
            self.data.push(n);
        }
        // println!("setting i to: {}", self.i);
    }
}

#[component]
pub fn StatsApp(
    heart_rate_data: ReadSignal<GraphData>,
    spo2_data: ReadSignal<GraphData>,
) -> impl IntoView {
    let (heart_rate, set_heart_rate) = create_signal(0.0);
    let (spo2, set_spo2) = create_signal(0.0);
    // let (spo2_data, set_spo2_data) = create_signal(Vec::from([0.5; 25]));
    // let heart_rate_data = create_server_signal::<GraphData>("heart_beat_graph");
    // let spo2_data = create_server_signal::<GraphData>("blood_oxygen_graph");

    view! {
        // <div class="justify-center grid grid-flow-row flex h-full w-full">
        //     // menu bar should include: "stats" (including heart rate & blood oxygen),
        //     // "map" (real worlkd map with GPS), "radio" (internet
        //     // radio/spotify/local mp3, yet unsure), "Ducky" (to select ducky scripts)
        //     // over USB or Bluetooth), "Serial" (monitor of UART/RS-232 port), "Cal" (Callender),
        //     // "TODO" (daily tasks to complete).
        //     <MenuBar set_tab=set_tab get_tab=menu_tab.clone()/>
        <div class="justify-center grid grid-flow-col w-full h-full">
            // <div class="grid grid-flow-col flex justify-center h-dvh">
            // left side menu
            <LeftPanel heart_rate=heart_rate spo2=spo2/>

            // display pip boy guy
            <MainDisplay/>

            // right side menu
            <RightPanel heart_rate_data=heart_rate_data spo2_data=spo2_data/>
        </div>
    }
}

#[component]
fn LeftPanel(heart_rate: ReadSignal<f64>, spo2: ReadSignal<f64>) -> impl IntoView {
    view! {
        <div class="justify-center items-top h-full w-25vw shrink-0">
            <h1 class="text-xl font-xl px-4 text-green-700"> "Heart Rate" </h1>
            <hr class="border-green-700"/>
            <p class="text-xl font-xl px-4 text-green-700"> {move || { format!("{:.2}", heart_rate.get()) }} </p>
            <br/>

            <h1 class="text-xl font-xl px-4 text-green-700"> "Oximiter" </h1>
            <hr class="border-green-700"/>
            <p class="text-xl font-xl px-4 text-green-700"> {move || {  format!("{:.2}", spo2.get()) }} </p>
            <br/>

            <h1 class="text-xl font-xl px-4 text-green-700"> "Medical Info" </h1>
            <hr class="border-green-700"/>
        </div>
    }
}

#[component]
fn MainDisplay() -> impl IntoView {
    view! {
        <div class="flex justify-center h-full w-50vw shrink-0">
            <img src="/assets/pip-boy-walking-animation.gif" alt="PIP BOI WALKING"/>
        </div>
    }
}

#[component]
fn RightPanel(
    heart_rate_data: ReadSignal<GraphData>,
    spo2_data: ReadSignal<GraphData>,
) -> impl IntoView {
    view! {
        <div class="h-full w-25vw">
            <div class="w-full" role="group">
                <h1 class="text-xl font-xl px-4 text-green-700"> "Patient Monitor" </h1>
                <hr class="border-green-700"/>
            </div>

            <br/>

            <div class="justify-center w-full" role="group">
                <Graph data=heart_rate_data/>
                <Graph data=spo2_data/>
            </div>
        </div>
    }
}

#[component]
fn Graph(data: ReadSignal<GraphData>) -> impl IntoView {
    let buffer = move || {
        with!(|data| {
            let blip_loc = data.i;
            // info!("blip: {blip_loc}");

            data.data
                .clone()
                .into_iter()
                .zip(data.data.clone().into_iter().skip(1))
                .enumerate()
                .map(move |(i, (y_1, y_2))| (i, (y_1, y_2), i == blip_loc))
        })
    };

    let el = create_node_ref::<Div>();
    let UseElementSizeReturn {
        width,
        height: _height,
    } = use_element_size(el);

    view! {
        <div node_ref=el class="w-fill h-fill">
        // <div node_ref=el class="w-25vw h-fill">
            // { console_log(format!("data buffer: {:?}", data.get()).as_str()); }
            <svg viewbox="0 0 200 100" width={ width }>
                // viewbox="0 0 200 100" width="100%" height="100%"
                // NOTE: all the 50s are bc there are 50 messages in the buffer
                <For
                    each=buffer
                    key=move |dp| (dp.2, width().to_string(), ((dp.0 as f64 / 50.0 * 100.0) as i64, (100.0 * dp.1.0) as i64), (((dp.0 + 1) as f64 / 50.0 * 100.0) as i64, (100.0 * dp.1.1) as i64))
                    children=move |(i, (d_1, d_2), blip)| {
                        // let color = if data.get().i == i { "black" } else { "green" };
                        let color = if blip { /* info!("blip at index: {i}"); */ "black" } else { "green" };

                        // console_log(format!("data buffer: {:?}", data.get()).as_str()); }
                        // let svg_w = if width() > 0.0 {
                        //     width()
                        // } else {
                        //     200.0
                        // };

                        view! {
                            // { console_log(format!("data buffer: {:?}", data.get()).as_str()); }
                            <line
                                // class="fill-current text-green-700 fill-current"
                                x1={i as f64 * width() / 50.0}
                                y1={100.0 * d_1}
                                x2={(i+1) as f64 * width() / 50.0}
                                y2={100.0 * d_2}
                                stroke={ color }
                                stroke-width="2"
                            />
                        }
                    }
                />
            </svg>
        </div>
    }
}
