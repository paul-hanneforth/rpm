use crates_io_api::{SyncClient, Error};
use std::fs;
use std::env;
use toml::Value;

fn get_version(name: &str) -> Result<std::string::String, Error> {

    let client = SyncClient::new();
    let versions = match client.get_crate(name) {
        Ok(response) => response.versions,
        Err(e) => return Err(e)
    };
    Ok(versions[0].num.to_owned())
}
fn write(line: &str) {

    let contents = fs::read_to_string("Cargo.toml").expect("Something went wrong!");
    fs::write("Cargo.toml", contents + "\n" + line).expect("Error while writing to file!");

}
fn add_crate(crate_name: &str) -> std::string::String {

    let version = match get_version(crate_name) {
        Ok(version) => version,
        Err(_e) => "Error".to_owned()
    };
    let final_line = format!("{} = \"{}\"", crate_name, version);
    write(&final_line);
    return version;

}
fn print_usage() {

    println!("Usage: ");
    println!("rpm install [crate_name]");
    println!("rpm list");

}
fn list() {

    let contents = fs::read_to_string("Cargo.toml").expect("Something went wrong!");

    let value = contents.parse::<Value>().unwrap();

    let vec = value.as_table();

    /* let result = match vec {
        Some(x) => match x.get("dependencies") {
            Some(x) => match x.as_table() {
                Some(x) => &next
                None => &next
            },
            None => &next
        },
        None => &next
    }; */
    println!("You are currently using these Crates:\n");
    match vec {
        Some(x) => match x.get("dependencies") {
            Some(x) => match x.as_table() {
                Some(x) => {
                    let iter = x.iter();
                    for val in iter {
                        println!("{0} = {1}", val.0, val.1);
                    }
                },
                None => println!("Error!")
            },
            None => println!("Error!")
        },
        None => println!("Error!")
    };

}
fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {

        print_usage();

    } else {

        let action = &args[1];

        if action == "install" {

            let crate_name = &args[2];
            let version = add_crate(crate_name);
            println!("Name: {}", crate_name);
            println!("Version: {}", version);
            println!("Successfully added crate to project!")

        } else if action == "list" {

            list();

        } else {

            print_usage();

        }

    }

}