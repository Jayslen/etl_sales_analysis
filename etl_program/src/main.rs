use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = env::args().nth(1).unwrap();
    process_csv(&args);
}

pub fn process_csv(path: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut process_data: Vec<Vec<String>> = Vec::new();
    let mut cities: HashMap<String, usize> = HashMap::new();
    let mut countries: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("CustomerID") {
            continue;
        }

        let mut cols = line
            .split(',')
            .map(|value| value.to_string())
            .collect::<Vec<String>>();
        // transform + load here

        let city_id = cities.get(&cols[5]);
        let country_id = countries.get(&cols[6]);
        if let Some(id) = country_id {
            cols[6] = id.to_string();
        } else {
            let new_id = countries.len() + 1;
            countries.insert(cols[6].clone(), new_id);
            cols[6] = new_id.to_string();
        }
        if let Some(id) = city_id {
            cols[5] = id.to_string();
        } else {
            let new_id = cities.len() + 1;
            cities.insert(cols[5].clone(), new_id);
            cols[5] = new_id.to_string();
        }
        process_data.push(cols);

        if process_data.len() >= 500 {
            load_data(&mut process_data, &mut countries, &mut cities);
        }
    }
}

fn load_data(
    data: &mut Vec<Vec<String>>,
    countries: &mut HashMap<String, usize>,
    cities: &mut HashMap<String, usize>,
) {
    data.clear();
}
