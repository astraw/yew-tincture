use yew::prelude::*;

pub struct CheckboxLabel {
    css_id: String,
    label: String,
    oncheck: Option<Callback<bool>>,
    checked: bool,
}

pub enum Msg {
    Checked(bool),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub label: String,
    pub oncheck: Option<Callback<bool>>,
    pub checked: bool,
}

impl Component for CheckboxLabel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            css_id: uuid::Uuid::new_v4().to_string(),
            label: props.label,
            oncheck: props.oncheck,
            checked: props.checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Checked(checked) => {
                if let Some(ref mut callback) = self.oncheck {
                    callback.emit(checked);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.label = props.label;
        self.oncheck = props.oncheck;
        self.checked = props.checked;
        true
    }
}

impl Renderable<CheckboxLabel> for CheckboxLabel {
    fn view(&self) -> Html<Self> {
        let new_value = !self.checked;
        // Hmm, putting this in one html!{} macro fails, so make a list
        // manually.

        // Ideally, the structure would be <label><input /></label> but
        // I could not get this to work, so we are using `css_id` here.
        let el1 = html! {
            <input
                id=&self.css_id,
                type="checkbox",
                checked=self.checked,
                onchange=|_| Msg::Checked(new_value),
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
