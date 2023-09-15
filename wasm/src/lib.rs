use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
// use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct Mapping {
    data: Vec<(i32, String, String)>,
}

// #[wasm_bindgen]
pub fn pinyin_to_hanzi(pinyin: &str) -> String {
    let path = Path::new("mapping.json");

    let data: Vec<(i32, String, String)>;

    if path.exists() {
        // Deserialize data from file if it exists
        let file = File::open(path).map_err(|e| e.to_string()).unwrap();
        let reader = BufReader::new(file);
        let saved_data: Mapping = serde_json::from_reader(reader).map_err(|e| e.to_string()).unwrap();
        data = saved_data.data;
    } else {
        // Otherwise, load and sort the data
        let file = File::open("./mapping.txt").map_err(|e| e.to_string()).unwrap();
        let reader = BufReader::new(file);
        data = load_and_sort_data(reader).map_err(|e| e.to_string()).unwrap();

        // Serialize and save the data
        let file = File::create(path).map_err(|e| e.to_string()).unwrap();
        let writer = BufWriter::new(file);
        let saved_data = Mapping { data: data.clone() };
        serde_json::to_writer(writer, &saved_data).map_err(|e| e.to_string()).unwrap();
    }

    let result = find_characters(&data, pinyin);

    match result {
        Ok(result) => serde_json::to_string(&result).unwrap_or_else(|_| "Error serializing data".to_string()),
        Err(e) => serde_json::to_string(&e).unwrap_or_else(|_| "Error serializing error".to_string()),
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

fn load_and_sort_data<R: BufRead>(reader: R) -> std::io::Result<Vec<(i32, String, String)>> {
    let mut data: Vec<(i32, String, String)> = Vec::new();

    // Load in lines
    for line in reader.lines() {
        let line = line?;
        let chunks: Vec<&str> = line.split('\t').collect();
        if chunks.len() < 3 {
            continue;
        }
        let frequency = chunks[0].parse::<i32>().unwrap();
        let hanzi = chunks[1].to_string();
        let pinyin = chunks[2].to_string();
        data.push((frequency, hanzi, pinyin));
    }

    // Sort by frequency
    data.sort_by(|a, b| b.0.cmp(&a.0));

    Ok(data)
}

