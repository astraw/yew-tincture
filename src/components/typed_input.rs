use web_sys::HtmlInputElement;
use yew::{
    events::KeyboardEvent, html, Callback, Component, Context, Html, InputEvent, Properties,
    TargetCast,
};

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::str::FromStr;

// TODO: reconsider and rework this as a result of the Components API change in
// yew 0.19. In particular: do we need to keep a clone of the `storage` field?

#[derive(Clone)]
pub struct RawAndParsed<T>
where
    T: FromStr + Clone,
    <T as FromStr>::Err: Clone,
{
    raw_value: String,
    parsed: Result<T, <T as FromStr>::Err>,
}

impl<T> RawAndParsed<T>
where
    T: FromStr + Clone,
    <T as FromStr>::Err: Clone,
{
    /// Create a RawAndParsed with empty value.
    fn empty() -> Self
    where
        T: std::fmt::Display,
    {
        let raw_value = "".to_string();
        let parsed = raw_value.parse();
        Self { raw_value, parsed }
    }

    /// Create a RawAndParsed with a good value.
    fn from_initial(value: T) -> Self
    where
        T: std::fmt::Display,
    {
        let raw_value = format!("{}", value);
        let parsed = Ok(value);
        Self { raw_value, parsed }
    }
}

fn new_focus_state() -> Rc<Cell<FocusState>> {
    Rc::new(Cell::new(FocusState::IsBlurred))
}

#[derive(PartialEq, Clone)]
pub struct TypedInputStorage<T>
where
    T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
    <T as FromStr>::Err: Clone,
{
    rc: Rc<RefCell<TypedInputStorageInner<T>>>,
}

impl<T> Default for TypedInputStorage<T>
where
    T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
    <T as FromStr>::Err: Clone,
{
    fn default() -> Self {
        TypedInputStorage::empty()
    }
}

