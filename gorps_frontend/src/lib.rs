use yew::{prelude::*, html};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlInputElement, FocusEvent};
use gloo::net::http::Request;
use gloo::console;

mod select;
mod input;
mod skill;
use skill::{Skill, GCSSkill};
mod skill_table;
use skill_table::SkillTable;

pub struct Main {
    pub skills: Vec<Skill>,
}
pub enum MainMsg {
    RequestSkills(String),
    ReceiveSkills(Vec<GCSSkill>),
}
impl Component for Main {
    type Message = MainMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Main {
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<Main>();
}

