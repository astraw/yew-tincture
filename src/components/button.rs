use yew::{Callback, Component, Context, Html, Properties, classes, html};

/// A button that emits `onsignal` when clicked.
///
/// The button gets the CSS classes `btn` and either `btn-active` or
/// `btn-inactive`, depending on `is_active`.
#[derive(Debug)]
pub struct Button {}

#[derive(Debug)]
pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    pub onsignal: Callback<()>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub is_active: bool,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                ctx.props().onsignal.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let active_class = if ctx.props().is_active {
            "btn-active"
        } else {
            "btn-inactive"
        };
        html! {
            <button
                class={classes!("btn",active_class)}
                onclick={ctx.link().callback(|_| Msg::Clicked)}
                disabled={ctx.props().disabled}>{ &ctx.props().title }
            </button>
        }
    }
}
