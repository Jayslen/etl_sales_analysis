mod config;
mod parser;

use std::{
    fs::File,
    io::{BufRead, Read},
};

use config::{ConfigData, connection};
use parser::customers;
use std::io::BufReader;

fn main() {
    let path = "../data/customers.csv";
    let db = &mut connection().unwrap();
    let csv_config = ConfigData::init();

    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    // let header = reader
    //     .by_ref()
    //     .lines()
    //     .next()
    //     .unwrap()
    //     .unwrap()
    //     .split(',')
    //     .collect();

    // csv_config.which_dataset(&header);
    println!("Header: {:?}", header);
    // if let Err(e) = customers::process_csv(db, &mut reader) {
    //     eprintln!("Error processing CSV: {}", e);
    // }
}
