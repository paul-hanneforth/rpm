use crates_io_api::{SyncClient, Error};
use std::fs;
use std::env;

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

}
fn main() {

    let args: Vec<String> = env::args().collect();
    let action = &args[1];

    if action == "install" {

        let crate_name = &args[2];
        let version = add_crate(crate_name);
        println!("Name: {}", crate_name);
        println!("Version: {}", version);
        println!("Successfully added crate to project!")

    } else {

        print_usage();

    }

}