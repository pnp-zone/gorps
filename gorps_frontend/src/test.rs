use yew::{prelude::*, html};

pub struct Test;
impl Component for Test {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Test }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let class = "baz";
        return html! {
            <p id="foo" {baz}>{"bar"}</p>
        };
    }
}

pub fn main() {
    yew::start_app::<Test>();
}
