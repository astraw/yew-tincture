use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use yew_tincture::components::{Button, TypedInput, TypedInputStorage};

#[derive(Debug)]
enum Msg {
    OnSizeInput,
    DoUpdates(bool),
    Tick,
}

struct Model {
    size: TypedInputStorage<f64>,
    interval: Option<Interval>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            size: TypedInputStorage::empty(),
            interval: None, //Interval::new(100, move || link.send_message(Msg::Tick)),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnSizeInput => {
                let result = self.size.get();
                log::info!("OnSizeInput received: {result:?}");
            }
            Msg::DoUpdates(val) => {
                if val {
                    let link = ctx.link().clone();
                    self.interval = Some(Interval::new(100, move || link.send_message(Msg::Tick)));
                } else {
                    self.interval = None;
                };
            }
            Msg::Tick => {
                let mut size = if let Ok(size) = self.size.get() {
                    size
                } else {
                    0.0
                };
                size += 0.1;
                self.size.set_if_not_focused(size);
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_updating = if self.interval.is_some() {
            "updates started"
        } else {
            "updates not started"
        };
        html! {
            <div>
                <label>{"size"}
                    <TypedInput<f64>
                        storage={self.size.clone()}
                        on_input={ctx.link().callback(|_| Msg::OnSizeInput)}
                        />
                </label>
                <p>{is_updating}</p>
                <p><Button title={"Start updates"} onsignal={ctx.link().callback(|_| Msg::DoUpdates(true))} /></p>
                <p><Button title={"Stop updates"} onsignal={ctx.link().callback(|_| Msg::DoUpdates(false))} /></p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Model>::new().render();
}
