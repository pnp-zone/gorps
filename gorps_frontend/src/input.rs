use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};
use yew::prelude::*;

/**
 * Wraps a callback taking a String to take a dom event instead.
 *
 * The event's target is cast into an \<input\> whose value is passed to the wrapped callback.
 *
 * Example:
 * ```rust
 * use yew::html;
 * use gorps_frontend::input::callback_by_value;
 *
 * html! {
 *     <input oninput={callback_by_value(Callback::from(|string| todo!()))}>
 * }
 * ```
 */
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
