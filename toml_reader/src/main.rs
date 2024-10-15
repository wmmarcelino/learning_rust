use clap::Parser;
use std::fs::read_to_string;

#[derive(Debug, Parser)]
struct Args{
    #[arg()]
    filename: String,
}


//uses tom 0.4

fn toml_dynamic(filename: &str) -> toml::Value {
    let file_content = read_to_string(filename).unwrap();
    let toml = file_content.parse::<toml::Value>().unwrap();
    toml
}

//Read static toml content
use serde_derive::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
struct Input{
    xml_file: String,
    json_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Postgresql{
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Toml{
    input: Input,
    postgresql: Postgresql,
}

fn toml_static(filename: &str) -> Toml {
    let file_content = read_to_string(filename).unwrap();
    toml::from_str::<Toml>(&file_content).unwrap()
}

fn main() {
    let args = Args::parse();
    let filename = args.filename;

    let toml_dynamic = toml_dynamic(&filename);
    let toml_static = toml_static(&filename);
    
    println!("Toml dynamic: {}", toml_dynamic.get("postgresql").unwrap().get("database").unwrap());

    println!("Toml static: {}", toml_static.postgresql.database);

}
