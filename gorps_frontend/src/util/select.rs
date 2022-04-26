use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{prelude::*, html, virtual_dom::Key};

use crate::util::CallbackExtension;

/**
 * Wraps a callback taking the options' type to take a dom event instead.
 *
 * The event's target is cast into an \<select\> whose value is parsed into an index.
 * This index is then used on the options parameter to retrieve the option
 * which is passed to the wrapped callback.
 *
 * Example:
 * ```rust
 * use yew::html;
 * use gorps_frontend::select::callback_by_option;
 *
 * pub enum Fruits {
 *     Apple,
 *     Banana,
 * }
 *
 * let fruits = vec![Fruits::Apple, Fruits::Banana];
 *
 * html! {
 *     <select oninput={callback_by_option(Callback::from(|fruit| todo!()))}>
 *         <option value="0">{"Apple"}</option>
 *         <option value="1">{"Banana"}</option>
 *     </select>
 * }
 * ```
 *
 * Since the \<option\> s have to match the options parameter, you should generate them using `options_from_slice`.
 */
pub fn callback_by_option<T: Clone + 'static, E: JsCast>(callback: &Callback<T>, options: &[T]) -> Callback<E> {
    let callback = callback.clone();
    let options: Vec<T> = options.into();
    Callback::log_err(move |event: E| {
        let event: Event = event.dyn_into()
            .map_err(|_| "Generic parameter is not a subtype of Event.")?;
        let target = event.target()
            .ok_or_else(|| "Event was throw without a target.")?;
        let select: HtmlSelectElement = target.dyn_into()
            .map_err(|_| "Event wasn't throw on an select.")?;
        let index: usize = select.value().parse()
            .map_err(|_| "Select's value wasn't an index. Seems like the options weren't generated properly")?;
        let value: &T = options.get(index)
            .ok_or_else(|| "Select's index is out of range. Seems like the options weren't generated properly.")?;
        callback.emit(value.clone());
        Ok(())
    })
}

/**
 * Creates a map over a slice turning the items into \<option\> s.
 *
 * A \<option\>'s value is its index in the slice, so it can be handled by `callback_by_option`.
 *
 * Example:
 * ```rust
 * use yew::html;
 * use gorps_frontend::select::options_from_slice;
 *
 * let fruits = ["Apple", "Banana"];
 *
 * html! {
 *     <select>
 *         { for options_from_slice(fruits) }
 *     </select>
 * }
 * ```
 */
pub fn options_from_slice<'a, T>(options: &'a [T]) -> impl Iterator<Item=Html> + 'a
    where
        T: Clone + 'static + ToString,
        Key: From<T>,
{
    options.iter()
        .enumerate()
        .map(|(index, value)| html! {
            <option key={ value.clone() } value={ index.to_string() } selected={ index == 0 }>{ value.clone() }</option>
        })
}
