use postgres::{Client, NoTls};
use std::collections::HashMap;

pub fn connection() -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::connect(
        "host=localhost user=postgres password=postgres dbname=sales",
        NoTls,
    )?;
    return Ok(client);
}

pub struct ConfigData {
    datasets: HashMap<String, Vec<String>>,
    // customers: [String; 8],
    // orders_details: [String; 4],
    // orders: [String; 4],
    // products: [String; 5],
}

impl ConfigData {
    pub fn init() -> ConfigData {
        let mut data = ConfigData {
            datasets: HashMap::new(),
        };
        data.datasets.insert(
            "customers".to_string(),
            vec![
                "customer_id".to_string(),
                "first_name".to_string(),
                "last_name".to_string(),
                "email".to_string(),
                "phone".to_string(),
                "address".to_string(),
                "city".to_string(),
                "country".to_string(),
            ],
        );
        data.datasets.insert(
            "orders_details".to_string(),
            vec![
                "order_detail_id".to_string(),
                "order_id".to_string(),
                "product_id".to_string(),
                "quantity".to_string(),
            ],
        );
        data.datasets.insert(
            "orders".to_string(),
            vec![
                "order_id".to_string(),
                "customer_id".to_string(),
                "order_date".to_string(),
                "status".to_string(),
            ],
        );
        data.datasets.insert(
            "products".to_string(),
            vec![
                "product_id".to_string(),
                "name".to_string(),
                "description".to_string(),
                "price".to_string(),
                "stock".to_string(),
            ],
        );
        // let data = ConfigData {
        //     customers: [
        //         "customer_id".to_string(),
        //         "first_name".to_string(),
        //         "last_name".to_string(),
        //         "email".to_string(),
        //         "phone".to_string(),
        //         "address".to_string(),
        //         "city".to_string(),
        //         "country".to_string(),
        //     ],
        //     orders_details: [
        //         "order_detail_id".to_string(),
        //         "order_id".to_string(),
        //         "product_id".to_string(),
        //         "quantity".to_string(),
        //     ],
        //     orders: [
        //         "order_id".to_string(),
        //         "customer_id".to_string(),
        //         "order_date".to_string(),
        //         "status".to_string(),
        //     ],
        //     products: [
        //         "product_id".to_string(),
        //         "name".to_string(),
        //         "description".to_string(),
        //         "price".to_string(),
        //         "stock".to_string(),
        //     ],
        // };
        return data;
    }

    pub fn which_dataset(&self, header: &Vec<String>) {
        for (key, value) in self.datasets.iter() {
            if value == header {
                println!("Dataset: {}", key);
                return;
            }
        }
    }
}
