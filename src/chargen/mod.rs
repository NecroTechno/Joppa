mod static_files;
use self::static_files::read_json;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Attributes {
    strength: u8,
    agility: u8,
    toughness: u8,
    intelligence: u8,
    willpower: u8,
    ego: u8,
}

#[derive(Serialize)]
pub struct Character {
    genotype: String,
    subtype: String,
}

fn retrieve_subtype(subtype_key_result: Option<char>, subtype_file: &str) -> Option<String> {
    let subtypes: HashMap<String, String> = read_json(&format!("static/caves_data/{}", subtype_file));
    match subtype_key_result {
        Some(subtype_key) => {
            if subtypes.contains_key(&subtype_key.to_string()) {
                Some(subtypes[&subtype_key.to_string()].to_string())
            } else {
                return None
            }
        },
        None => return None
    }
}

pub fn chargen(text: &str) -> Option<Character> {
    let text_lowercase = text.to_lowercase();

    let genotypes: HashMap<String, String> = read_json("static/caves_data/genotypes.json"); 
    let genotype_key_result = &text_lowercase.chars().nth(0);
    let subtype_key_result = &text_lowercase.chars().nth(1);
    let attribute_keys_result = &text_lowercase.get(2..8).unwrap();
    println!("{}", attribute_keys_result);

    let genotype: String;
    let subtype: String;
    let attributes: Attributes;

    match genotype_key_result {
        Some(genotype_key) => {
            if genotypes.contains_key(&genotype_key.to_string()) {
                genotype = genotypes[&genotype_key.to_string()].to_string();
                if genotype_key.to_string() == "a" {
                    match retrieve_subtype(*subtype_key_result, "castes.json") {
                        Some(subtype_result) => subtype = subtype_result,
                        None => return None,

                    }
                } else if genotype_key.to_string() == "b" {
                    match retrieve_subtype(*subtype_key_result, "callings.json") {
                        Some(subtype_result) => subtype = subtype_result,
                        None => return None,

                    }
                } else {
                    return None
                }
            } else {
                return None
            }   
        },
        None => return None
    }

    let character = Character {
        genotype: genotype,
        subtype: subtype,
    };
    Some(character)
}
