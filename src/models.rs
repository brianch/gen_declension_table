use super::schema::nouns;
use serde_derive::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[derive(Queryable)]
pub struct Noun {
    pub id: i32,

    pub gender: String,
    pub animate: i32,

    pub sing_nominative: String,
    pub sing_genitive: String,
    pub sing_dative: String,
    pub sing_accusative: String,
    pub sing_instrumental: String,
    pub sing_prepositional: String,
    pub sing_locative: String,

    pub plur_nominative: String,
    pub plur_genitive: String,
    pub plur_dative: String,
    pub plur_accusative: String,
    pub plur_instrumental: String,
    pub plur_prepositional: String,
    pub plur_locative: String,
}

impl Noun {
    pub fn new() -> Noun {
        Noun {
            id: 0,
            gender: String::new(),
            animate: 0,
            sing_nominative: String::new(),
            sing_genitive: String::new(),
            sing_dative: String::new(),
            sing_accusative: String::new(),
            sing_instrumental: String::new(),
            sing_prepositional: String::new(),
            sing_locative: String::new(),
        
            plur_nominative: String::new(),
            plur_genitive: String::new(),
            plur_dative: String::new(),
            plur_accusative: String::new(),
            plur_instrumental: String::new(),
            plur_prepositional: String::new(),
            plur_locative: String::new(),
        }
    }
}

#[derive(Insertable)]
#[table_name="nouns"]
pub struct AddNoun<'a> {
    pub gender: &'a str,
    pub animate: &'a i32,

    pub sing_nominative: &'a str,
    pub sing_genitive: &'a str,
    pub sing_dative: &'a str,
    pub sing_accusative: &'a str,
    pub sing_instrumental: &'a str,
    pub sing_prepositional: &'a str,
    pub sing_locative: &'a str,

    pub plur_nominative: &'a str,
    pub plur_genitive: &'a str,
    pub plur_dative: &'a str,
    pub plur_accusative: &'a str,
    pub plur_instrumental: &'a str,
    pub plur_prepositional: &'a str,
    pub plur_locative: &'a str,
}