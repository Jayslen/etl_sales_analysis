use postgres::Client;
use std::fs::File;
use std::io::BufReader;

pub fn process_order_details_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    use std::io::{BufRead, Seek, SeekFrom, Write};

    // 🔹 Build order_map (CSV OrderID → DB order_id)
    let mut order_map = HashMap::new();
    for row in client.query("SELECT order_id FROM orders", &[])? {
        let id: i32 = row.get(0);
        order_map.insert(id.to_string(), id);
    }

    // 🔹 Build product_map (CSV ProductID → DB product_id)
    let mut product_map = HashMap::new();
    for row in client.query("SELECT product_id FROM products", &[])? {
        let id: i32 = row.get(0);
        product_map.insert(id.to_string(), id);
    }

    // Reset reader
    reader.seek(SeekFrom::Start(0))?;

    // 🔹 Insert order_details
    let mut writer = client.copy_in(
        "COPY order_details (order_id, product_id, quantity, total) FROM STDIN WITH (FORMAT csv)",
    )?;

    for line in reader.lines() {
        let line = line?;
        if line.contains("OrderID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();

        let csv_order_id = cols[0];
        let csv_product_id = cols[1];
        let quantity = cols[2];
        let total = cols[3];

        let order_id = order_map.get(csv_order_id).ok_or("Order ID not found")?;

        let product_id = product_map
            .get(csv_product_id)
            .ok_or("Product ID not found")?;

        let row = format!("{},{},{},{}\n", order_id, product_id, quantity, total);

        writer.write_all(row.as_bytes())?;
    }

    writer.finish()?;

    Ok(())
}
