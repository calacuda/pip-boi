use chrono::{DateTime, Utc};
use leptos::{leptos_dom::logging::console_log, *};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub type SingleMsg = (Vec<u8>, DateTime<Utc>);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SerialData {
    pub msg: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

/// a collection of serial messages
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SerialMessages {
    pub sent_msgs: VecDeque<SingleMsg>,
    pub recv_msgs: VecDeque<SingleMsg>,
    sent_msg_i: usize,
    recv_msg_i: usize,
}

impl SerialMessages {
    /// returns the limit size of this type. the sumed lengths of both VecDeques must always be
    /// less then or equal to this.
    fn limit(&self) -> usize {
        500
    }

    fn full(&self) -> bool {
        self.sent_msgs.len() + self.recv_msgs.len() > self.limit()
    }

    fn shorten(&mut self) {
        match (self.sent_msgs.get(0), self.recv_msgs.get(0)) {
            (Some(sent_msg), Some(recv_msg)) => {
                if sent_msg.1 > recv_msg.1 {
                    self.sent_msgs.pop_front();
                } else {
                    self.recv_msgs.pop_front();
                }
            }
            (Some(_), None) => {
                self.sent_msgs.pop_front();
            }
            (None, Some(_)) => {
                self.recv_msgs.pop_front();
            }
            (None, None) => {}
        }
    }

    pub fn push_rx(&mut self, msg: SingleMsg) {
        if self.full() {
            self.shorten();
        }

        self.recv_msgs.push_back(msg);
    }

    pub fn push_tx(&mut self, msg: SingleMsg) {
        if self.full() {
            self.shorten();
        }

        self.sent_msgs.push_back(msg);
    }
}

impl Iterator for SerialMessages {
    type Item = ((Vec<u8>, DateTime<Utc>), bool);

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.sent_msgs.get(self.sent_msg_i),
            self.recv_msgs.get(self.recv_msg_i),
        ) {
            (Some(sent_msg), Some(recv_msg)) => {
                if sent_msg.1 <= recv_msg.1 {
                    self.sent_msg_i += 1;
                    Some((sent_msg.clone(), true))
                } else {
                    self.recv_msg_i += 1;
                    Some((sent_msg.clone(), false))
                }
            }
            (Some(sent_msg), None) => {
                self.sent_msg_i += 1;
                Some((sent_msg.clone(), true))
            }
            (None, Some(recv_msg)) => {
                self.recv_msg_i += 1;
                Some((recv_msg.clone(), false))
            }
            (None, None) => {
                self.sent_msg_i = 0;
                self.recv_msg_i = 0;

                None
            }
        }
    }
}

#[component]
pub fn ComApp(
    rx_msgs: ReadSignal<SerialMessages>,
    add_msgs: WriteSignal<SerialMessages>,
) -> impl IntoView {
    let input_element: NodeRef<html::Textarea> = create_node_ref();
    let (new_msg, set_new_msg) = create_signal(String::new());

    let submit_message = move |ev| {
        let mut msg_text = event_target_value(&ev);

        let text = if event_target_value(&ev).ends_with("\n")
            && !event_target_value(&ev).ends_with("\\n")
        {
            msg_text.pop();
            let msg = (
                msg_text.as_bytes().to_owned().into_iter().collect(),
                Utc::now(),
            );
            // console_log(&format!("msg => {msg:?}"));
            // TODO: send message to backend
            add_msgs.update(|msgs| msgs.push_tx(msg));
            String::new()
        } else {
            msg_text
        };

        set_new_msg.set(text);
    };

    view! {
        <div class="justify-center w-full h-full shrink-0 px-8 pb-8">
            // <super::TodoPage menu_tab=MenuTab::Serial/>
            <div class="justify-center w-full h-full shrink-0 p-2 border-green-700 border-2 flex flex-col space-y-2">
                // display messages (give it its own scroll-bar.)
                <div class="h-fill w-full border-green-700 border-2 p-2 flex-auto overflow-y-scroll flex flex-col-reverse" role="group">
                    <For
                        each=move || {
                            let mut msgs: Vec<(usize, (SingleMsg, bool))> = rx_msgs().enumerate().collect();
                            msgs.reverse();
                            msgs.into_iter()
                        }
                        key=move |msg| msg.clone()
                        children=|(i, ((msg_bytes, _time_stamp), is_user))| {
                            let msg_text = String::from_utf8(msg_bytes).expect("500 deserializeation failed");

                            view! {
                                <pre class="text-xl font-xl text-green-600 flex-wrap">
                                    { format!(" {i: >3} {} | ", if is_user { ">" } else {"<"}) }
                                    <div class="text-xl font-xl text-green-700 inline"> { msg_text } </div>
                                </pre>
                            }
                        }
                    />
                </div>

                // text box input with send bytes & send string
                <div class="w-full border-green-700 border-2 p-2">
                    <textarea
                        node_ref=input_element
                        on:input=submit_message
                        prop:value=new_msg
                        rows="1"
                        cols="33"
                        maxlength="500"
                        class="resize-none text-xl font-xl text-green-700 bg-black w-full"
                    >
                    </textarea>
                </div>
            </div>
        </div>
    }
}
