use gloo::net::http::{Method::POST, Request};
use wasm_bindgen::JsCast;
use web_sys::{Element, FocusEvent, HtmlFormElement, HtmlInputElement};
use yew::{prelude::*, html};
use serde::Serialize;
use crate::util::{Closure, Future};

pub struct Login;
#[derive(Properties, PartialEq)]
pub struct LoginProps {
    #[prop_or_default] pub callback: Option<Callback<String>>,
}
pub enum LoginMsg {
    Try(User),
    Success(User),
    Error(User),
}

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = LoginProps;

    fn create(_ctx: &Context<Self>) -> Self { Login }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use LoginMsg::*;
        match msg {
            Try(user) => {
                ctx.link().send_future_batch(Future::log_err(async move {
                    let req = Request::new("/api/v1/login")
                        .method(POST)
                        .json(&user)
                        .map_err(|_| "Couldn't create request")?;

                    let res = req.send()
                        .await
                        .map_err(|_| "Couldn't send request")?;

                    let msg = if !res.ok() {
                        Success(user)
                    } else {
                        Error(user)
                    };
                    Ok(vec![msg])
                }));
            },
            Success(user) => {
                if let Some(callback) = ctx.props().callback.as_ref() {
                    callback.emit(user.username);
                }
            }
            Error(user) => {
                use gloo::console::error;
                error!("Couldn't log in as", user.username);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().batch_callback(Closure::log_err(|event: FocusEvent| {
            event.prevent_default();
            let target = event.target()
                .ok_or_else(|| "Event was thrown without a target.")?;
            let form: HtmlFormElement = target.dyn_into()
                .map_err(|_| "Event wasn't thrown on a form.")?;
            let inputs = form.elements();

            let to_input = |elem: Element| {elem.dyn_into().ok()};
            let username: HtmlInputElement = inputs.named_item("username").map(to_input).flatten()
                .ok_or_else(|| "Form doesn't have a \"username\" input")?;
            let password: HtmlInputElement = inputs.named_item("password").map(to_input).flatten()
                .ok_or_else(|| "Form doesn't have a \"password\" input")?;
            Ok(Some(LoginMsg::Try(User {username: username.value(), password: password.value()})))
        }));
        return html!{
            <form {onsubmit}>
                <p><label>{"Username: "}<input type="text" name="username"/></label></p>
                <p><label>{"Password: "}<input type="password" name="password"/></label></p>
                <p><input type="submit"/></p>
            </form>
        };
    }
}