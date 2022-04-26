use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};
use yew::prelude::*;
use gloo::console::error;

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
    Callback::from(move |event: E| { (|| {
        let event: Option<Event> = event.dyn_into().ok();
        if event.is_none() { error!("Generic parameter is not a subtype of Event."); }

        let target: Option<_> = event?.target();
        if target.is_none() { error!("Event was throw without a target."); }

        let input: Option<HtmlInputElement> = target?.dyn_into().ok();
        if input.is_none() { error!("Event wasn't thrown on an input."); }

        cb.emit(input?.value());
        Some(())
    })(); })
}
