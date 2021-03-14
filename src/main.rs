use gen_declension_table::models::AddNoun;
use gen_declension_table::models::Noun;
use gen_declension_table::*;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::path::Path;
use std::str;
use std::env;

extern crate diesel;

pub const MASCULINE: char = 'm';
pub const FEMININE: char = 'f';
pub const NEUTER: char = 'n';

pub const NOMINATIVE    : usize = 0;
pub const GENITIVE      : usize = 1;
pub const DATIVE        : usize = 2;
pub const ACUSATIVE     : usize = 3;
pub const INSTRUMENTAL  : usize = 4;
pub const PREPOSITIONAL : usize = 5;
pub const LOCATIVE      : usize = 6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_xml: &String;
    if args.len() < 2 {
        println!("the path to the open corpora xml (dict.opcorpora.xml) needs to be provided as an argument (including file name)");
        return
    } else {
        path_to_xml = args.get(1).unwrap();
    }
    let xml_reader = Reader::from_file(Path::new(path_to_xml));
    let mut dec_table: Vec<Noun> = Vec::new();

    let connection = establish_connection();

    let mut buf = Vec::new();
    let mut current_format: String = String::new();
    let mut sing: bool = true;
    let mut case: usize = 6;
    let mut animate: i32 = 0;
    let mut gender: String = String::new();

    let mut is_noun: bool = false;
    let mut w = Noun::new();

    println!("starting search in the XML...");
    match xml_reader {
        Ok(mut reader) => {
            'readloop: loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        match e.name() {
                            b"f" => {
                                if is_noun {
                                    for attr in e.attributes() {
                                        let attrib = attr.unwrap().unescaped_value().unwrap().into_owned();
                                        current_format = str::from_utf8(&attrib).unwrap().to_string();
                                    }
                                }
                            },
                            _ => (),
                        }
                    },
                    Ok(Event::Empty(ref e)) => {
                        match e.name() {
                            b"g" => {
                                if is_noun { // if we already know it's a noun, check the word number and declensions
                                    for attr in e.attributes() {
                                        let attrib = &attr.unwrap().unescaped_value().unwrap().into_owned();
                                        match str::from_utf8(attrib).unwrap_or_default() {
                                            "sing" => sing = true,
                                            "plur" => sing = false,
                                            "nomn" => case = NOMINATIVE,
                                            "gent" => case = GENITIVE,
                                            "datv" => case = DATIVE,
                                            "accs" => case = ACUSATIVE,
                                            "ablt" => case = INSTRUMENTAL,
                                            "loct" => case = PREPOSITIONAL,
                                            "loc1" => case = PREPOSITIONAL,
                                            "loc2" => case = LOCATIVE,
                                            "anim" => animate = 1,
                                            "inan" => animate = 0,
                                            "femn" => gender = FEMININE.to_string(),
                                            "masc" => gender = MASCULINE.to_string(),
                                            "neut" => gender = NEUTER.to_string(),
                                            _ => (),
                                        }
                                    }
                                } else { // if we don't know if it's a noun yet, take a look to see if it is
                                    for attr in e.attributes() {
                                        let attrib = attr.unwrap().unescaped_value().unwrap().into_owned();
                                        is_noun = str::from_utf8(&attrib).unwrap() == "NOUN";
                                    }
                                }
                            }
                            _ => (),
                        }
                    },
                    Ok(Event::End(ref e)) => {
                        match e.name() {
                            b"lemmata" => break 'readloop,
                            b"f" => {
                                if is_noun {
                                    w.gender = gender.to_owned();
                                    w.animate = animate;
                                    if sing {
                                        match case {
                                            NOMINATIVE => w.sing_nominative = current_format,
                                            GENITIVE => w.sing_genitive = current_format,
                                            DATIVE => w.sing_dative = current_format,
                                            ACUSATIVE => w.sing_accusative = current_format,
                                            INSTRUMENTAL => w.sing_instrumental = current_format,
                                            PREPOSITIONAL => w.sing_prepositional = current_format,
                                            LOCATIVE => w.sing_locative = current_format,
                                            _ => (),
                                        }
                                    } else {
                                        match case {
                                            NOMINATIVE => w.plur_nominative = current_format,
                                            GENITIVE => w.plur_genitive = current_format,
                                            DATIVE => w.plur_dative = current_format,
                                            ACUSATIVE => w.plur_accusative = current_format,
                                            INSTRUMENTAL => w.plur_instrumental = current_format,
                                            PREPOSITIONAL => w.plur_prepositional = current_format,
                                            LOCATIVE => w.plur_locative = current_format,
                                            _ => (),
                                        }
                                    }
                                }
                                //reset values
                                current_format = String::new();
                                sing = true;
                                case = 6;
                            },
                            b"lemma" => {
                                if is_noun {
                                    dec_table.push(w);
                                    w = Noun::new();
                                    is_noun = false;
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (), // There are several other `Event`s we do not consider here
                }
                buf.clear();
            };
            //let json_table = serde_json::to_string(&dec_table).expect("failed to serialize table");
            //println!("{}", json_table);
            
            println!("finished search in the XML...");
            println!("Number of nouns found: {}", dec_table.len());

            //there's a limit of 65535 parameters in the query, so we can't
            //insert all entries at once.
            let mut i:usize = 0;
            let mut range_to: usize;
            while i <= dec_table.len() {
                if i + 2000 <= dec_table.len() {
                    range_to = i + 2000;
                } else {
                    range_to = dec_table.len();
                }
                println!("inserting from {} to {}", i, range_to);
                let dec_table_slice: &[Noun] = &dec_table[i..range_to];

                let mut insert_vals: Vec<AddNoun> = Vec::new();
                
                for w in dec_table_slice {
                    let n = AddNoun {
                        gender: &w.gender,
                        animate: &w.animate,
                        sing_nominative: &w.sing_nominative,
                        sing_genitive: &w.sing_genitive,
                        sing_dative: &w.sing_dative,
                        sing_accusative: &w.sing_accusative,
                        sing_instrumental: &w.sing_instrumental,
                        sing_prepositional: &w.sing_prepositional,
                        sing_locative: &w.sing_locative,
                    
                        plur_nominative: &w.plur_nominative,
                        plur_genitive: &w.plur_genitive,
                        plur_dative: &w.plur_dative,
                        plur_accusative: &w.plur_accusative,
                        plur_instrumental: &w.plur_instrumental,
                        plur_prepositional: &w.plur_prepositional,
                        plur_locative: &w.plur_locative,
                    };
                    insert_vals.push(n);                 
                }
                add_nouns(&connection, &insert_vals); 
                i = range_to + 1;               
            }
        },
        Err(e) => print!("{}", e),
    }
}
