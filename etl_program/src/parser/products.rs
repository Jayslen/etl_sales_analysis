use std::fs::File;

use postgres::Client;
use std::io::BufReader;

pub fn process_products_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::{HashMap, HashSet};
    use std::io::{BufRead, Seek, SeekFrom, Write};

    let mut categories_set = HashSet::new();

    // First pass → collect unique categories
    for line in reader.lines() {
        let line = line?;
        if line.contains("ProductID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();
        let category = cols[2].to_string();

        categories_set.insert(category);
    }

    // Insert categories
    let mut writer =
        client.copy_in("COPY categories (category_name) FROM STDIN WITH (FORMAT csv)")?;

    for category in &categories_set {
        writer.write_all(format!("{}\n", category).as_bytes())?;
    }

    writer.finish()?;

    // Build category_map
    let mut category_map = HashMap::new();
    for row in client.query("SELECT category_id, category_name FROM categories", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        category_map.insert(name, id);
    }

    // Reset reader
    reader.seek(SeekFrom::Start(0))?;

    // Insert products
    let mut writer = client.copy_in(
        "COPY products (product_name, category_id, price, stock) FROM STDIN WITH (FORMAT csv)",
    )?;

    for line in reader.lines() {
        let line = line?;
        if line.contains("ProductID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();

        let product_name = cols[1];
        let category = cols[2];
        let price = cols[3];
        let stock = cols[4];

        let category_id = category_map[category];

        let row = format!("{},{},{},{}\n", product_name, category_id, price, stock);

        writer.write_all(row.as_bytes())?;
    }

    writer.finish()?;

    Ok(())
}
