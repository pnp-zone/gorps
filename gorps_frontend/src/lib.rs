use yew::{prelude::*, html};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlInputElement, FocusEvent};
use gloo::net::http::Request;
use gloo::console;

pub mod util;
pub mod skill;
pub mod skill_table;
pub mod auth;

use skill::{Skill, GCSSkill};
use skill_table::SkillTable;
use auth::Login;

pub struct Main {
    pub username: Option<String>,
    pub skills: Vec<Skill>,
}
pub enum MainMsg {
    Login(String),
    RequestSkills(String),
    ReceiveSkills(Vec<GCSSkill>),
}
impl Component for Main {
    type Message = MainMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Main {
            username: None,
            skills: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use MainMsg::*;
        match msg {
            RequestSkills(url) => {
                let fut = async move {
                    use console::error;
                    match Request::get(&url).send().await {
                        Ok(res) => match res.binary().await {
                            Ok(text) => match serde_json::from_reader(&text[..]) {
                                Ok(skills) => return vec![ReceiveSkills(skills)],
                                Err(_) => error!("Couldn't parse response's body"),
                            },
                            Err(_) => error!("Couldn't read response's body"),
                        },
                        Err(_) => error!("Couldn't send request"),
                    }
                    return Vec::new();
                };
                ctx.link().send_future_batch(fut);
                false
            }
            ReceiveSkills(skills) => {
                let skills: Vec<Skill> = skills.into_iter().map(Skill::from).collect();
                self.skills.extend_from_slice(&skills);
                true
            }
            Login(username) => {self.username = Some(username); true}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(username) = self.username.as_ref() {
            return html! {
                <>
                    <form onsubmit={ctx.link().callback(|event: FocusEvent| {
                        let input: HtmlInputElement = js_sys::Reflect::get(
                            &event.target().expect("Event has to fire on something"),
                            &"0".into(),
                        ).expect_throw("The form has a field")
                        .dyn_into()
                        .expect_throw("The form's 1st field is an input");
                        event.prevent_default();
                        let url = input.value();
                        input.set_value("");
                        MainMsg::RequestSkills(url)
                    })}>
                        <input name="url" value="/static/sample.skl"/>
                    </form>
                    <SkillTable skills={self.skills.clone()}/>
                </>
            };
        } else {
            return html! {
                <Login callback={ctx.link().callback(MainMsg::Login)}/>
            };
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<Main>();
}

