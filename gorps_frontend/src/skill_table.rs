use yew::{prelude::*, html, virtual_dom::Key};
use crate::skill::{ToStr, Skill, Optional, Attribute, Difficulty, get_categories};
use crate::select::Select;
use crate::input::Input;

macro_rules! derive_from_to_str {
    ($T:path) => {
        impl From<$T> for &'static str {
            fn from(t: $T) -> Self {
                t.to_str()
            }
        }
        impl ToString for $T {
            fn to_string(&self) -> String {
                self.to_str().to_string()
            }
        }
        impl From<$T> for Key {
            fn from(t: $T) -> Self {
                Key::from(t.to_str())
            }
        }
    };
}
derive_from_to_str!(Difficulty);
derive_from_to_str!(Attribute);
derive_from_to_str!(Optional<Difficulty>);
derive_from_to_str!(Optional<Attribute>);

#[derive(Properties)]
pub struct SkillTableProps {
    #[prop_or_default] pub skills: Vec<Skill>,
}
impl PartialEq for SkillTableProps {
    fn eq(&self, other: &Self) -> bool {
        // only possible update is adding or removing skills
        self.skills.len() == other.skills.len()
    }
}

pub struct SkillTable {
    filter_name: Option<String>,
    filter_attr: Optional<Attribute>,
    filter_diff: Optional<Difficulty>,
    filter_cate: Option<&'static str>,
}

pub enum SkillTableMsg {
    FilterName(String),
    FilterAttr(Optional<Attribute>),
    FilterDiff(Optional<Difficulty>),
    FilterCate(&'static str),
}

impl Component for SkillTable {
    type Message = SkillTableMsg;
    type Properties = SkillTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        SkillTable {
            filter_name: None,
            filter_attr: None.into(),
            filter_diff: None.into(),
            filter_cate: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use SkillTableMsg::*;
        match msg {
            FilterName(name) => self.filter_name = if name.len() > 0 {Some(name)} else {None},
            FilterAttr(attr) => self.filter_attr = attr,
            FilterDiff(diff) => self.filter_diff = diff,
            FilterCate(cate) => self.filter_cate = if cate == "--" {None} else {Some(cate)},
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let filter = |skill: &Skill| (
            if let Some(name) = self.filter_name.as_ref() {
                skill.name.contains(name)
            } else { true }
        ) && (
            if self.filter_diff.is_none() { true } else {
                self.filter_diff == skill.difficulty
            }
        ) && (
            if self.filter_attr.is_none() { true } else {
                self.filter_attr == skill.attribute
            }
        ) && (
            if let Some(category) = self.filter_cate.as_ref() {
                skill.categories.contains(category)
            } else {true}
        );
        let skills = ctx.props().skills.iter()
            .map(|skill| {
                let class = if filter(skill) {None} else {Some("hidden")};
                let categories = skill.categories.iter()
                    .map(|&str| html!{<span>{ str }</span>});
                html! {
                    <tr key={ &skill.id[..] } { class }>
                        <td>{
                            if let Some(sp) = skill.specialization.as_ref() {format!("{} ({})", &skill.name, sp)}
                            else { skill.name.clone() }
                        }</td>
                        <td>{ skill.attribute }</td>
                        <td>{ skill.difficulty }</td>
                        <td>{ for categories }</td>
                        <td>{ &skill.reference[..] }</td>
                    </tr>
                }
            });
        let mut categories = get_categories();
        categories.insert(0, "--");

        return html! {
            <table class="skills row-border-hover">
                <thead class="stick-top">
                    <tr>
                        <th>{"Name"}</th>
                        <th>{"Attribute"}</th>
                        <th>{"Difficulty"}</th>
                        <th>{"Categories"}</th>
                        <th>{"Reference"}</th>
                    </tr>
                    <tr>
                        <th><Input on_change={ ctx.link().callback(SkillTableMsg::FilterName) }/></th>
                        <th><Select<Optional<Attribute>>
                            options={vec![
                                None.into(),
                                Some(Attribute::Strength).into(),
                                Some(Attribute::Dexterity).into(),
                                Some(Attribute::Intelligence).into(),
                                Some(Attribute::Health).into(),
                                Some(Attribute::Perception).into(),
                                Some(Attribute::Willpower).into(),
                            ]}
                            on_change={ ctx.link().callback(SkillTableMsg::FilterAttr) }
                        /></th>
                        <th><Select<Optional<Difficulty>>
                            options={vec![
                                None.into(),
                                Some(Difficulty::Easy).into(),
                                Some(Difficulty::Average).into(),
                                Some(Difficulty::Hard).into(),
                                Some(Difficulty::VeryHard).into(),
                            ]}
                            on_change={ ctx.link().callback(SkillTableMsg::FilterDiff) }
                        /></th>
                        <th><Select<&'static str> options={categories} on_change={ ctx.link().callback(SkillTableMsg::FilterCate) }/></th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    { for skills }
                </tbody>
            </table>
        };
    }
}