impl<T> TypedInputStorage<T>
where
    T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
    <T as FromStr>::Err: Clone,
    Result<T, <T as FromStr>::Err>: Clone,
{
    /// Create a TypedInputStorage with an empty value.
    pub fn empty() -> Self
    where
        T: std::fmt::Display,
    {
        Self::create(RawAndParsed::empty())
    }

    /// Create a TypedInputStorage with an empty value.
    pub fn from_initial(value: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::create(RawAndParsed::from_initial(value))
    }

    /// Create a TypedInputStorage with an empty value.
    fn create(raw_and_parsed: RawAndParsed<T>) -> Self
    where
        T: std::fmt::Display,
    {
        let inner = TypedInputStorageInner {
            raw_and_parsed,
            focus_state: new_focus_state(),
            link: None,
        };
        let me = Rc::new(RefCell::new(inner));
        Self { rc: me }
    }

    pub fn parsed(&self) -> Result<T, <T as FromStr>::Err> {
        self.rc.borrow().raw_and_parsed.parsed.clone()
    }

    fn set_link(&mut self, link: yew::html::Scope<TypedInput<T>>) {
        let mut inner = self.rc.borrow_mut();
        inner.link = Some(link);
    }

    /// Modify the value.
    ///
    /// See also the [Self::set_if_not_focused] method.
    pub fn modify<F>(&mut self, f: F) -> Result<(), ()>
    where
        F: Fn(&mut T),
        T: std::fmt::Display,
    {
        let mut inner = self.rc.borrow_mut();
        let raw_value = match &mut inner.raw_and_parsed.parsed {
            Ok(value) => {
                f(value);
                format!("{}", value)
            }
            Err(_) => {
                return Err(());
            }
        };
        if let Some(link) = &inner.link {
            link.send_message(Msg::NewValue(raw_value));
        }
        Ok(())
    }

    /// Get the value.
    pub fn get(&self) -> Result<T, <T as FromStr>::Err> {
        self.rc.borrow_mut().raw_and_parsed.parsed.clone()
    }

    /// Update the value if the user is not editing it.
    ///
    /// See also the [Self::set] method.
    pub fn set_if_not_focused(&mut self, value: T)
    where
        T: std::fmt::Display,
    {
        use std::ops::Deref;
        {
            let mut inner = self.rc.borrow_mut();

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
    T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
    <T as FromStr>::Err: Clone,
{
    raw_and_parsed: RawAndParsed<T>,
    // TODO: does this need to be Rc<Cell<_>> or can I make it &'a _?
    focus_state: Rc<Cell<FocusState>>,
    link: Option<yew::html::Scope<TypedInput<T>>>,
}

impl<T> PartialEq for TypedInputStorageInner<T>
where
    T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
    <T as FromStr>::Err: Clone,
{
    fn eq(&self, rhs: &Self) -> bool {
        // I am not sure when yew uses this. Here is my
        // best effort implementation.
        Rc::ptr_eq(&self.focus_state, &rhs.focus_state)
            && self.raw_and_parsed.raw_value == rhs.raw_and_parsed.raw_value
        // TODO: compare link?
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
enum FocusState {
    IsFocused,
    #[default]
    IsBlurred,
}

pub struct TypedInput<T>
where
    T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
    <T as FromStr>::Err: Clone,
{
    raw_value_copy: String, // TODO: can we remove this and just use storage?
    storage: TypedInputStorage<T>,
}

pub enum Msg {
    NewValue(String),
    OnFocus,
    OnBlur,
    SendValueIfValid,
    Ignore,
}

#[derive(PartialEq, Properties)]
pub struct Props<T>
where
    T: FromStr + Clone + PartialEq + std::fmt::Display + 'static,
    <T as FromStr>::Err: Clone,
{
    /// The backing store for the data.
    pub storage: TypedInputStorage<T>,
    /// The placeholder text displayed when the input field is blank.
    #[prop_or_default]
    pub placeholder: String,
    /// Called when the user wants to send a valid value
    #[prop_or_default]
    pub on_send_valid: Option<Callback<T>>,
    /// Called whenever the user changes the value
    #[prop_or_default]
    pub on_input: Option<Callback<RawAndParsed<T>>>,
}

impl<T> TypedInput<T>
where
    T: std::fmt::Display + FromStr + PartialEq + Clone,
    <T as FromStr>::Err: Clone,
{
    fn send_value_if_valid(&mut self, on_send_valid: Option<&Callback<T>>) {
        if let Some(callback) = on_send_valid {
            if let Ok(value) = &self.storage.rc.borrow().raw_and_parsed.parsed {
                callback.emit(value.clone());
            }
        }
    }
}

impl<T> Component for TypedInput<T>
where
    T: 'static + Clone + PartialEq + FromStr + std::fmt::Display,
    <T as FromStr>::Err: Clone,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let raw_value_copy = ctx
            .props()
            .storage
            .rc
            .borrow()
            .raw_and_parsed
            .raw_value
            .clone();
        let mut storage = ctx.props().storage.clone();
        let link: yew::html::Scope<TypedInput<T>> = ctx.link().clone();
        storage.set_link(link);
        Self {
            raw_value_copy,
            storage,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewValue(raw_value) => {
                self.raw_value_copy = raw_value.clone();
                let parsed = raw_value.parse();
                let stor2 = {
                    let mut stor = self.storage.rc.borrow_mut();
                    stor.raw_and_parsed.raw_value = raw_value;
                    stor.raw_and_parsed.parsed = parsed;
                    stor.raw_and_parsed.clone()
                };

                if let Some(ref callback) = &ctx.props().on_input {
                    callback.emit(stor2);
                }
            }
            Msg::OnFocus => {
                let stor = self.storage.rc.borrow_mut();
                stor.focus_state.replace(FocusState::IsFocused);
            }
            Msg::OnBlur => {
                {
                    let stor = self.storage.rc.borrow_mut();
                    stor.focus_state.replace(FocusState::IsBlurred);
                }
                self.send_value_if_valid(ctx.props().on_send_valid.as_ref());
            }
            Msg::SendValueIfValid => {
                self.send_value_if_valid(ctx.props().on_send_valid.as_ref());
                return false;
            }
            Msg::Ignore => {
                return false; // no need to rerender DOM
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_class = match &self.storage.rc.borrow().raw_and_parsed.parsed {
            Ok(_) => "ranged-value-input",
            Err(_) => "ranged-value-input-error",
        };

        html! {
            <input type="text"
                class={input_class}
                placeholder={ctx.props().placeholder.clone()}
                value={self.raw_value_copy.clone()}
                oninput={ctx.link().callback(|e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    Msg::NewValue(input.value())
                })}
                onfocus={ctx.link().callback(|_| Msg::OnFocus)}
                onblur={ctx.link().callback(|_| Msg::OnBlur)}
                onkeypress={ctx.link().callback(|e: KeyboardEvent| {
                    if e.key() == "Enter" { Msg::SendValueIfValid } else { Msg::Ignore }
                })}
                />
        }
    }
}
