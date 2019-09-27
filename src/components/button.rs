use yew::prelude::*;

pub struct Button {
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
    #[props(required)]
    pub title: String,
    #[props(required)]
    pub onsignal: Callback<()>,
    pub disabled: bool,
    pub is_active: bool,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button {
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
}

impl Renderable<Button> for Button {
    fn view(&self) -> Html<Self> {
        let active_class = if self.is_active {
            "btn-active"
        } else {
            "btn-inactive"
        };
        html! {
            <button class=("btn",{active_class}), onclick=|_| Msg::Clicked, disabled=self.disabled,>{ &self.title }</button>
        }
    }
}
