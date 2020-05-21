# RPM
## The most convinient way to install Crates in Rust.
## Notices
If you catch any errors feel free to write a mail `paul.hanneforth.o@gmail.com` or create an issue on GitHub.

## Installation
First clone the repository in a directory of your choice
```sh
git clone https://github.com/paul-hanneforth/rpm.git
```
Then go into the directory and build the crate using cargo.
```sh
cargo build --release
```
You will find the finished executable in the `/target/release` folder

## Usage
To install a new crate simply run
```sh
rpm install [crate_name]
```
This will automatically add the newest version of the crate to your `Cargo.toml` file