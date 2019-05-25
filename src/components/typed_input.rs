use yew::prelude::*;

pub struct TypedInputStorage<T>
    where
        T: std::str::FromStr,
{
    raw_value: String,
    parsed: Result<T, <T as std::str::FromStr>::Err>,
}

impl<T> Default for TypedInputStorage<T>
    where
        T: std::str::FromStr,
{
    fn default() -> Self {
        let raw_value = "".to_string();
        let parsed = raw_value.parse();
        Self {
            raw_value,
            parsed,
        }
    }
}

impl<T> TypedInputStorage<T>
    where
        T: std::fmt::Display + std::str::FromStr,
{
    pub fn from_initial(initial: T) -> Self {
        let raw_value = format!("{}", initial);
        let parsed = Ok(initial);
        Self {
            raw_value,
            parsed,
        }
    }
}

impl<T> TypedInputStorage<T>
    where
        T: std::str::FromStr,
{
    /// Create a TypedInputStorage with a specific value.
    ///
    /// Note that it may be better to not specific an initial value. In that
    /// case, use `default()`.
    pub fn new(value: T) -> Self
        where
            T: std::fmt::Display,
    {
        Self {
            raw_value: format!("{}", value),
            parsed: Ok(value),
        }
    }
    pub fn value(&self) -> &str {
        &self.raw_value
    }
    pub fn parsed(&self) -> &Result<T, <T as std::str::FromStr>::Err> {
        &self.parsed
    }
}

impl<T> From<&str> for TypedInputStorage<T>
    where
        T: std::str::FromStr,
{
    fn from(orig: &str) -> Self {
        let raw_value = orig.to_string();
        let parsed = raw_value.parse();
        Self { raw_value, parsed }
    }
}

pub struct TypedInput<'a,T>
    where
        T: std::str::FromStr,
{
    value: &'a str,
    placeholder: String,
    on_send_valid: Option<Callback<T>>,
    on_input: Option<Callback<TypedInputStorage<T>>>,
}

pub enum Msg {
    NewValue(String),
    SendValue,
    Ignore,
}

#[derive(PartialEq, Clone)]
pub struct Props<'a, T>
    where
        T: std::str::FromStr,
{
    pub value: &'a str,
    pub placeholder: String,
    pub on_send_valid: Option<Callback<T>>,
    pub on_input: Option<Callback<TypedInputStorage<T>>>,
}

impl<'a, T> Default for Props<'a, T>
    where
        T: std::str::FromStr,
{
    fn default() -> Self {
        Props {
            value: "",
            placeholder: "".to_string(),
            on_send_valid: None,
            on_input: None,
        }
    }
}

impl<T> Component for TypedInput<'static,T>
    where
        T: 'static + Clone + PartialEq + std::str::FromStr,
        Result<T, <T as std::str::FromStr>::Err>: Clone,
{
    type Message = Msg;
    type Properties = Props<'static,T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            value: props.value,
            placeholder: props.placeholder,
            on_send_valid: props.on_send_valid,
            on_input: props.on_input,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewValue(raw_value) => {
                let parsed = raw_value.parse();
                let msg = TypedInputStorage {
                    raw_value,
                    parsed,
                };
                if let Some(ref mut callback) = self.on_input {
                    callback.emit(msg);
                }
            }
            Msg::SendValue => {

                if let Some(ref mut callback) = self.on_send_valid {
                    if let Ok(value) = self.value.parse() {
                        callback.emit(value);
                    }
                }
                return false; // no need to rerender DOM
            }
            Msg::Ignore => {
                return false; // no need to rerender DOM
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        self.placeholder = props.placeholder;
        self.on_send_valid = props.on_send_valid;
        true
    }
}

impl<T> Renderable<TypedInput<'static,T>> for TypedInput<'static,T>
    where
        T: 'static + Clone + PartialEq + std::str::FromStr,
        Result<T, <T as std::str::FromStr>::Err>: Clone,
{
    fn view(&self) -> Html<Self> {
        let tmp: Result<T, <T as std::str::FromStr>::Err> = self.value.parse();
        let input_class = match tmp {
            Ok(_) => "ranged-value-input",
            Err(_) => "ranged-value-input-error",
        };
        html! {
            <input type="text",
                class=input_class,
                placeholder=&self.placeholder,
                value=self.value,
                oninput=|e| Msg::NewValue(e.value),
                onblur=|_| Msg::SendValue,
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::SendValue } else { Msg::Ignore }
                },
                />
        }
    }
}
