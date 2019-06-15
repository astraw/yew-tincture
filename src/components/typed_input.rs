use yew::prelude::*;

use std::str::FromStr;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct RawAndParsed<T>
    where
        T: FromStr,
{
    raw_value: String,
    parsed: Result<T, <T as FromStr>::Err>,
}

impl<T> RawAndParsed<T>
    where
        T: FromStr,
{
    /// Create a RawAndParsed with empty value.
    fn empty() -> Self
        where
            T: std::fmt::Display,
    {
        let raw_value = "".to_string();
        let parsed = raw_value.parse();
        Self {
            raw_value,
            parsed,
        }
    }

}

fn new_focus_state() -> Rc<Cell<FocusState>> {
    Rc::new(Cell::new(FocusState::IsBlurred))
}

#[derive(PartialEq,Clone)]
pub struct TypedInputStorage<T: FromStr + Clone> ( Rc<RefCell<TypedInputStorageInner<T>>> );

impl<T> TypedInputStorage<T>
    where
        T: FromStr + Clone,
        Result<T, <T as FromStr>::Err> : Clone,
{
    /// Create a TypedInputStorage with an empty value.
    pub fn empty() -> Self
        where
            T: std::fmt::Display,
    {
        let inner = TypedInputStorageInner {
            raw_and_parsed: RawAndParsed::empty(),
            focus_state: new_focus_state(),
        };
        let me = Rc::new(RefCell::new(inner));
        TypedInputStorage(me)
    }

    pub fn parsed(&self) -> Result<T, <T as FromStr>::Err> {
        self.0.borrow().raw_and_parsed.parsed.clone()
    }

    /// Update the value if the user is not editing it.
    ///
    /// See also the `set()` method.
    pub fn set_if_not_focused(&mut self, value: T)
        where
            T: std::fmt::Display,
    {
        use std::ops::Deref;
        {
            let mut inner = self.0.borrow_mut();

            match (*(inner.focus_state).deref()).get() {
                FocusState::IsFocused => {}
                FocusState::IsBlurred => {
                    inner.raw_and_parsed.raw_value = format!("{}", value);
                    inner.raw_and_parsed.parsed = Ok(value);
                }
            }
        }
    }

}

struct TypedInputStorageInner<T>
    where
        T: FromStr + Clone,
{
    raw_and_parsed: RawAndParsed<T>,
    // TODO: does this need to be Rc<Cell<_>> or can I make it &'a _?
    focus_state: Rc<Cell<FocusState>>,
}

impl<T> PartialEq for TypedInputStorageInner<T>
    where
        T: FromStr + Clone,
{
    fn eq(&self, rhs: &Self) -> bool {
        // I am not sure when yew uses this. Here is my
        // best effort implementation.
        Rc::ptr_eq(&self.focus_state, &rhs.focus_state) &&
            self.raw_and_parsed.raw_value == rhs.raw_and_parsed.raw_value
    }
}

impl<T> TypedInputStorageInner<T>
    where
        T: FromStr + Clone,
{
    // /// Create a TypedInputStorageInner with a specific value.
    // ///
    // /// Note that it may be better to not specific an initial value. In that
    // /// case, use `empty()`.
    // pub fn new(value: T, focus_state: Rc<Cell<FocusState>>) -> Self
    //     where
    //         T: std::fmt::Display,
    // {
    //     Self {
    //         raw_value: format!("{}", value),
    //         parsed: Ok(value),
    //         focus_state: focus_state,
    //     }
    // }

    // /// Create a TypedInputStorageInner with an empty value.
    // pub fn empty() -> Self
    //     where
    //         T: std::fmt::Display,
    // {
    //     Self {
    //         raw_and_parsed: RawAndParsed::empty(),
    //         focus_state: new_focus_state(),
    //     }
    // }

    // pub fn from_str(orig: &str, focus_state: Rc<Cell<FocusState>>) -> Self {
    //     let raw_value = orig.to_string();
    //     let parsed = raw_value.parse();
    //     Self {
    //         raw_value,
    //         parsed,
    //         focus_state: focus_state,
    //     }
    // }

    // /// Update the value
    // ///
    // /// See also the `set_if_not_focused()` method.
    // pub fn set(&mut self, raw_and_parsed: RawAndParsed<T>)
    //     where
    //         T: std::fmt::Display,
    // {
    //     self.raw_and_parsed = raw_and_parsed;
    // }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum FocusState {
    IsFocused,
    IsBlurred,
}

