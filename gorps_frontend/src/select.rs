use std::marker::PhantomData;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{prelude::*, html, virtual_dom::Key};

pub fn callback_by_option<T: Clone + 'static, E: JsCast>(callback: &Callback<T>, options: &Vec<T>) -> Callback<E> {
    use gloo::console::error;
    let callback = callback.clone();
    let options = options.clone();
    Callback::from(move |event: E| {
        if let Ok(event) = event.dyn_into::<Event>() {
            if let Some(target) = event.target() {
                if let Ok(input) = target.dyn_into::<HtmlSelectElement>() {
                    if let Ok(index) = input.value().parse::<usize>() {
                        if let Some(value) = options.get(index) {
                            callback.emit(value.clone());
                        } else {
                            error!("Select's index is out of range. Seems like the options weren't generated properly.")
                        }
                    } else {
                        error!("Select's value wasn't an index. Seems like the options weren't generated properly");
                    }
                } else {
                    error!("Event wasn't throw on an input.");
                }
            } else {
                error!("Event was throw without a target.");
            }
        } else {
            error!("Generic parameter is not a subtype of Event.")
        }
    })
}

pub fn options_from_vec<'a, T>(vec: &'a Vec<T>) -> impl Iterator<Item=Html> + 'a
    where
        T: Clone + 'static + ToString,
        Key: From<T>,
{
    vec.iter()
        .enumerate()
        .map(|(index, value)| html! {
            <option key={ value.clone() } value={ index.to_string() } selected={ index == 0 }>{ value.clone() }</option>
        })
}

#[derive(Properties)]
pub struct SelectProps<T> {
    pub options: Vec<T>,
    pub on_change: Option<Callback<T>>,
}
impl <T> PartialEq for SelectProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.options.len() == other.options.len() && self.on_change == other.on_change
    }
}

#[derive(Default)]
pub struct Select<T> {
    options: PhantomData<T>,
}

impl <T> Component for Select<T>
where
    T: Clone + 'static + ToString,
    Key: From<T>,
{
    type Message = ();
    type Properties = SelectProps<T>;

    fn create(_: &Context<Self>) -> Self {
        Self { options: PhantomData, }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.props().on_change.as_ref().map(|cb| callback_by_option(cb, &ctx.props().options));
        let options = options_from_vec(&ctx.props().options);
        return html! {
            <select {oninput}>
                { for options }
            </select>
        };
    }
}
