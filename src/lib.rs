pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{AddNoun};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))

}

pub fn add_noun<'a>(conn: &SqliteConnection,
                    gender: &'a str,
                    animate: &'a i32,
                    sing_nominative: &'a str,
                    sing_genitive: &'a str,
                    sing_dative: &'a str,
                    sing_accusative: &'a str,
                    sing_instrumental: &'a str,
                    sing_prepositional: &'a str,
                    sing_locative: &'a str,
                    plur_nominative: &'a str,
                    plur_genitive: &'a str,
                    plur_dative: &'a str,
                    plur_accusative: &'a str,
                    plur_instrumental: &'a str,
                    plur_prepositional: &'a str,
                    plur_locative: &'a str) {
    use schema::nouns;

    let new_noun = AddNoun {
        gender: gender,
        animate: animate,
        sing_nominative: sing_nominative,
        sing_genitive: sing_genitive,
        sing_dative: sing_dative,
        sing_accusative: sing_accusative,
        sing_instrumental: sing_instrumental,
        sing_prepositional: sing_prepositional,
        sing_locative: sing_locative,

        plur_nominative: plur_nominative,
        plur_genitive: plur_genitive,
        plur_dative: plur_dative,
        plur_accusative: plur_accusative,
        plur_instrumental: plur_instrumental,
        plur_prepositional: plur_prepositional,
        plur_locative: plur_locative
    };

    diesel::insert_into(nouns::table)
            .values(&new_noun)
            .execute(conn)
            .expect("Error adding nouns");
}

pub fn add_nouns<'a>(conn: &SqliteConnection,
                    nouns: &[AddNoun]) {
    use schema::nouns;

    diesel::insert_into(nouns::table)
            .values(nouns)
            .execute(conn)
            .expect("Error adding nouns");
}