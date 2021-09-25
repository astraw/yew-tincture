use yew::{html, Callback, Component, Context, Html, Properties};

pub struct CheckboxLabel {
    css_id: String,
    /// copy of state in DOM
    checked: bool,
}

pub enum Msg {
    Toggle,
}

#[derive(PartialEq, Properties)]
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            css_id: uuid::Uuid::new_v4().to_string(),
            checked: ctx.props().initially_checked,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                let checked = !self.checked;
                self.checked = checked;
                if let Some(ref callback) = &ctx.props().oncheck {
                    callback.emit(checked);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Hmm, putting this in one html!{} macro fails, so make a list
        // manually.

        // Ideally, the structure would be <label><input /></label> but
        // I could not get this to work, so we are using `css_id` here.
        let el1 = html! {
            <input
                id={self.css_id.clone()}
                type="checkbox"
                checked={self.checked}
                onclick={ctx.link().callback(|_| Msg::Toggle)}
                />
        };
        let el2 = html! {
            <label for={self.css_id.clone()}>
                {ctx.props().label.as_str()}
            </label>
        };
        let mut vlist = yew::virtual_dom::vlist::VList::new();
        vlist.add_child(el1);
        vlist.add_child(el2);
        vlist.into()
    }
}
