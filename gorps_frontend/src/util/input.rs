use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};
use yew::prelude::*;

use crate::util::CallbackExtension;

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
    let cb = cb.clone();
    Callback::log_err(move |event: E| {
        let event: Event = event.dyn_into()
            .map_err(|_| "Generic parameter is not a subtype of Event.")?;
        let target = event.target()
            .ok_or_else(|| "Event was throw without a target.")?;
        let input: HtmlInputElement =  target.dyn_into()
            .map_err(|_| "Event wasn't thrown on an input.")?;
        cb.emit(input.value());
        Ok(())
    })
}
