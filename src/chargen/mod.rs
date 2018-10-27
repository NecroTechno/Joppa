mod static_files;
use self::static_files::read_json;
use std::collections::HashMap;

const CAVES_DATA_STRING: &str = "static/caves_data";

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

// read the subtype json dependent on the genotype
fn retrieve_subtype(subtype_key_result: Option<char>, subtype_file: &str) -> Option<String> {
    let subtypes: HashMap<String, String> =
        read_json(&format!("{}/{}", CAVES_DATA_STRING, subtype_file));
    match subtype_key_result {
        Some(subtype_key) => {
            if subtypes.contains_key(&subtype_key.to_string()) {
                Some(subtypes[&subtype_key.to_string()].to_string())
            } else {
                return None;
            }
        }
        None => return None,
    }
}

pub fn chargen(text: &str) -> Result<Character, &'static str> {
    let text_lowercase = text.to_lowercase();
   
    // ensure the form input length is at least 8 characters long
    if text_lowercase.len() < 8 {
        return Err("not enough characters")
    }

    // read the genotypes json to a variable and start retrieving some
    // of the input values
    let genotypes: HashMap<String, String> =
        read_json(&format!("{}/genotypes.json", CAVES_DATA_STRING));
    let genotype_key_result = &text_lowercase.chars().nth(0);
    let subtype_key_result = &text_lowercase.chars().nth(1);
    let attribute_keys_result = &text_lowercase.get(2..8).unwrap();
    println!("{}", attribute_keys_result);

    let genotype: String;
    let subtype: String;
    let attributes: Attributes;

    // retrieve genotype and subtype
    // the subtype is dependent on the genotype so these must be run together
    match genotype_key_result {
        Some(genotype_key) => {
            if genotypes.contains_key(&genotype_key.to_string()) {
                genotype = genotypes[&genotype_key.to_string()].to_string();
                if genotype_key.to_string() == "a" || genotype_key.to_string() == "b" {
                    match retrieve_subtype(
                        *subtype_key_result,
                        if genotype_key.to_string() == "a" {
                            "castes.json"
                        } else {
                            "callings.json"
                        },
                    ) {
                        Some(subtype_result) => subtype = subtype_result,
                        None => return Err("No subtype found."),
                    }
                } else {
                    return Err("Character code must begin with a or b.");
                }
            } else {
                return Err("Character code must begin with a or b.");
            }
        }
        None => return Err("Field cannot be empty."),
    }

    // return the character
    let character = Character {
        genotype: genotype,
        subtype: subtype,
    };
    Ok(character)
}
