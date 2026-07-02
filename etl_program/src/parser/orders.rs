use postgres::Client;
use std::fs::File;
use std::io::BufReader;

pub fn process_orders_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::{HashMap, HashSet};
    use std::io::{BufRead, Seek, SeekFrom, Write};

    let mut status_set = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("OrderID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();
        let status = cols[3].to_string();

        status_set.insert(status);
    }

    let mut writer = client.copy_in("COPY status (status_name) FROM STDIN WITH (FORMAT csv)")?;

    for status in &status_set {
        writer.write_all(format!("{}\n", status).as_bytes())?;
    }

    writer.finish()?;

    let mut status_map = HashMap::new();
    for row in client.query("SELECT status_id, status_name FROM status", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        status_map.insert(name, id);
    }

    reader.seek(SeekFrom::Start(0))?;

    let mut writer = client
        .copy_in("COPY orders (customer_id, order_date, status_id) FROM STDIN WITH (FORMAT csv)")?;

    for line in reader.lines() {
        let line = line?;
        if line.contains("OrderID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();

        let customer_id = cols[1];
        let order_date = cols[2];
        let status = cols[3];

        let status_id = status_map.get(status).ok_or("Status not found")?;

        let row = format!("{},{},{}\n", customer_id, order_date, status_id);

        writer.write_all(row.as_bytes())?;
    }

    writer.finish()?;

    Ok(())
}
