use clap::Parser;
use std::{borrow::BorrowMut, fs::read_to_string};
use serde_json::{Number, Value};

#[derive(Debug, Parser)]
struct Args{
    #[arg(short='i', allow_hyphen_values=true)]
    input_file: String,
    #[arg(short='o', allow_hyphen_values=true)]
    output_file: String,
}


fn json_dynamic (filename: &String) -> serde_json::Value {
    let file_content = read_to_string(filename).unwrap();
    file_content.parse::<Value>().unwrap()
}


// JSON Static

use serde_derive::{Deserialize,Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sale {
    id: String,
    product_id: u32,
    date: u32,
    quantity: f64,
    unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JSON {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

fn json_static(filename: &String) -> JSON {
    let file_content = read_to_string(filename).unwrap();
    serde_json::from_str::<JSON>(&file_content).unwrap()
}


fn main() {
    let args = Args::parse();
    let input_file = args.input_file;
    let output_file = args.output_file;
    

    //Creates JSON struct dynamically
    let mut json_dynamic = json_dynamic(&input_file);

    //Adds 1 to quantity of the sold chairs
    if let Value::Number(n) = json_dynamic["sales"][0]["quantity"].borrow_mut() {
        *n = Number::from_f64(n.as_f64().unwrap() + 1.0).unwrap();

        /* It also works.
        json_dynamic["sales"][0]["quantity"] = Value::Number(Number::from_f64(n.as_f64().unwrap() + 1.0).unwrap());
        */
    }

    // Save JSON object in the output file
    let  output_file_dyn = output_file.clone().replace(".json", "_dyn.json");
    let _ = std::fs::write(
        output_file_dyn,
        serde_json::to_string_pretty(&json_dynamic).unwrap());

    //Creates JSON struct statically
    let mut json_static = json_static(&input_file);

    json_static.sales[0].quantity += 1.0;

    // Save JSON object in the output file
    let  output_file_sta = output_file.clone().replace(".json", "_sta.json");
    let _ = std::fs::write(
        output_file_sta,
        serde_json::to_string_pretty(&json_static).unwrap());

}
