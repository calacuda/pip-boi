use actix::{Actor, Addr, Handler, StreamHandler};
use actix_broker::{BrokerSubscribe, SystemBroker};
use actix_web::web;
use actix_web_actors::ws;
use pip_boi_api::{MsgType, PipBoiMsg};
use std::sync::Mutex;

/// Define HTTP actor
#[derive(Clone)]
pub struct Ingest {
    pub event: web::Data<Mutex<super::ServerSignalContainer>>,
    pub data_type: MsgType,
}

impl Actor for Ingest {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ingest {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // ctx.text("{\"response\":\"text messages/responces are not yet implemented\"}")
                // ctx.text(text.clone());

                // debug!("connection {} sent a message", self.id);
                // self.event.do_send(PipBoiMsgInternalWrapper {
                // id: self.id,
                // message,
                // })
                match self.data_type {
                    MsgType::HeartRate => {
                        if let Ok(message) = serde_json::from_str::<f64>(&text.to_string()) {
                            let mut signals = self.event.lock().unwrap().heart_beat_graph.clone();
                            // let n = message.as_f64().unwrap_or(0.5);
                            actix::spawn(async move {
                                for sig in signals.iter_mut() {
                                    _ = sig.with(|data| data.append(message)).await;
                                }
                            });
                        } else {
                            // warn!("received a message that doesn't follow Sherlock Message specifications. Did you serialize it using from a SherlockMessage struct/object?");
                            ctx.text("{\"response\":\"malformed JSON message.\"}")
                        }
                    }
                    // TODO: write the rest of the setters
                    _ => {}
                }
            }
            Ok(ws::Message::Binary(_bin)) => {
                ctx.text("{\"response\":\"binary messages/responces are not yet implemented\"}")
            } // ctx.binary(bin),
            _ => (),
        }
    }
}