impl Default for FocusState {
    fn default() -> FocusState {
        FocusState::IsBlurred
    }
}

pub struct TypedInput<T>
    where
        T: FromStr + Clone,
{
    raw_value_copy: String, // TODO: can we remove this and just use storage?
    storage: TypedInputStorage<T>,
    placeholder: String,
    on_send_valid: Option<Callback<T>>,
    on_input: Option<Callback<RawAndParsed<T>>>,
}

pub enum Msg {
    NewValue(String),
    OnFocus,
    OnBlur,
    SendValueIfValid,
    Ignore,
}

#[derive(PartialEq, Clone)]
pub struct Props<T>
    where
        T: FromStr + Clone,
{
    pub storage: TypedInputStorage<T>,
    pub placeholder: String,
    /// Called when the user wants to send a valid value
    pub on_send_valid: Option<Callback<T>>,
    /// Called whenever the user changes the value
    pub on_input: Option<Callback<RawAndParsed<T>>>,
}

impl<T> Default for Props<T>
    where
        T: FromStr + Clone + std::fmt::Display,
        <T as FromStr>::Err: Clone,
{
    fn default() -> Self {
        Props {
            storage: TypedInputStorage::empty(),
            placeholder: "".to_string(),
            on_send_valid: None,
            on_input: None,
        }
    }
}

impl<T> TypedInput<T>
    where
        T: FromStr + Clone + Clone,
{
    fn send_value_if_valid(&mut self) {
        if let Some(ref mut callback) = self.on_send_valid {
            if let Ok(value) = &self.storage.0.borrow().raw_and_parsed.parsed {
                callback.emit(value.clone());
            }
        }
    }
}

impl<T> Component for TypedInput<T>
    where
        T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
        <T as FromStr>::Err: Clone,
        Result<T, <T as FromStr>::Err>: Clone,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let raw_value_copy = props.storage.0.borrow().raw_and_parsed.raw_value.clone();
        Self {
            raw_value_copy,
            storage: props.storage,
            placeholder: props.placeholder,
            on_send_valid: props.on_send_valid,
            on_input: props.on_input,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.raw_value_copy = props.storage.0.borrow().raw_and_parsed.raw_value.clone();
        self.storage = props.storage;
        self.placeholder = props.placeholder;
        self.on_send_valid = props.on_send_valid;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewValue(raw_value) => {
                let raw_value2 = raw_value.clone();
                self.raw_value_copy = raw_value.clone();
                let parsed = raw_value.parse();
                let parsed2 = parsed.clone();
                {
                    let mut stor = self.storage.0.borrow_mut();
                    stor.raw_and_parsed.raw_value = raw_value;
                    stor.raw_and_parsed.parsed = parsed;
                }

                if let Some(ref mut callback) = self.on_input {
                    let stor2 = RawAndParsed {
                        raw_value: raw_value2,
                        parsed: parsed2,
                    };

                    callback.emit(stor2);
                }

            }
            Msg::OnFocus => {
                let stor = self.storage.0.borrow_mut();
                stor.focus_state.replace(FocusState::IsFocused);
                return true;
            }
            Msg::OnBlur => {
                {
                    let stor = self.storage.0.borrow_mut();
                    stor.focus_state.replace(FocusState::IsBlurred);
                }
                self.send_value_if_valid();
                return true;
            }
            Msg::SendValueIfValid => {
                self.send_value_if_valid();
                return false;
            }
            Msg::Ignore => {
                return false; // no need to rerender DOM
            }
        }
        true
    }
}

impl<T> Renderable<TypedInput<T>> for TypedInput<T>
    where
        T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
        <T as FromStr>::Err: Clone,
        Result<T, <T as FromStr>::Err>: Clone,
{
    fn view(&self) -> Html<Self> {
        let input_class = match &self.storage.0.borrow().raw_and_parsed.parsed {
            Ok(_) => "ranged-value-input",
            Err(_) => "ranged-value-input-error",
        };

        html! {
            <input type="text",
                class=input_class,
                placeholder=&self.placeholder,
                value=&self.raw_value_copy,
                oninput=|e| Msg::NewValue(e.value),
                onfocus=|_| Msg::OnFocus,
                onblur=|_| Msg::OnBlur,
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::SendValueIfValid } else { Msg::Ignore }
                },
                />
        }
    }
}
