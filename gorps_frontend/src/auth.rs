use gloo::net::http::{Method::POST, Request};
use wasm_bindgen::JsCast;
use web_sys::{Element, FocusEvent, HtmlFormElement, HtmlInputElement};
use yew::{prelude::*, html};
use serde::Serialize;

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
                ctx.link().send_future(async move {
                    let req = Request::new("/api/v1/login")
                        .method(POST)
                        .json(&user)
                        .unwrap(); // TODO error handling

                    let res = req.send()
                        .await
                        .unwrap(); // TODO error handling

                    if res.ok() {
                        Success(user)
                    } else {
                        Error(user)
                    }
                });
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
        let onsubmit = ctx.link().batch_callback(|event: FocusEvent| {
            use gloo::console::error;

            event.prevent_default();

            let target: Option<_> = event.target();
            if target.is_none() {error!("Event was thrown without a target.");}

            let form: Option<HtmlFormElement> = target?.dyn_into().ok();
            if form.is_none() {error!("Event wasn't thrown on a form.");}

            let inputs = form?.elements();
            let to_input: fn (Element) -> Option<HtmlInputElement> = |elem| {elem.dyn_into().ok()};
            let username: Option<_> = inputs.named_item("username").map(to_input).flatten();
            let password: Option<_> = inputs.named_item("password").map(to_input).flatten();
            if username.is_none() {error!("Form doesn't have a \"username\" input");}
            if password.is_none() {error!("Form doesn't have a \"password\" input");}

            Some(LoginMsg::Try(User {username: username?.value(), password: password?.value()}))
        });
        return html!{
            <form {onsubmit}>
                <p><label>{"Username: "}<input type="text" name="username"/></label></p>
                <p><label>{"Password: "}<input type="password" name="password"/></label></p>
                <p><input type="submit"/></p>
            </form>
        };
    }
}