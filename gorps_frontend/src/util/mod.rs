use yew::Callback;
use gloo::console::error;

pub mod input;
pub mod select;

/**
 * Trait to add methods to yew's callback
 */
pub trait CallbackExtension<IN> {
    /**
     * Wrap a function returning a Result into a callback which logs an error to console.
     */
    fn log_err<OK, F>(func: F) -> Self
    where
        F: Fn(IN) -> Result<OK, &'static str>,
        F: 'static,
    ;
}

impl <IN> CallbackExtension<IN> for Callback<IN> {
    fn log_err<OK, F>(func: F) -> Self
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
