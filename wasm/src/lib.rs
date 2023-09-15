use serde::{Deserialize};
mod mapping; // Declare the module
use mapping::get_mapping; // Use the function from the module

use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
struct Mapping {
    data: Vec<(i32, String, String)>,
}

#[wasm_bindgen]
pub fn pinyin_to_hanzi(pinyin: &str) -> String {
    let data: Mapping = serde_json::from_str(get_mapping()).unwrap();
    let result = find_characters(&data.data, pinyin);

    match result {
        Ok(result) => {
            serde_json::to_string(&result).unwrap_or_else(|_| "Error serializing data".to_string())
        }
        Err(e) => {
            serde_json::to_string(&e).unwrap_or_else(|_| "Error serializing error".to_string())
        }
    }
}

fn find_characters(data: &Vec<(i32, String, String)>, text: &str) -> Result<Vec<String>, String> {
    // Find exact matches
    let exact_matches: Vec<_> = data
        .iter()
        .filter(|&&(_, _, ref entry)| entry == text)
        .map(|&(_, ref hanzi, _)| hanzi.clone())
        .collect();

    if !exact_matches.is_empty() {
        Ok(exact_matches)
    } else {
        // If no exact match, find partial matches that start with the text and return the top 10
        let partial_matches: Vec<_> = data
            .iter()
            .filter(|&&(_, _, ref entry)| entry.starts_with(text))
            .map(|&(_, ref hanzi, _)| hanzi.clone())
            .take(10)
            .collect();

        if !partial_matches.is_empty() {
            Ok(partial_matches)
        } else {
            Err("No matching characters found.".to_string())
        }
    }
}


