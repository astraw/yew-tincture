use yew::{Callback, Component, Context, Html, Properties, html};

/// A checkbox followed by a `<label>` for it.
///
/// Emits `oncheck` with the new state whenever the user toggles the checkbox.
#[derive(Debug)]
pub struct CheckboxLabel {
    css_id: String,
    /// copy of state in DOM
    checked: bool,
}

#[derive(Debug)]
pub enum Msg {
    Toggle,
}

#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub oncheck: Option<Callback<bool>>,
    /// Sets the `checked` property once, upon the creation of the checkbox.
    #[prop_or_default]
    pub initially_checked: bool,
    /// When not `None`, sets the `checked` property of the checkbox whenever the
    /// parent re-renders, overriding `initially_checked`.
    #[prop_or_default]
    pub updating_checked: Option<bool>,
}

impl Component for CheckboxLabel {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            css_id: uuid::Uuid::new_v4().to_string(),
            checked: ctx
                .props()
                .updating_checked
                .unwrap_or(ctx.props().initially_checked),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(checked) = ctx.props().updating_checked {
            self.checked = checked;
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                let checked = !self.checked;
                self.checked = checked;
                if let Some(callback) = &ctx.props().oncheck {
                    callback.emit(checked);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <input
                    id={self.css_id.clone()}
                    type="checkbox"
                    checked={self.checked}
                    onclick={ctx.link().callback(|_| Msg::Toggle)}
                    />
                <label for={self.css_id.clone()}>
                    {ctx.props().label.as_str()}
                </label>
            </>
        }
    }
}
