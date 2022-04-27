use std::default::Default;
use yew::Callback;
use gloo::console::error;

pub mod input;
pub mod select;

/**
 * **Error Logging**
 *
 * When interacting with javascript bindings lots of calls could fail
 * i.e. return an Option or Result.
 *
 * This makes sense since javascript is unsafe, but it is annoying when
 * 1) you known they won't happen and
 * 2) you can't do anything but abort when they do anyway.
 *
 * In javascript you could throw an exception and in a small rust program
 * you could use `.expect()` or `.unwrap()` for that.
 * But in wasm, javascript exceptions would clear the stack
 * without proper dropping on rust's side and rust panics are even worse.
 *
 * For example:
 * ```rust
 * use::yew::html;
 * html! {
 *   <input oninput={
 *     // This event handler takes an event whose target is the <input> tag,
 *     // but `.target()` returns an Option<Element> which needs to be cast
 *     // into a HtmlInputElement producing a Result.
 *     // This should never fail and when it doesn't well abort.
 *   }/>
 * }
 * ```
 *
 * Therefore I adapted a pattern of log and early return,
 * which I made convenient in the following wrappers:
 * - `Callback::log_err` (import `CallbackExtension` trait)
 * - `Closure::log_err`
 * - `Future::log_err`
 */

// Trait to inject methods to yew's callback struct
pub trait CallbackExtension<IN> {
    fn log_err<OK, F>(func: F) -> Self
    where
        F: Fn(IN) -> Result<OK, &'static str>,
        F: 'static,
    ;
}

impl <IN> CallbackExtension<IN> for Callback<IN> {
    /**
     * Wrap a function which returns a Result into a callback
     * which logs an error and just discards a success
     * (Since callbacks don't have return values).
     */
    fn log_err<OK, F>(func: F) -> Callback<IN>
    where
        F: Fn(IN) -> Result<OK, &'static str>,
        F: 'static,
    {
        Callback::from(move |arg| {
            if let Err(error) = func(arg) {
                error!(error);
            }
        })
    }
}

pub struct Closure;
impl Closure {
    /**
     * Wrap a closure / lambda which returns a Result into one which doesn't
     * by logging an error and returning the default success value.
     */
    pub fn log_err<IN, OUT, F>(func: F) -> impl Fn(IN) -> OUT
    where
        F: Fn(IN) -> Result<OUT, &'static str>,
        F: 'static,
        OUT: Default,
    {
        move |input| {
            match func(input) {
                Ok(output) => output,
                Err(err) => {
                    error!(err);
                    Default::default()
                }
            }
        }
    }
}

pub struct Future;
impl Future {
    /**
     * Wrap a future which returns a Result into one which doesn't
     * by logging an error and returning the default success value.
     */
    pub fn log_err<OK, F>(fut: F) -> impl std::future::Future<Output = OK>
    where
        F: std::future::Future<Output = Result<OK, &'static str>>,
        F: 'static,
        OK: Default,
    {
        async move {
            match fut.await {
                Ok(ok) => ok,
                Err(err) => {
                    error!(err);
                    Default::default()
                }
            }
        }
    }
}