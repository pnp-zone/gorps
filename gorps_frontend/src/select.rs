use std::marker::PhantomData;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlSelectElement, InputEvent};
use yew::{prelude::*, html, virtual_dom::Key};

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
        let oninput = if let Some(callback) = ctx.props().on_change.as_ref() {
            let callback = callback.clone();
            let options = ctx.props().options.clone();
            Some(Callback::from(move |event: InputEvent| {
                let select: HtmlSelectElement = event
                    .target().expect_throw("Event was fired without target")
                    .dyn_into().expect_throw("Event was fired on something else than <select>");
                let index: usize = select.value().parse().expect_throw("Value is not an usize");
                let value = options.get(index).expect_throw("Index out of range").clone();
                callback.emit(value);
            }))
        } else {None};

        let options = ctx.props().options.iter()
            .enumerate()
            .map(|(index, value)| html! {
                <option key={ value.clone() } value={ index.to_string() } selected={ index == 0 }>{ value.clone() }</option>
            });

        return html! {
            <select {oninput} value="0">
                { for options }
            </select>
        };
    }
}