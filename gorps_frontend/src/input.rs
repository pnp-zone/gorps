use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use yew::{prelude::*, html};

pub struct Input;
#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub on_change: Option<Callback<String>>,
}

impl Component for Input {
    type Message = ();
    type Properties = InputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Input
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = if let Some(callback) = ctx.props().on_change.as_ref() {
            let callback = callback.clone();
            Some(Callback::from(move |event: InputEvent| {
                let select: HtmlInputElement = event
                    .target().expect_throw("Event was fired without target")
                    .dyn_into().expect_throw("Event was fired on something else than <input>");
                let value: String = select.value();
                callback.emit(value);
            }))
        } else {None};

        return html! {
            <input {oninput}/>
        };
    }
}