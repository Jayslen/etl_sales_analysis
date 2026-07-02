use postgres::Client;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

#[derive(Default, Debug)]
pub struct EtlStats {
    pub processed: usize,
    pub inserted: usize,
    pub rejected: usize,
}

pub fn print_summary(stats: &EtlStats, name: &str) {
    println!("Processed : {}", stats.processed);
    println!("Inserted  : {}", stats.inserted);
    println!("Rejected  : {}", stats.rejected);

    let success_rate = if stats.processed > 0 {
        (stats.inserted as f64 / stats.processed as f64) * 100.0
    } else {
        0.0
    };

    println!("Success % : {:.2}%", success_rate);
}

pub fn process_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    // let file = File::open(path)?;
    //  let mut reader = BufReader::new(file);
    let mut stats = EtlStats::default();
    let mut countries_set = HashSet::new();
    let mut cities_set = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("CustomerID") {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();

        let country = cols[6].to_string();
        let city = cols[5].to_string();

        countries_set.insert(country);
        cities_set.insert((city, cols[6].to_string()), ());
    }

    let mut writer =
        client.copy_in("COPY countries (country_name) FROM STDIN WITH (FORMAT csv)")?;

    for country in &countries_set {
        writer.write_all(format!("{}\n", country).as_bytes())?;
    }
    writer.finish()?;

    let mut country_map = HashMap::new();
    for row in client.query("SELECT country_id, country_name FROM countries", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        country_map.insert(name, id);
    }

    let mut writer =
        client.copy_in("COPY cities (city_name, country_id) FROM STDIN WITH (FORMAT csv)")?;

    for ((city, country), _) in &cities_set {
        let country_id = country_map[country];
        writer.write_all(format!("{},{}\n", city, country_id).as_bytes())?;
    }
    writer.finish()?;

    let mut city_map = HashMap::new();
    for row in client.query("SELECT city_id, city_name FROM cities", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        city_map.insert(name, id);
    }

    let mut country_map = HashMap::new();
    for row in client.query("SELECT country_id, country_name FROM countries", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        country_map.insert(name, id);
    }

    reader.seek(SeekFrom::Start((0)))?;
    //    let file = File::open(path)?;
    //    let reader = BufReader::new(file);

    let mut writer = client.copy_in(
    "COPY customers (first_name, last_name, email, phone, city_id, country_id) FROM STDIN WITH (FORMAT csv)"
)?;

    for line in reader.lines() {
        let line = line?;
        if line.contains("CustomerID") {
            continue;
        }
        stats.processed += 1;
        let cols: Vec<&str> = line.split(',').collect();

        let city_id = city_map[cols[5]];
        let country_id = country_map[cols[6]];

        let row = format!(
            "{},{},{},{},{},{}\n",
            cols[1], cols[2], cols[3], cols[4], city_id, country_id
        );

        writer.write_all(row.as_bytes())?;
    }

    writer.finish()?;
    print_summary(&stats, "order_details");
    Ok(())
}
