use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};
use yew::{prelude::*, html};

pub fn callback_by_value<E: JsCast>(cb: &Callback<String>) -> Callback<E> {
    use gloo::console::error;
    let cb = cb.clone();
    Callback::from(move |event: E| {
        if let Ok(event) = event.dyn_into::<Event>() {
            if let Some(target) = event.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    cb.emit(input.value());
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

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub on_change: Option<Callback<String>>,
}

#[function_component(Input)]
pub fn input_function(props: &InputProps) -> Html {
    let oninput = props.on_change.as_ref().map(callback_by_value);
    return html! {
        <input {oninput}/>
    };
}
