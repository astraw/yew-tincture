use yew::prelude::*;

pub struct Button {
    link: ComponentLink<Self>,
    title: String,
    onsignal: Callback<()>,
    disabled: bool,
    is_active: bool,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone, Properties)]
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            title: props.title,
            onsignal: props.onsignal,
            disabled: props.disabled,
            is_active: props.is_active,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.onsignal.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.onsignal = props.onsignal;
        self.disabled = props.disabled;
        self.is_active = props.is_active;
        true
    }

    fn view(&self) -> Html {
        let active_class = if self.is_active {
            "btn-active"
        } else {
            "btn-inactive"
        };
        html! {
            <button class=classes!("btn",{active_class}) onclick=self.link.callback(|_| Msg::Clicked) disabled=self.disabled>{ &self.title }</button>
        }
    }
}
