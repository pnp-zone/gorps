use std::collections::HashSet;
use std::ops::Deref;
use std::str::FromStr;
use once_cell::unsync::Lazy;
use serde::{Serialize, Deserialize};

/*
 * Trait for things which have a string representation in the binary
 */
pub trait ToStr {
    fn to_str(&self) -> &'static str;
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Average,
    Hard,
    VeryHard,
}
impl ToStr for Difficulty {
    fn to_str(&self) -> &'static str {
        use Difficulty::*;
        match self {
            Easy => "Easy",
            Average => "Average",
            Hard => "Hard",
            VeryHard => "Very Hard",
        }
    }
}
impl FromStr for Difficulty {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Difficulty::*;
        match s {
            "e"  |  "E" => Ok(Easy),
            "a"  |  "A" => Ok(Average),
            "h"  |  "H" => Ok(Hard),
            "vh" | "VH" => Ok(VeryHard),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Attribute {
    Strength,
    Dexterity,
    Intelligence,
    Health,
    Perception,
    Willpower,
}
impl ToStr for Attribute {
    fn to_str(&self) -> &'static str {
        use Attribute::*;
        match self {
            Strength => "ST",
            Dexterity => "DX",
            Intelligence => "IQ",
            Health => "HT",
            Perception => "Per",
            Willpower => "Will",
        }
    }
}
impl FromStr for Attribute {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Attribute::*;
        match s {
            "st"   | "ST"   => Ok(Strength),
            "dx"   | "DX"   => Ok(Dexterity),
            "iq"   | "IQ"   => Ok(Intelligence),
            "ht"   | "HT"   => Ok(Health),
            "per"  | "Per"  => Ok(Perception),
            "will" | "Will" => Ok(Willpower),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Optional<T>(Option<T>);
impl <T> Deref for Optional<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl <T: ToStr> ToStr for Optional<T> {
    fn to_str(&self) -> &'static str {
        match self.0.as_ref() {
            None => "--",
            Some(t) => t.to_str(),
        }
    }
}
impl <T> From<Option<T>> for Optional<T> {
    fn from(o: Option<T>) -> Self {
        Optional(o)
    }
}
impl <T> From<Optional<T>> for Option<T> {
    fn from(o: Optional<T>) -> Self {
        o.0
    }
}

static mut CATEGORIES: Lazy<HashSet<String>> = Lazy::new(HashSet::new);
pub fn get_categories() -> Vec<&'static str> {
    unsafe {
        CATEGORIES.iter()
            .map(|string| &string[..])
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub specialization: Option<String>,
    pub difficulty: Optional<Difficulty>,
    pub attribute: Optional<Attribute>,
    pub reference: String,
    //pub points: f64,
    //pub defaults: Vec<!>
    pub categories: Vec<&'static str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GCSSkill {
    pub id: String,
    pub name: String,
    pub specialization: Option<String>,
    pub difficulty: String,
    pub reference: String,
    pub categories: Vec<String>,
}

impl From<GCSSkill> for Skill {
    fn from(skill: GCSSkill) -> Self {
        let mut diff_attr: Vec<&str> = skill.difficulty.splitn(2, "/").collect();
        diff_attr.reverse();

        Skill {
            id: skill.id,
            name: skill.name,
            specialization: skill.specialization,
            difficulty: diff_attr.get(0)
                .map(Deref::deref)
                .map(Difficulty::from_str)
                .map(Result::ok)
                .flatten()
                .into(),
            attribute: diff_attr.get(1)
                .map(Deref::deref)
                .map(Attribute::from_str)
                .map(Result::ok)
                .flatten()
                .into(),
            reference: skill.reference,
            categories: skill.categories.iter()
                .map(|string| {
                    unsafe {
                        if let Some(string) = CATEGORIES.get(string) {
                            string
                        } else {
                            CATEGORIES.insert(string.clone());
                            let string: &'static str = CATEGORIES.get(string).unwrap();
                            wasm_bindgen::intern(string) // Intern cause why not
                        }
                    }
                })
                .collect(),
        }
    }
}