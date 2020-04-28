use yew::prelude::*;

pub struct CheckboxLabel {
    link: ComponentLink<Self>,
    css_id: String,
    label: String,
    oncheck: Option<Callback<bool>>,
    checked: bool,
}

pub enum Msg {
    Toggle,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub oncheck: Option<Callback<bool>>,
    /// Sets the `checked` property once, upon the creation of the checkbox.
    #[prop_or_default]
    pub initially_checked: bool,
    /// When not None, sets the `checked` property of the checkbox.
    #[prop_or_default]
    pub updating_checked: Option<bool>,
}

impl Component for CheckboxLabel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            css_id: uuid::Uuid::new_v4().to_string(),
            label: props.label,
            oncheck: props.oncheck,
            checked: props.initially_checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                let checked = !self.checked;
                self.checked = checked;
                if let Some(ref mut callback) = self.oncheck {
                    callback.emit(checked);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
        self.oncheck = props.oncheck;
        if let Some(checked) = props.updating_checked {
            self.checked = checked;
        }
        // ignore initially_checked
        true
    }

    fn view(&self) -> Html {
        // Hmm, putting this in one html!{} macro fails, so make a list
        // manually.

        // Ideally, the structure would be <label><input /></label> but
        // I could not get this to work, so we are using `css_id` here.
        let el1 = html! {
            <input
                id=&self.css_id,
                type="checkbox",
                checked=self.checked,
                onclick=self.link.callback(|_| Msg::Toggle),
                />
        };
        let el2 = html! {
            <label for=&self.css_id,>
                {&self.label}
            </label>
        };
        let mut vlist = yew::virtual_dom::vlist::VList::new();
        vlist.add_child(el1);
        vlist.add_child(el2);
        vlist.into()
    }
}
